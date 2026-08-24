# ⚛️ Vella: The Ultimate 150 React Integration Cookbook

This guide contains **150 progressively complex React patterns**, moving from basic Vella SDK fetching to advanced enterprise SCADA, AI, and Real-Time WebSocket state management.

---

### 🟢 Part 1: Core Fetching & Hooks (001 - 015)
**001. Init SDK:** `const vella = new VellaClient("https://api.vella.dev");`
**002. Fetch List:** `useEffect(() => { vella.collection('Users').getList().then(setData) }, []);`
**003. Fetch One:** `const fetchUser = async (id) => setRecord(await vella.collection('Users').getById(id));`
**004. Loading State:** `if (isLoading) return <Spinner />;`
**005. Error State:** `if (error) return <div className="text-red-500">{error.message}</div>;`
**006. Empty State:** `if (!data.length) return <p>No records found.</p>;`
**007. Async/Await Component:** `<button onClick={async () => await fetchUser(1)}>Load</button>`
**008. Custom Hook (useUsers):** `const useUsers = () => { /* fetching logic */ return { data, loading, error }; };`
**009. Refetch Trigger:** `const [tick, setTick] = useState(0); useEffect(() => { ... }, [tick]);`
**010. Cancel Fetch:** `const abortController = new AbortController(); // Pass signal to Vella SDK`
**011. Pagination Params:** `vella.collection('Posts').getList({ page, limit: 10 });`
**012. Filter Params:** `vella.collection('Posts').getList({ filter: "status='Published'" });`
**013. Sort Params:** `vella.collection('Posts').getList({ sort: "-created_at" });`
**014. Field Selection:** `vella.collection('Posts').getList({ fields: "id,title" });`
**015. Deep Expand:** `vella.collection('Posts').getList({ expand: "author,category" });`

---

### 🟡 Part 2: Forms & Mutations (016 - 035)
**016. Create Record:** `vella.collection('Task').create({ title, status });`
**017. Update Record:** `vella.collection('Task').update(id, { status: "Done" });`
**018. Delete Record:** `vella.collection('Task').delete(id);`
**019. Controlled Input:** `<input value={title} onChange={e => setTitle(e.target.value)} />`
**020. Form Submit:** `<form onSubmit={e => { e.preventDefault(); submit(); }}>`
**021. Optimistic Update:** `setTasks(prev => [...prev, newTask]); await vella.create(newTask);`
**022. Debounced Save:** `useDebounce(() => vella.update(id, data), 500, [data]);`
**023. File Upload:** `vella.collection('Files').create({ file: e.target.files[0] });`
**024. Multi-File Upload:** `Array.from(files).forEach(f => vella.upload(f));`
**025. Upload Progress:** `vella.upload(file, { onProgress: (p) => setProgress(p) });`
**026. Checkbox Toggle:** `<input type="checkbox" checked={val} onChange={e => vella.update(id, { val: e.target.checked })} />`
**027. Radio Group:** `<input type="radio" value="A" onChange={() => setType("A")} />`
**028. Select Dropdown:** `<select onChange={e => setRole(e.target.value)}><option>Admin</option></select>`
**029. Multi-Select Array:** `vella.update(id, { tags: [...tags, newTag] });`
**030. Form Reset:** `formRef.current.reset(); setTitle('');`
**031. Validation (Empty):** `if (!title.trim()) setError("Title required");`
**032. Validation (Regex):** `if (!/^\S+@\S+\.\S+$/.test(email)) setError("Invalid email");`
**033. Zod Integration:** `const schema = z.object({ title: z.string() }); schema.parse(data);`
**034. Disabled Submit:** `<button disabled={isSubmitting || !isValid}>Save</button>`
**035. Success Toast:** `vella.create(data).then(() => toast.success("Saved!"));`

---

