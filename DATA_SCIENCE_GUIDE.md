# 🔬 Vella: Data Science & MLOps Python Guide

For a Data Science Programmer, Vella acts as the ultimate **"Production Bridge."** It allows you to stay in your preferred Python/Jupyter environment while Vella handles the heavy Rust-based vector mathematics, zero-copy data streaming, and ML model serving natively on the backend.

Here are **10 progressively complex sample programs** showing exactly how a Data Scientist interacts with Vella from a Python environment.

---

## Level 1: Simple Data Retrieval & Ingestion

### 1. The Standard JSON Fetch (Baseline)
Before using advanced features, this is how you fetch standard API data from Vella.
```python
import requests
import pandas as pd

# Fetch raw JSON data from Vella's Headless CMS
response = requests.get("http://127.0.0.1:8080/api/d/Users?limit=100")
data = response.json()

# Load into Pandas (Slow for large datasets due to JSON parsing)
df = pd.DataFrame(data['records'])
print(df.head())
```

### 2. Zero-Copy Apache Arrow to Polars (High Performance)
For datasets over 1 million rows, JSON is too slow. Vella natively streams Apache Arrow bytes, skipping serialization entirely.
```python
import requests
import pyarrow.ipc as ipc
import polars as pl

# Request raw columnar Arrow bytes from Vella
url = "http://127.0.0.1:8080/api/d/Telemetry/export?format=arrow&limit=5000000"
response = requests.get(url, stream=True)

# Stream bytes directly into a Polars DataFrame with ZERO memory copying
with ipc.open_stream(response.content) as reader:
    df = pl.from_arrow(reader.read_all())

print(f"Loaded {df.height} rows instantly!")
```

### 3. Bulk Pushing Cleaned Data to Vella
After cleaning a dataset in Python, push it back into Vella's database via the batch endpoint.
```python
import requests
import polars as pl

# 1. Clean data in Python
df = pl.read_csv("messy_sensor_data.csv").drop_nulls()

# 2. Convert to list of dictionaries
clean_records = df.to_dicts()

# 3. Push to Vella in one massive batch transaction
requests.post(
    "http://127.0.0.1:8080/api/d/SensorData/batch",
    json={"records": clean_records}
)
print("Data successfully ingested into Vella.")
```

---

## Level 2: Intermediate AI & Vectors

### 4. Native Vector Embedding Search
You don't need a separate Pinecone or Milvus database. Vella handles `pgvector` and in-memory SIMD math natively.
```python
import requests
import numpy as np

# Generate a mock embedding vector in Python (e.g., from OpenAI/SentenceTransformers)
my_vector = np.random.rand(1536).tolist()

# Query Vella for the 5 nearest neighbors using Cosine Similarity
response = requests.post("http://127.0.0.1:8080/api/d/KnowledgeDoc/search-vector", json={
    "vector_field": "embedding",
    "query_vector": my_vector,
    "top_k": 5,
    "metric": "Cosine"
})

similar_docs = response.json()
print("Top Matches:", [doc['title'] for doc in similar_docs])
```

### 5. Triggering Vella's RAG Semantic Cache
Vella intercepts LLM queries to save API costs. If you ask a similar question, Vella answers in `<1ms` from its Semantic Cache.
```python
import requests
import time

def ask_vella(question):
    start = time.time()
    res = requests.post("http://127.0.0.1:8080/api/ai/rag/query", json={"query": question}).json()
    print(f"Response in {(time.time() - start)*1000:.2f}ms | Cache Hit: {res.get('cache_hit')}")

# First query (Hits OpenAI, takes ~800ms)
ask_vella("How do I reset the industrial pump?")

# Second query (Vella detects 95% semantic match, answers in 0.5ms from RAM)
ask_vella("What is the pump reset procedure?")
```

