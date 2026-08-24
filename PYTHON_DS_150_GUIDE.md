# 🐍 Vella: The Ultimate 150 Data Science & MLOps Cookbook

This guide contains **150 progressively complex Python** patterns. It demonstrates how Data Scientists can leverage Vella's Rust backend for zero-copy data streaming, vector similarity searches, time-series downsampling, and enterprise MLOps model routing without leaving Jupyter Notebooks.

---

### 🟢 Part 1: Core Data Extraction & ETL (001 - 020)
**001. Init Session:** `import requests; session = requests.Session(); session.headers.update({"Authorization": "Bearer KEY"})`
**002. Fetch JSON List:** `res = session.get("http://api.vella.dev/api/d/Users").json()`
**003. Load to Pandas:** `import pandas as pd; df = pd.DataFrame(res['records'])`
**004. Fetch Single Record:** `user = session.get(f"{URL}/api/d/Users/{user_id}").json()`
**005. Server-Side Filtering:** `session.get(f"{URL}?filter=status='Active'&age>25")`
**006. Deep Relational Expand:** `session.get(f"{URL}?expand=company,department")`
**007. Pagination Loop:** `while url: data = session.get(url).json(); all_data.extend(data['records']); url = data['next']`
**008. Async Fetch (aiohttp):** `async with aiohttp.ClientSession() as s: async with s.get(URL) as r: data = await r.json()`
**009. Batch Async Fetch:** `results = await asyncio.gather(*[fetch(url) for url in urls])`
**010. Fetch Apache Arrow IPC:** `res = session.get(f"{URL}/export?format=arrow", stream=True)`
**011. Load to PyArrow:** `import pyarrow.ipc as ipc; table = ipc.open_stream(res.content).read_all()`
**012. Load to Polars (Zero-Copy):** `import polars as pl; df = pl.from_arrow(table)`
**013. Export to Parquet:** `res = session.get(f"{URL}/export?format=parquet"); open("data.parquet", "wb").write(res.content)`
**014. Clean in Polars:** `clean_df = df.drop_nulls().filter(pl.col("age") > 18)`
**015. Push Batch to Vella:** `session.post(f"{URL}/batch", json={"records": clean_df.to_dicts()})`
**016. Update Single Record:** `session.put(f"{URL}/{id}", json={"LTV": 5000})`
**017. Delete Record:** `session.delete(f"{URL}/{id}")`
**018. Parse Vella Timestamps:** `df['created_at'] = pd.to_datetime(df['created_at'])`
**019. Check Server Health:** `health = session.get("http://api.vella.dev/health").json()`
**020. Auto-Retry Logic:** `from requests.adapters import HTTPAdapter; session.mount('http://', HTTPAdapter(max_retries=3))`

---