### 🟠 Part 3: Realtime WebSockets (036 - 055)
**036. Global Realtime Hook:** `useRealtimeSubscription('*', handleAllEvents);`
**037. Model Subscription:** `useRealtimeSubscription('Messages', handleMessage);`
**038. Record Subscription:** `useRealtimeSubscription('Task', taskId, handleUpdates);`
**039. Create Event:** `if (e.action === 'CREATE') setList(prev => [e.record, ...prev]);`
**040. Update Event:** `if (e.action === 'UPDATE') setList(prev => prev.map(r => r.id === e.record.id ? e.record : r));`
**041. Delete Event:** `if (e.action === 'DELETE') setList(prev => prev.filter(r => r.id !== e.record.id));`
**042. Connection Status:** `const status = useVellaConnectionStatus(); // 'connected' | 'reconnecting'`
**043. Manual Reconnect:** `<button onClick={() => vella.realtime.reconnect()}>Reconnect</button>`
**044. Unsubscribe:** `useEffect(() => { const unsub = vella.subscribe(...); return () => unsub(); }, []);`
**045. Presence (Join):** `vella.realtime.sendPresence('Room1', { user: 'Alice' });`
**046. Presence (Leave):** `useEffect(() => return () => vella.realtime.leavePresence('Room1'), []);`
**047. Typing Indicator (Send):** `<input onKeyDown={() => vella.realtime.broadcast('typing', true)} />`
**048. Typing Indicator (Recv):** `useRealtimeSubscription('typing', (e) => setTyping(e.user));`
**049. Live Cursor (Send):** `onMouseMove={e => vella.realtime.broadcast('cursor', {x: e.clientX, y: e.clientY})}`
**050. Live Cursor (Recv):** `<div style={{ left: cursor.x, top: cursor.y }} className="absolute cursor" />`
**051. Notification Bell:** `useRealtimeSubscription('Alert', () => setBell(prev => prev + 1));`
**052. Sync Conflict Warning:** `if (e.record.updated_at > local.updated_at) showWarning("Newer version exists");`
**053. Chat Auto-Scroll:** `useEffect(() => bottomRef.current.scrollIntoView(), [messages]);`
**054. Read Receipts:** `vella.update(msgId, { read: true });`
**055. Live Poll Votes:** `useRealtimeSubscription('Vote', (e) => updateChart(e.record));`

---

### 🔴 Part 4: Auth & Security (056 - 075)
**056. Login:** `await vella.auth.loginWithPassword(email, password);`
**057. Logout:** `vella.auth.logout(); setUser(null);`
**058. Get Current User:** `const user = vella.auth.getUser();`
**059. Check Session:** `if (!vella.auth.isValid) navigate('/login');`
**060. Auth Context:** `<AuthContext.Provider value={{ user, login, logout }}>{children}</AuthContext.Provider>`
**061. Google OAuth:** `<button onClick={() => vella.auth.loginWithProvider('google')}>Google</button>`
**062. GitHub OAuth:** `<button onClick={() => vella.auth.loginWithProvider('github')}>GitHub</button>`
**063. OAuth Callback:** `useEffect(() => { vella.auth.handleOAuthCallback().then(...) }, []);`
**064. Magic Link Request:** `await vella.auth.requestMagicLink(email);`
**065. Magic Link Verify:** `await vella.auth.verifyMagicLink(token);`
**066. Password Reset Req:** `await vella.auth.requestPasswordReset(email);`
**067. Password Reset Confirm:** `await vella.auth.confirmPasswordReset(token, newPassword);`
**068. Change Email:** `await vella.auth.requestEmailChange(newEmail);`
**069. Refresh Token:** `await vella.auth.refreshSession();`
**070. Role Check:** `if (user.role !== 'Admin') return <AccessDenied />;`
**071. Field-Level RBAC:** `{user.permissions.canEdit && <EditButton />}`
**072. Hide UI by Role:** `className={user.role === 'Guest' ? 'hidden' : 'block'}`
**073. Require Auth Route:** `<Route path="/admin" element={user ? <Admin /> : <Navigate to="/" />} />`
**074. Remember Me:** `<input type="checkbox" onChange={e => setRemember(e.target.checked)} />`
**075. MFA Prompt:** `if (authRes.requiresMfa) showMfaModal();`

---

