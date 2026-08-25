# ⚡ WASM Edge Pipelines

Vella is built for modern data science. Rather than running a sluggish Python backend API strictly to execute Pandas or Scikit-Learn scripts, Vella executes ML operations directly at the edge using **WebAssembly (WASM)**.

## How it Works
1. Data scientists write their machine learning pipelines in Python or Rust.
2. The code is compiled into a lightweight `.wasm` binary.
3. Vella loads the WASM binary directly into its high-performance runtime.

## Zero-Latency Execution
Because the WASM module runs *inside* Vella's memory space, there is zero network latency. When a user requests data, Vella pulls it from the database, passes it instantly into the WASM pipeline, and serves the computed result back via HTTP or WebSockets.

## Apache Arrow Integration
Vella natively exports data to the Apache Arrow format. This allows for zero-copy deserialization. Data moves from the Postgres database into the WASM compute module without ever being converted into slow JSON strings.

*This makes Vella ideal for heavy financial modeling, real-time fraud detection, and biometric processing at the edge.*
