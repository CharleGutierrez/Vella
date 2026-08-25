# 📝 Full-Stack Todo App Tutorial (Production Ready)

This tutorial walks you through building a complete, production-ready Todo Application using Vella as the backend, and demonstrates how to consume it using **React**, **Vue**, or **Angular**.

---

## Part 1: The Vella Backend (Rust)

First, we define our database schema in Vella. Vella will automatically create the Postgres tables, generate the REST APIs, and spin up a Real-Time WebSocket server.

Create a file `src/main.rs`:

```rust
use vella::app::VellaApp;
use vella::model::{Schema, Field, FieldType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = VellaApp::new();
    
    // Define the Todo Table
    let todo_schema = Schema::new("Todos")
        .field(Field {
            name: "title".to_string(),
            field_type: FieldType::String,
            required: true,
            ..Default::default()
        })
        .field(Field {
            name: "is_completed".to_string(),
            field_type: FieldType::Boolean,
            default_value: Some("false".to_string()),
            ..Default::default()
        })
        .with_timestamps();

    app.register_schema(todo_schema);
    
    // Boot the server on port 8080
    app.run().await?;
    
    Ok(())
}
```

Run `cargo run` to boot the backend. Vella will generate your endpoints at `http://localhost:8080/api/collections/todos/records`.

---

## Part 2: The React Frontend ⚛️

Here is a complete, real-time React application using standard Hooks.

```tsx
import React, { useState, useEffect } from 'react';

export default function TodoApp() {
    const [todos, setTodos] = useState([]);
    const [newTodo, setNewTodo] = useState("");

    // 1. Initial Fetch
    useEffect(() => {
        fetch('http://localhost:8080/api/collections/todos/records')
            .then(res => res.json())
            .then(data => setTodos(data.items));
    }, []);

    // 2. Real-Time WebSockets
    useEffect(() => {
        const ws = new WebSocket('ws://localhost:8080/api/realtime');
        ws.onmessage = (event) => {
            const { action, collection, record } = JSON.parse(event.data);
            if (collection === 'Todos') {
                if (action === 'INSERT') setTodos(prev => [...prev, record]);
                if (action === 'UPDATE') setTodos(prev => prev.map(t => t.id === record.id ? record : t));
                if (action === 'DELETE') setTodos(prev => prev.filter(t => t.id !== record.id));
            }
        };
        return () => ws.close();
    }, []);

    // 3. Create Todo
    const addTodo = async (e) => {
        e.preventDefault();
        await fetch('http://localhost:8080/api/collections/todos/records', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ title: newTodo, is_completed: false })
        });
        setNewTodo("");
    };

    // 4. Toggle Completion
    const toggleTodo = async (todo) => {
        await fetch(`http://localhost:8080/api/collections/todos/records/${todo.id}`, {
            method: 'PATCH',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ is_completed: !todo.is_completed })
        });
    };

    return (
        <div>
            <h1>Vella React Todos</h1>
            <form onSubmit={addTodo}>
                <input value={newTodo} onChange={e => setNewTodo(e.target.value)} placeholder="New Task..." />
                <button type="submit">Add</button>
            </form>
            <ul>
                {todos.map(todo => (
                    <li key={todo.id} style={{ textDecoration: todo.is_completed ? 'line-through' : 'none' }}>
                        <span onClick={() => toggleTodo(todo)}>{todo.title}</span>
                    </li>
                ))}
            </ul>
        </div>
    );
}
```

---

## Part 3: The Vue 3 Frontend 🟩

Here is the exact same logic written in Vue 3's Composition API (`<script setup>`).

```vue
<template>
  <div>
    <h1>Vella Vue Todos</h1>
    <form @submit.prevent="addTodo">
      <input v-model="newTodo" placeholder="New Task..." />
      <button type="submit">Add</button>
    </form>
    <ul>
      <li v-for="todo in todos" :key="todo.id" 
          :style="{ textDecoration: todo.is_completed ? 'line-through' : 'none' }"
          @click="toggleTodo(todo)">
        {{ todo.title }}
      </li>
    </ul>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'

const todos = ref([])
const newTodo = ref("")
let ws;

onMounted(async () => {
  // 1. Initial Fetch
  const res = await fetch('http://localhost:8080/api/collections/todos/records')
  const data = await res.json()
  todos.value = data.items

  // 2. Real-Time WebSockets
  ws = new WebSocket('ws://localhost:8080/api/realtime')
  ws.onmessage = (event) => {
    const payload = JSON.parse(event.data)
    if (payload.collection === 'Todos') {
      if (payload.action === 'INSERT') todos.value.push(payload.record)
      if (payload.action === 'UPDATE') {
        const index = todos.value.findIndex(t => t.id === payload.record.id)
        if (index !== -1) todos.value[index] = payload.record
      }
      if (payload.action === 'DELETE') {
        todos.value = todos.value.filter(t => t.id !== payload.record.id)
      }
    }
  }
})

onUnmounted(() => {
  if (ws) ws.close()
})

const addTodo = async () => {
  await fetch('http://localhost:8080/api/collections/todos/records', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ title: newTodo.value, is_completed: false })
  })
  newTodo.value = ""
}

const toggleTodo = async (todo) => {
  await fetch(`http://localhost:8080/api/collections/todos/records/${todo.id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ is_completed: !todo.is_completed })
  })
}
</script>
```

---

## Part 4: Production Deployment

Never run `cargo run` in production. Instead, build a release binary and place it behind a reverse proxy like NGINX.

### 1. Compile for Production
```bash
# This strips debug symbols and heavily optimizes the binary
cargo build --release
```
Your compiled binary will be located at `target/release/vella`.

### 2. Setup Systemd (Linux Servers)
Create a service file to keep Vella running in the background and restart it if it crashes.
`sudo nano /etc/systemd/system/vella.service`

```ini
[Unit]
Description=Vella Todo App Backend
After=network.target

[Service]
ExecStart=/path/to/your/target/release/vella
Restart=always
User=www-data
Environment=PORT=8080
Environment=DATABASE_URL=postgres://user:pass@localhost:5432/vella

[Install]
WantedBy=multi-user.target
```
Run `sudo systemctl enable vella && sudo systemctl start vella`.

### 3. NGINX Reverse Proxy (with WebSockets)
To expose Vella securely over port 80/443, configure NGINX to pass traffic to port 8080. **You must explicitly upgrade the connection to support Vella's Real-Time WebSockets.**

`sudo nano /etc/nginx/sites-available/vella`

```nginx
server {
    listen 80;
    server_name api.yourdomain.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        
        # Required for Vella WebSockets
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        
        # Standard Headers
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```
Run `sudo nginx -s reload`. Your Vella backend is now live in production!