### 🟡 Part 2: Feature Engineering & Time-Series (021 - 045)
**021. Push to Feature Store:** `session.post(f"{URL}/features/push", json={"user_123": {"avg_spend": 145.0}})`
**022. Pull Feature (<1ms):** `spend = session.get(f"{URL}/features/user_123/avg_spend").json()['value']`
**023. Live Inference Merge:** `features = fetch_features(user_id); prediction = model.predict(features)`
**024. Trigger Vella TimescaleDB:** `session.get(f"{URL}/ts/query?metric=temperature&bucket=1s")`
**025. 100ms Granularity Bucket:** `session.get(f"{URL}/ts/query?metric=vibration&bucket=100ms")`
**026. Load TS to Pandas:** `ts_df = pd.DataFrame(ts_res['buckets']); ts_df.set_index('time', inplace=True)`
**027. Rolling Window (Polars):** `df.rolling(index_column="time", period="1h").agg(pl.col("value").mean())`
**028. Detect Anomalies (IsolationForest):** `clf.fit(ts_df[['value']]); ts_df['anomaly'] = clf.predict(ts_df[['value']])`
**029. Push Anomalies to Vella:** `session.post(f"{URL}/Alerts", json={"type": "Anomaly", "timestamp": "..."})`
**030. Fetch Vella Audit Logs:** `logs = session.get(f"{URL}/api/d/AuditLogs").json()`
**031. Parse JSON fields:** `df = pd.json_normalize(df['metadata_column'])`
**032. One-Hot Encoding:** `pd.get_dummies(df, columns=['status'])`
**033. Scale Features (MinMax):** `scaler.fit_transform(df[['age', 'spend']])`
**034. Tune TS Bucket Size:** `session.post(f"{URL}/ai/tuner", json={"target": "timeseries_bucket", "val": "500ms"})`
**035. Fetch Swinging Door Data:** `session.get(f"{URL}/scada/compressed_history")`
**036. Tune Swinging Door Tol:** `session.post(f"{URL}/scada/compression/tune", json={"tolerance": 0.5})`
**037. Calculate Signal Variance:** `np.var(df['temperature'])`
**038. Fast Fourier Transform (FFT):** `np.fft.fft(df['vibration'].values)`
**039. Push FFT arrays to Vella:** `session.post(f"{URL}/FrequencyData", json={"fft": fft_result.tolist()})`
**040. Merge Tabular & TS:** `pd.merge_asof(tabular_df, ts_df, on='timestamp')`
**041. Calculate Lifetime Value:** `df.groupby('user_id')['amount'].sum()`
**042. Trigger Vella Background Cron:** `session.post(f"{URL}/jobs/trigger/calculate_ltv")`
**043. Fetch Job Status:** `status = session.get(f"{URL}/jobs/calculate_ltv/status").json()`
**044. Generate Correlation Matrix:** `df.corr()`
**045. Save Model (Joblib):** `joblib.dump(clf, "model.pkl")`

---

### 🟠 Part 3: Vectors, RAG & Semantic Cache (046 - 075)
**046. Generate OpenAI Embedding:** `vec = openai.Embedding.create(input="text", model="text-embedding-ada-002")['data'][0]['embedding']`
**047. Push Vector to Vella:** `session.post(f"{URL}/KnowledgeDoc", json={"content": "text", "embedding": vec})`
**048. Semantic Search (Vella):** `res = session.post(f"{URL}/KnowledgeDoc/search-vector", json={"query_vector": vec, "top_k": 5})`
**049. Cosine Similarity (Numpy):** `np.dot(vec1, vec2) / (np.linalg.norm(vec1) * np.linalg.norm(vec2))`
**050. Semantic Search via Text:** `res = session.post(f"{URL}/Knowledge/search-text", json={"query": "How to fix pump?"})`
**051. Retrieve Top-K Texts:** `[doc['content'] for doc in res.json()['results']]`
**052. Vella Semantic Cache Query:** `res = session.post(f"{URL}/ai/rag/query", json={"query": "What is policy?"})`
**053. Check Cache Hit:** `print(f"Answered from cache: {res.json()['cache_hit']}")`
**054. Check Token Usage:** `print(f"Cost: {res.json()['token_usage']['total']} tokens")`
**055. Tune Cache Threshold:** `session.post(f"{URL}/ai/tuner/cache", json={"threshold": 0.95})`
**056. Vella Document Splitter:** `chunks = session.post(f"{URL}/ai/chunk", json={"text": giant_pdf}).json()`
**057. Semantic Chunk Tuning:** `session.post(f"{URL}/ai/tuner/chunk", json={"target_size": 1024})`
**058. Push Scraped Webpage:** `session.post(f"{URL}/KnowledgeDoc", json={"url": "...", "content": html})`
**059. Fetch Vector Array into Numpy:** `vec_array = np.array(doc['embedding'])`
**060. L2 Distance (Euclidean):** `np.linalg.norm(vec1 - vec2)`
**061. Plot Embeddings (t-SNE):** `tsne = TSNE(n_components=2); coords = tsne.fit_transform(all_vectors)`
**062. Scatter Plot (Matplotlib):** `plt.scatter(coords[:, 0], coords[:, 1])`
**063. K-Means Clustering:** `kmeans = KMeans(n_clusters=5).fit(all_vectors)`
**064. Push Clusters to Vella:** `session.patch(f"{URL}/KnowledgeDoc/{id}", json={"cluster_id": label})`
**065. RAG System Prompt Injection:** `session.post(f"{URL}/ai/prompt", json={"system": "You are a helpful assistant."})`
**066. Chat History Formatting:** `messages = [{"role": "user", "content": "hi"}, {"role": "assistant", "content": "hello"}]`
**067. Extract AI Highlighted Keywords:** `keywords = res.json()['extracted_keywords']`
**068. Filter by Confidence Score:** `high_conf_docs = [d for d in docs if d['confidence'] > 0.85]`
**069. Voice-to-Text Push:** `session.post(f"{URL}/ai/audio", files={"file": open("audio.wav", "rb")})`
**070. Generate Image Prompt:** `url = session.post(f"{URL}/ai/image", json={"prompt": "Graph visualization"}).json()['url']`
**071. Fetch AI Token Quota:** `quota = session.get(f"{URL}/ai/quota").json()['remaining']`
**072. Multi-Vector Search (Hybrid):** `session.post(f"{URL}/search-hybrid", json={"query": "pump", "vector": vec})`
**073. Calculate Cross-Encoder Rerank:** `scores = cross_encoder.predict([(query, doc) for doc in docs])`
**074. Re-order by Reranker:** `sorted_docs = [doc for _, doc in sorted(zip(scores, docs), reverse=True)]`
**075. Push Reranked to Cache:** `session.post(f"{URL}/ai/cache/seed", json={"query": query, "best_doc_id": sorted_docs[0]['id']})`