### 🟣 Part 5: AI & RAG Interfaces (076 - 095)
**076. Semantic Search:** `vella.ai.searchVector('Knowledge', queryText).then(setResults);`
**077. Search Bar UI:** `<input type="search" placeholder="Ask AI natively..." />`
**078. Streaming Text:** `vella.ai.streamRag(query, (chunk) => setText(prev => prev + chunk));`
**079. AI Cache Hit Badge:** `{res.cache_hit && <Badge>Answered in 1ms (Cached)</Badge>}`
**080. Prompt Tokens:** `<span>Tokens used: {res.token_usage.total}</span>`
**081. Highlight RAG Text:** `<span className="bg-yellow-200">{aiHighlightedKeyword}</span>`
**082. Voice to Text:** `recognition.onresult = (e) => vella.ai.searchVector('Docs', e.results[0][0].transcript);`
**083. Confidence Score Bar:** `<div className="w-full bg-green-500" style={{ width: `${res.confidence * 100}%` }} />`
**084. AI Suggestion Chip:** `<button onClick={() => setInput(suggestion)}>{suggestion}</button>`
**085. Generate Image Hook:** `vella.ai.generateImage(prompt).then(setUrl);`
**086. Auto-Complete (Input):** `vella.ai.complete(input).then(setGhostText);`
**087. AI Scaffolder UI:** `<textarea placeholder="Describe database schema..."></textarea>`
**088. Execute Scaffold:** `vella.admin.generateModel(description).then(showSuccess);`
**089. LLM Context Window:** `<div className="overflow-y-auto max-h-96">{ragContextChunks}</div>`
**090. Semantic Chunk Adjuster:** `vella.ai.tuneChunkSize({ size: 1024 });`
**091. Shadow Model UI:** `<select><option>v1-active</option><option>v2-shadow</option></select>`
**092. Shadow Accuracy Diff:** `<div>Variance: {shadowStats.variance}%</div>`
**093. GPU Routing UI:** `<span className={gpuActive ? 'text-green' : 'text-gray'}>CUDA Active</span>`
**094. Vector Upload (Python sync):** `vella.uploadVector(numpyArray.toJSON());`
**095. AI Rate Limit Warn:** `if (res.tokens_remaining < 100) showWarning("Rate limit approaching");`

---

### 🔵 Part 6: Complex Tables & Layouts (096 - 120)
**096. Table Header Sort:** `<th onClick={() => setSort('name')}>Name {sort === 'name' ? '▲' : '▼'}</th>`
**097. Data Grid (Map):** `<tbody>{rows.map(r => <tr key={r.id}><td>{r.name}</td></tr>)}</tbody>`
**098. Server Pagination:** `vella.collection('Users').getList({ page: currentPage });`
**099. Next Page Btn:** `<button onClick={() => setPage(p => p + 1)}>Next</button>`
**100. Relational Cell:** `<td>{record.expand?.company?.name || 'N/A'}</td>`
**101. Deep Relational Cell:** `<td>{record.expand?.author?.expand?.department?.name}</td>`
**102. Bulk Select All:** `<input type="checkbox" onChange={e => setSelection(e.target.checked ? allIds : [])} />`
**103. Bulk Delete:** `Promise.all(selection.map(id => vella.delete(id))).then(refresh);`
**104. Export CSV:** `const csv = toCsv(data); download(csv);`
**105. Export Arrow:** `window.open("https://api.vella.dev/api/d/Data/export?format=arrow");`
**106. Masonry Grid:** `<div className="columns-1 sm:columns-2 md:columns-3">{cards}</div>`
**107. Kanban Column:** `<div className="w-1/3 bg-gray-100">{tasks.filter(t => t.status === col).map(...)}</div>`
**108. Drag Start:** `<div draggable onDragStart={() => setDragged(task.id)}>`
**109. Drop Zone:** `<div onDragOver={e => e.preventDefault()} onDrop={() => updateTaskStatus(dragged, col)}>`
**110. Infinite Scroll Hook:** `if (inView) setPage(p => p + 1);`
**111. Virtualized List:** `<FixedSizeList height={400} itemCount={1000} itemSize={35}>{Row}</FixedSizeList>`
**112. Search Debounce:** `useEffect(() => { const timer = setTimeout(() => search(term), 300); }, [term]);`
**113. Sticky Header:** `<thead className="sticky top-0 bg-white">`
**114. Row Hover:** `<tr className="hover:bg-blue-50 transition-colors">`
**115. Status Badge:** `<span className={`badge badge-${record.status}`}>{record.status}</span>`
**116. Avatar Group:** `<div className="flex -space-x-2">{users.map(u => <Avatar src={u.img} />)}</div>`
**117. Accordion Row:** `<tr onClick={() => setExpanded(r.id)}>{expanded === r.id && <ExpandedData />}</tr>`
**118. Context Menu:** `onContextMenu={e => { e.preventDefault(); showMenu(e.clientX, e.clientY); }}`
**119. Tooltip Cell:** `<td title={record.long_description}>{record.short_description}</td>`
**120. Date Formatting:** `<td>{new Date(record.created_at).toLocaleDateString()}</td>`