### 6. Live Inference with the Feature Store
When deploying a live ML model, you need historical features instantly. Vella's In-Memory feature store provides this.
```python
import requests

def predict_fraud(transaction_amount, user_id):
    # Fetch historical user features from Vella's sub-millisecond RAM store
    feature_req = requests.get(f"http://127.0.0.1:8080/api/features/{user_id}/avg_spend_30d")
    avg_spend = feature_req.json().get('value', 0)
    
    # Python Inference Logic
    if transaction_amount > (avg_spend * 5):
        return "FRAUD_DETECTED"
    return "SAFE"

print(predict_fraud(5000.00, "user_789"))
```

---

## Level 3: Advanced MLOps

### 7. WebAssembly (Wasm) Edge UDF Deployment
Instead of pulling data to Python to clean it, compile your Python/C++ cleaning logic to Wasm and push it to Vella to execute natively on the database.
```python
import requests

# Upload a compiled WebAssembly module to Vella
with open("pii_scrubber.wasm", "rb") as f:
    requests.post("http://127.0.0.1:8080/api/admin/wasm/upload", data=f)

# Instruct Vella to run this Wasm pipeline automatically on all incoming User data
requests.post("http://127.0.0.1:8080/api/admin/wasm/bind", json={
    "model": "User",
    "wasm_module": "pii_scrubber",
    "trigger": "before_insert"
})
print("Wasm UDF Pipeline active at the edge.")
```

### 8. Evaluating ML Model Shadow Routing
You want to test a new model (`v2`) without impacting production users on `v1`.
```python
import requests

# Configure Vella to mirror 100% of live traffic to your new experimental model
requests.post("http://127.0.0.1:8080/api/ai/registry/shadow-route", json={
    "active_model": "mistral-v1",
    "shadow_model": "mistral-v2-experimental"
})

# Simulate a production user request
response = requests.post("http://127.0.0.1:8080/api/ai/inference", json={"prompt": "Summarize my data."})

# The user gets the 'v1' response safely
print("User sees:", response.text)

# But Vella executed 'v2' in the background! You can pull the shadow logs to compare accuracy
shadow_logs = requests.get("http://127.0.0.1:8080/api/ai/registry/shadow-logs").json()
print("Shadow Model Accuracy Variance:", shadow_logs)
```

### 9. Dynamically Offloading Matrix Math to Vella's GPU (CUDA)
Python is single-threaded. You can offload heavy matrix multiplications to Vella, which natively routes it to Nvidia CUDA cores.
```python
import requests
import numpy as np

huge_matrix = np.random.rand(10000, 10000).tolist()

# Send heavy tensor math to Vella
# Vella's HardwareAccelerator detects CUDA and computes it via GPU automatically
response = requests.post("http://127.0.0.1:8080/api/compute/tensor-multiply", json={
    "matrix": huge_matrix
})

print("GPU Math executed successfully by Vella backend.")
```

---

## Level 4: Autonomous Engine Mastery

### 10. Triggering Autonomous AI Postgres Indexing
Data Scientists frequently write terrible, slow SQL joins when exploring data. Vella's `AiTuner` watches for this and fixes the database for you autonomously.
```python
import requests
import time

def run_terrible_unindexed_query():
    # A deep relational graph query that takes 300ms because there are no indexes
    requests.get("http://127.0.0.1:8080/api/d/users?expand=company,purchases,analytics")

# Simulate a Data Scientist running an unoptimized exploratory query 50 times
for _ in range(50):
    run_terrible_unindexed_query()
    
# Wait a moment for Vella's background AI Tuner to process the telemetry
time.sleep(2)

# Pull the Vella AI Tuner Report
report = requests.get("http://127.0.0.1:8080/api/ai/report").json()

print("Vella AI Intervened:")
for rec in report['recommendations']:
    if rec['is_applied']:
        print(f"✅ AUTO-FIXED: Vella detected your slow query and autonomously ran:")
        print(f"   {rec['ddl']}") # e.g., CREATE INDEX idx_ai_auto_users_company ON users (company_id);
```