---

### 🔴 Part 4: MLOps, Shadow Routing & Governance (076 - 105)
**076. Upload .gguf Weights:** `session.post(f"{URL}/ai/registry/upload", files={"model": open("mistral.gguf", "rb")})`
**077. Set Active Model:** `session.post(f"{URL}/ai/registry/set-active", json={"version": "v1.0"})`
**078. Enable Shadow Routing:** `session.post(f"{URL}/ai/registry/shadow", json={"active": "v1.0", "shadow": "v2.0-beta"})`
**079. Fetch Shadow Variance Logs:** `variance = session.get(f"{URL}/ai/registry/shadow-logs").json()`
**080. Calculate MSE of Shadow Predictions:** `mean_squared_error(y_true_v1, y_pred_v2)`
**081. Trigger 1-Click Rollback:** `session.post(f"{URL}/api/d/rollback/{bad_migration_log_id}")`
**082. Reject Model Approval:** `session.post(f"{URL}/api/d/approvals/{id}/reject", json={"reason": "Low accuracy"})`
**083. Approve Model Deploy:** `session.post(f"{URL}/api/d/approvals/{id}/approve")`
**084. Upload Wasm UDF Pipeline:** `session.post(f"{URL}/admin/wasm/upload", files={"module": open("cleaner.wasm", "rb")})`
**085. Bind Wasm to Table:** `session.post(f"{URL}/admin/wasm/bind", json={"table": "Events", "trigger": "before_insert"})`
**086. Run Chaos Monkey:** `session.post(f"{URL}/chaos/trigger", json={"fault_probability": 0.05, "max_latency": 500})`
**087. Fetch OTel Trace Logs:** `traces = session.get(f"{URL}/observability/traces").json()`
**088. Analyze Circuit Breaker Trips:** `trips = session.get(f"{URL}/resilience/breakers").json()`
**089. Fetch AI Tuner Report:** `tuner_report = session.get(f"{URL}/ai/report").json()`
**090. Execute AI Auto-Index DDL:** `session.post(f"{URL}/ai/indexes/apply", json={"ddl": tuner_report['recommendations'][0]['ddl']})`
**091. Fetch Slow Queries:** `slow_logs = session.get(f"{URL}/db/slow_queries").json()`
**092. Analyze Schema Registry:** `schemas = session.get(f"{URL}/api/d/schema").json()`
**093. Agentic Schema Scaffold:** `session.post(f"{URL}/ai/generate-model", json={"prompt": "A credit card fraud table"})`
**094. Purge CDN Edge Cache:** `session.post(f"{URL}/cdn/purge", json={"cache_key": "ml_models"})`
**095. Trigger Purge All:** `session.post(f"{URL}/cdn/purge_all")`
**096. Track Deployment Latency:** `import time; start=time.time(); res=session.get(URL); print(time.time()-start)`
**097. Fetch Vella System Heat:** `temp = session.get(f"{URL}/system/temperature").json()['celsius']`
**098. Force CUDA Fallback Test:** `session.post(f"{URL}/system/simulate-overheat")`
**099. Check Hardware Accelerator:** `hw = session.get(f"{URL}/system/accelerator").json()['type']` # e.g. 'CUDA'
**100. Stream Execution Logs:** `for line in session.get(f"{URL}/logs/stream", stream=True).iter_lines(): print(line)`
**101. Verify Multi-Tenancy (RLS):** `res = session.get(f"{URL}/api/d/Data", headers={"X-Tenant": "TenantA"})`
**102. Update RLS Policy:** `session.post(f"{URL}/db/rls", json={"table": "Data", "tenant_column": "tenant_id"})`
**103. Generate Test Data (Faker):** `from faker import Faker; fake = Faker(); data = [{"name": fake.name()} for _ in range(100)]`
**104. Batch Push Test Data:** `session.post(f"{URL}/api/d/Users/batch", json={"records": data})`
**105. Fetch Database Size:** `size = session.get(f"{URL}/db/stats").json()['size_mb']`