---

### ⚫ Part 7: Enterprise, SCADA & F1 Components (121 - 150)
**121. Approval Workflow (Manager):** `<button onClick={() => vella.cms.approve(id)}>Approve</button>`
**122. Reject Workflow:** `<button onClick={() => vella.cms.reject(id, reason)}>Reject</button>`
**123. Audit Log Viewer:** `vella.collection('AuditLogs').getList().then(setLogs);`
**124. Time-Travel Rollback:** `<button onClick={() => vella.audit.rollback(log.id)}>⏪ Revert</button>`
**125. SCADA Tank Fill (SVG):** `<rect y={100 - level} height={level} fill="blue" />`
**126. SCADA ISA-18 Alarm (Unack):** `<div className="animate-flash bg-red-600 text-white">UNACK_ACTIVE</div>`
**127. SCADA Alarm Acknowledge:** `<button onClick={() => vella.scada.ackAlarm('PUMP_1')}>Acknowledge</button>`
**128. SCADA Alarm Cleared:** `<div className="bg-yellow-500">UNACK_CLEARED</div>`
**129. Swinging Door Compression Toggle:** `<button onClick={() => vella.scada.tuneCompression(0.5)}>Set Tolerance</button>`
**130. TMR Voter Status:** `<div>Voting Consensus: {tmrStatus === 'A_B_C' ? 'Healthy' : 'Diverged'}</div>`
**131. Modbus Manual Override:** `<button onClick={() => vella.scada.writeCoil(40001, true)}>OPEN VALVE</button>`
**132. F1 UDP Stream Status:** `<div className={udpActive ? 'text-green-500' : 'text-red-500'}>Radio Link</div>`
**133. Time-Series Chart (Recharts):** `<LineChart data={tsData}><Line type="monotone" dataKey="tire_temp" /></LineChart>`
**134. Dynamic Time-Bucket Selector:** `<select onChange={e => setBucket(e.target.value)}><option>100ms</option><option>1s</option></select>`
**135. MPI Cluster Status:** `<div>Nodes Converged: {mpi.converged} / {mpi.total}</div>`
**136. RTOS Priority Indicator:** `<Badge color="purple">Hard Real-Time Loop Active</Badge>`
**137. 1000Hz IPC Shared Memory Poller:** `requestAnimationFrame(() => updateUI(vella.ipc.readFrame()));`
**138. Wasm UDF Uploader:** `<input type="file" accept=".wasm" onChange={e => vella.admin.uploadWasm(e.target.files[0])} />`
**139. Chaos Monkey Config:** `<input type="range" min="0" max="100" onChange={e => setChaosProb(e.target.value)} />`
**140. Chaos Monkey Inject:** `<button onClick={() => vella.chaos.triggerPartition()}>Simulate Outage</button>`
**141. Cassandra Multi-Region Map:** `<Map activeRegions={['us-east', 'eu-west']} />`
**142. Graph DB Node Graph (D3):** `<ForceGraph3D graphData={cypherResults} />`
**143. HLS Video Player (ReactPlayer):** `<ReactPlayer url="https://cdn.vella.dev/movie.m3u8" playing controls />`
**144. DRM License Key Injector:** `<ReactPlayer config={{ file: { hlsOptions: { drm: widevineConfig } } }} />`
**145. CDN Global Purge Btn:** `<button onClick={() => vella.cdn.purge('movie_meta')}>Purge Edge Cache</button>`
**146. Computer Vision Intro Skipper:** `{timestamp > introEnd && <button>Skip Intro</button>}`
**147. Smart Thumbnail Image:** `<img src={vella.vision.getSmartThumbnail(videoId)} />`
**148. System CPU Heat Gauge:** `<div style={{ backgroundColor: temp > 85 ? 'red' : 'green' }}>{temp}°C</div>`
**149. AI Tuner Auto-Fix Log:** `<ul>{tunerLogs.map(l => <li>{l.action_taken}</li>)}</ul>`
**150. Global Error Boundary:** `<ErrorBoundary fallback={<ErrorUI />}>{children}</ErrorBoundary>`
