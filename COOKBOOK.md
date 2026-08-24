# 📖 Vella Integration Cookbook

This guide provides concrete, copy-pasteable examples to help developers seamlessly integrate Vella into their existing tech stacks. These recipes cover the frontend UI, custom backend business logic, AI pipelines, data science extraction, and IoT ingestion.

---

## 1. Frontend Integration: React 18 Real-Time UI

**The Goal:** Connect a React frontend to Vella to display a live, updating list of records (like a Kanban board or chat app) without writing manual WebSocket loops.

```tsx
// frontend/src/components/LiveTaskBoard.tsx
import { useEffect, useState } from 'react';
import { VellaClient, useRealtimeSubscription } from '../types/vella-sdk';

// Initialize the auto-generated SDK client
const vella = new VellaClient("https://api.vella.dev", "your-api-key");

export function LiveTaskBoard() {
  const [tasks, setTasks] = useState([]);

  // 1. Initial Data Fetch (Fully Type-Safe)
  useEffect(() => {
    vella.collection('Task').getList({ filter: "status='In_Progress'" })
      .then(setTasks);
  }, []);

  // 2. Realtime WebSocket Hook: Auto-updates the UI when the DB changes
  useRealtimeSubscription('Task', (event) => {
    if (event.action === 'CREATE') {
      setTasks(prev => [event.record, ...prev]);
    } else if (event.action === 'UPDATE') {
      setTasks(prev => prev.map(t => t.id === event.record.id ? event.record : t));
    }
  });

  return (
    <div>
      {tasks.map(task => <div key={task.id}>{task.title} - {task.status}</div>)}
    </div>
  );
}
```

---

## 2. Backend Integration: Custom Pre-Save Hooks (Rust)

**The Goal:** Headless CMS auto-routes are great, but sometimes you need custom business logic (e.g., hashing a password or verifying a Stripe subscription) *before* a record is saved to the database.

```rust
// src/main.rs
use vella::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user_schema = ModelSchema::new("User")
        .field(Field::string("email").unique())
        .field(Field::string("password_hash"))
        
        // INTERCEPT THE LIFECYCLE: Run custom Rust code before saving
        .on_before_insert(|mut record| async move {
            let plain_text = record.get("password_hash").unwrap().as_str();
            let hashed = vella::auth::crypto::hash_password(plain_text);
            record.set("password_hash", hashed);
            
            // Return Ok to proceed with the DB insert, or Err to block it
            Ok(record)
        });

    VellaApp::new().register(user_schema).run().await?;
    Ok(())
}
```

---

## 3. Backend Integration: Adding Custom API Routes (Axum)

**The Goal:** You aren't locked into Vella's Headless CMS. You can attach entirely custom HTTP routes (like a Stripe Webhook) alongside the auto-generated APIs.

```rust
// src/main.rs
use vella::prelude::*;
use axum::{routing::post, Json};

// A completely custom Axum handler
async fn stripe_webhook_handler(Json(payload): Json<serde_json::Value>) -> &'static str {
    println!("Received Stripe Webhook: {}", payload["type"]);
    // Trigger custom Vella database queries here...
    "Webhook OK"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    VellaApp::new()
        .database("postgres://...")
        // Mount your custom routes alongside the Vella engine
        .add_custom_route("/api/billing/webhook", post(stripe_webhook_handler))
        .run()
        .await?;
    Ok(())
}
```

---

## 4. AI Integration: Querying the Semantic RAG Cache (JavaScript)

**The Goal:** Query the backend Vector database to find related documents using natural language, directly from a standard web client.

```javascript
// frontend/src/api/ai.js
async function askQuestion(userQuestion) {
  const response = await fetch("https://api.vella.dev/api/d/KnowledgeDoc/search-vector", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      query_text: userQuestion, // Vella auto-embeds this into a vector
      vector_field: "embedding",
      top_k: 5,
      metric: "Cosine"
    })
  });

  const data = await response.json();
  
  // The Vella Semantic Cache intercepts this. 
  // If `data.cache_hit` is true, this executed in < 1ms!
  console.log(`Cache Hit: ${data.cache_hit} | Response:`, data.results);
  return data.results;
}
```

---

## 5. Data Science Integration: Zero-Copy Arrow Streams (Python)

**The Goal:** A Data Scientist needs to extract 5 million telemetry events from Vella to train an ML model without crashing their Jupyter Notebook.

```python
# jupyter_notebook.ipynb
import requests
import pyarrow.ipc as ipc
import polars as pl

# 1. Hit the Vella export endpoint requesting the Apache Arrow format
url = "https://api.vella.dev/api/d/Telemetry/export?format=arrow&limit=5000000"
response = requests.get(url, stream=True)

# 2. Stream the raw bytes directly into the IPC reader
with ipc.open_stream(response.content) as reader:
    # 3. Instantiate a Polars DataFrame with ZERO memory copying
    df = pl.from_arrow(reader.read_all())

print(f"Loaded {df.height} rows in milliseconds!")
print(df.describe())
```

---

## 6. IoT / Industrial Integration: High-Frequency Sensor Ingestion (cURL)

**The Goal:** Push massive amounts of sensor data from an edge device (like a Raspberry Pi) directly into Vella's Time-Series engine.

```bash
# Edge Device Bash Script
# Send a batch of 100 temperature readings to the Vella Time-Series endpoint
curl -X POST https://api.vella.dev/api/d/SensorData/batch \
  -H "Authorization: Bearer edge_device_token" \
  -H "Content-Type: application/json" \
  -d '{
    "records": [
      {"sensor": "pump_101", "temp": 85.4, "timestamp": "2024-10-12T10:00:00Z"},
      {"sensor": "pump_101", "temp": 85.6, "timestamp": "2024-10-12T10:00:01Z"}
    ]
  }'
```

---

## 💡 Summary of Integration Philosophies:
1. **Frontend:** Don't write fetch requests manually. Use the auto-generated TS SDK.
2. **Backend:** Don't fork the framework. Use `.on_before_insert()` hooks and `.add_custom_route()`.
3. **Data Science:** Don't parse JSON. Request the `?format=arrow` endpoint for instant data frames.
4. **AI:** Don't build separate Python microservices. Send queries directly to Vella's `/search-vector` endpoints and let the AI Middleware handle the caching and token limits.