---

### 🟣 Part 5: PyTorch, Distributed Compute & F1 Edge (106 - 150)
**106. Offload Matrix Math to Vella (CUDA):** `session.post(f"{URL}/compute/tensor-multiply", json={"matrix": np.random.rand(1000,1000).tolist()})`
**107. Convert Polars to PyTorch Tensor:** `import torch; tensor = torch.tensor(df.to_numpy())`
**108. Train Simple Neural Net:** `model = nn.Sequential(nn.Linear(10, 5), nn.ReLU(), nn.Linear(5, 1))`
**109. Inference Forward Pass:** `output = model(tensor.float())`
**110. Export Model to ONNX:** `torch.onnx.export(model, dummy_input, "model.onnx")`
**111. Push ONNX to Vella Registry:** `session.post(f"{URL}/ai/registry/onnx", files={"model": open("model.onnx", "rb")})`
**112. Trigger MPI CFD Cluster Job:** `session.post(f"{URL}/compute/mpi/cfd", json={"mesh_id": "front_wing_v1"})`
**113. Poll MPI Convergence Status:** `status = session.get(f"{URL}/compute/mpi/status").json()['converged']`
**114. Read Computer Vision Extracted Frames:** `frames = session.get(f"{URL}/ai/vision/frames/video_123").json()`
**115. Fetch "Skip Intro" Timestamp:** `timestamp = session.get(f"{URL}/ai/vision/intro_end/video_123").json()['timestamp']`
**116. Fetch Smart Thumbnail (Bytes):** `img = session.get(f"{URL}/ai/vision/thumbnail/video_123").content`
**117. Display Thumbnail (PIL):** `from PIL import Image; import io; Image.open(io.BytesIO(img)).show()`
**118. Open Raw UDP Socket (Python):** `import socket; sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM); sock.bind(("0.0.0.0", 8000))`
**119. Listen to Vella UDP Telemetry:** `data, addr = sock.recvfrom(1024); print("Received F1 packet:", data)`
**120. Send Raw UDP to Vella Edge:** `sock.sendto(b"TELEMETRY_FRAME", ("127.0.0.1", 8001))`
**121. Read Vella 1000Hz IPC Shared Memory (mmap):** `import mmap; shm = mmap.mmap(-1, 8, "vella_ipc_ring")`
**122. Unpack IPC Struct:** `import struct; frame_data = struct.unpack("Q", shm.read(8))[0]`
**123. Fetch Cassandra Multi-Master Node Status:** `nodes = session.get(f"{URL}/db/cassandra/nodes").json()`
**124. Execute Neo4j Cypher Traversal via Vella:** `session.post(f"{URL}/db/graph/query", json={"cypher": "MATCH (u:User) RETURN u"})`
**125. Extract Graph Edges (NetworkX):** `import networkx as nx; G = nx.Graph(); G.add_edges_from(edges)`
**126. Draw NetworkX Graph:** `nx.draw(G, with_labels=True)`
**127. PageRank Centrality:** `ranks = nx.pagerank(G)`
**128. Fetch SCADA OPC-UA Tags:** `tags = session.get(f"{URL}/scada/tags").json()`
**129. Read PLC Holding Register:** `val = session.get(f"{URL}/scada/modbus/read?register=40001").json()['value']`
**130. DANGER: Write Modbus Coil:** `session.post(f"{URL}/scada/modbus/write", json={"coil": 1, "state": True})`
**131. Fetch ISA-18.2 Alarm State:** `state = session.get(f"{URL}/scada/alarms/PUMP_1").json()['state']`
**132. Acknowledge SCADA Alarm:** `session.post(f"{URL}/scada/alarms/PUMP_1/ack")`
**133. Shelve Alarm for Maintenance:** `session.post(f"{URL}/scada/alarms/PUMP_1/shelve", json={"duration_hours": 2})`
**134. Fetch Triple Modular Redundancy (TMR) Votes:** `votes = session.get(f"{URL}/core/tmr/status").json()`
**135. Analyze TMR Divergence:** `if votes['diverged_node']: print(f"Hardware failure isolated on node {votes['diverged_node']}")`
**136. Monitor RTOS Execution Thread:** `status = session.get(f"{URL}/core/rtos/status").json()`
**137. Fetch HLS Video Manifest (M3U8):** `manifest = session.get(f"{URL}/media/manifest/movie_123.m3u8").text`
**138. Parse HLS Bandwidths:** `bandwidths = [line.split("BANDWIDTH=")[1] for line in manifest.split() if "BANDWIDTH=" in line]`
**139. Validate DRM FairPlay Injection:** `assert "EXT-X-KEY" in manifest`
**140. Export Data to S3/GCS:** `session.post(f"{URL}/storage/export", json={"destination": "s3://bucket/data/"})`
**141. Move Hot S3 Asset to Memory (AiTuner):** `session.post(f"{URL}/storage/promote", json={"asset_id": "file.pdf", "tier": "Memory"})`
**142. Train Random Forest Classifier:** `from sklearn.ensemble import RandomForestClassifier; clf = RandomForestClassifier().fit(X, y)`
**143. Calculate Feature Importances:** `importances = clf.feature_importances_`
**144. Fetch OpenTelemetry Span Data:** `spans = session.get(f"{URL}/observability/spans").json()`
**145. Calculate p99 Latency:** `p99 = np.percentile([s['duration_ms'] for s in spans], 99)`
**146. Send Real-Time WebSockets Event (Websockets lib):** `import websockets; await websockets.connect("ws://localhost:8080/api/realtime/ws")`
**147. Authenticate WebSocket:** `await ws.send(json.dumps({"action": "auth", "token": "KEY"}))`
**148. Listen to Live Model Retraining Stream:** `async for msg in ws: print("Retraining Epoch:", json.loads(msg)['loss'])`
**149. Fetch WebAssembly Transformation Benchmarks:** `metrics = session.get(f"{URL}/admin/wasm/benchmarks").json()`
**150. Terminate Cluster Chaos Sandbox:** `session.post(f"{URL}/chaos/stop")`
