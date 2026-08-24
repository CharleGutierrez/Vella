# 🟩 Vella: The Ultimate 150 Vue 3 Integration Cookbook

This guide contains **150 progressively complex Vue 3 (Composition API)** patterns, moving from basic Vella SDK fetching to advanced enterprise SCADA, AI, and Real-Time WebSocket state management.

---

### 🟢 Part 1: Core Fetching & Composition API (001 - 015)
**001. Init SDK:** `const vella = new VellaClient("https://api.vella.dev");`
**002. Fetch List:** `const users = ref([]); onMounted(async () => { users.value = await vella.collection('Users').getList(); });`
**003. Fetch One:** `const fetchUser = async (id) => record.value = await vella.collection('Users').getById(id);`
**004. Loading State:** `<template><Spinner v-if="isLoading" /></template>`
**005. Error State:** `<div v-if="error" class="text-red-500">{{ error.message }}</div>`
**006. Empty State:** `<p v-if="!data.length && !isLoading">No records found.</p>`
**007. Async/Await Component:** `<button @click="fetchUser(1)">Load</button>`
**008. Custom Composable (useUsers):** `export function useUsers() { /* logic */ return { data, isLoading, error }; }`
**009. Refetch Trigger:** `watch(tick, async () => { await fetchData(); });`
**010. Cancel Fetch:** `const abortController = new AbortController(); // Pass signal to Vella SDK`
**011. Pagination Params:** `vella.collection('Posts').getList({ page: currentPage.value, limit: 10 });`
**012. Filter Params:** `vella.collection('Posts').getList({ filter: "status='Published'" });`
**013. Sort Params:** `vella.collection('Posts').getList({ sort: "-created_at" });`
**014. Field Selection:** `vella.collection('Posts').getList({ fields: "id,title" });`
**015. Deep Expand:** `vella.collection('Posts').getList({ expand: "author,category" });`

---

### 🟡 Part 2: Forms & Mutations (016 - 035)
**016. Create Record:** `await vella.collection('Task').create({ title: title.value, status: status.value });`
**017. Update Record:** `await vella.collection('Task').update(id, { status: "Done" });`
**018. Delete Record:** `await vella.collection('Task').delete(id);`
**019. v-model Input:** `<input v-model="title" placeholder="Task title" />`
**020. Form Submit:** `<form @submit.prevent="submitForm">`
**021. Optimistic Update:** `tasks.value.unshift(newTask); await vella.create(newTask).catch(revert);`
**022. Debounced Save:** `watch(formData, debounce(async (val) => await vella.update(id, val), 500), { deep: true });`
**023. File Upload:** `const upload = (e) => vella.collection('Files').create({ file: e.target.files[0] });`
**024. Multi-File Upload:** `Array.from(files).forEach(f => vella.upload(f));`
**025. Upload Progress:** `vella.upload(file, { onProgress: (p) => progress.value = p });`
**026. Checkbox Toggle:** `<input type="checkbox" v-model="isActive" @change="updateActive" />`
**027. Radio Group:** `<input type="radio" value="A" v-model="type" />`
**028. Select Dropdown:** `<select v-model="role"><option>Admin</option></select>`
**029. Multi-Select Array:** `tags.value.push(newTag); await vella.update(id, { tags: tags.value });`
**030. Form Reset:** `formRef.value.reset(); title.value = '';`
**031. Validation (Computed):** `const isValid = computed(() => title.value.trim().length > 0);`
**032. Validation (Regex):** `const isEmailValid = computed(() => /^\S+@\S+\.\S+$/.test(email.value));`
**033. Zod Integration:** `const schema = z.object({ title: z.string() }); schema.parse(formData.value);`
**034. Disabled Submit:** `<button :disabled="isSubmitting || !isValid">Save</button>`
**035. Success Toast:** `vella.create(data).then(() => toast.success("Saved!"));`

---

### 🟠 Part 3: Realtime WebSockets (036 - 055)
**036. Global Realtime Setup:** `onMounted(() => vella.realtime.subscribe('*', handleAllEvents));`
**037. Cleanup Subscription:** `onUnmounted(() => vella.realtime.unsubscribe('*'));`
**038. Record Subscription:** `vella.realtime.subscribe('Task', taskId, handleUpdates);`
**039. Create Event:** `if (e.action === 'CREATE') list.value.unshift(e.record);`
**040. Update Event:** `if (e.action === 'UPDATE') { const i = list.value.findIndex(r => r.id === e.record.id); list.value[i] = e.record; }`
**041. Delete Event:** `if (e.action === 'DELETE') list.value = list.value.filter(r => r.id !== e.record.id);`
**042. Connection Status:** `const status = ref(vella.realtime.status); // 'connected' | 'reconnecting'`
**043. Manual Reconnect:** `<button @click="vella.realtime.reconnect()">Reconnect</button>`
**044. Reactive Subs (VueUse):** `const data = useVellaSubscription('Messages');`
**045. Presence (Join):** `vella.realtime.sendPresence('Room1', { user: 'Alice' });`
**046. Presence (Leave):** `onUnmounted(() => vella.realtime.leavePresence('Room1'));`
**047. Typing Indicator (Send):** `<input @keydown="vella.realtime.broadcast('typing', true)" />`
**048. Typing Indicator (Recv):** `vella.realtime.subscribe('typing', (e) => isTyping.value = e.user);`
**049. Live Cursor (Send):** `<div @mousemove="vella.realtime.broadcast('cursor', {x: $event.clientX, y: $event.clientY})">`
**050. Live Cursor (Recv):** `<div class="absolute" :style="{ left: cursor.x + 'px', top: cursor.y + 'px' }" />`
**051. Notification Bell:** `vella.realtime.subscribe('Alert', () => unreadCount.value++);`
**052. Sync Conflict Warning:** `if (e.record.updated_at > local.value.updated_at) showWarning();`
**053. Chat Auto-Scroll:** `watch(messages, async () => { await nextTick(); bottomRef.value.scrollIntoView(); });`
**054. Read Receipts:** `await vella.update(msgId, { read: true });`
**055. Live Poll Votes:** `vella.realtime.subscribe('Vote', (e) => chartData.value.push(e.record));`

---

### 🔴 Part 4: Auth & Security (056 - 075)
**056. Login:** `await vella.auth.loginWithPassword(email.value, password.value);`
**057. Logout:** `vella.auth.logout(); user.value = null;`
**058. Get Current User:** `const user = ref(vella.auth.getUser());`
**059. Vue Router Guard:** `router.beforeEach((to) => { if (to.meta.requiresAuth && !vella.auth.isValid) return '/login'; });`
**060. Provide Auth State:** `provide('auth', { user, login, logout }); // Use inject('auth') in children`
**061. Google OAuth:** `<button @click="vella.auth.loginWithProvider('google')">Google</button>`
**062. GitHub OAuth:** `<button @click="vella.auth.loginWithProvider('github')">GitHub</button>`
**063. OAuth Callback View:** `onMounted(async () => await vella.auth.handleOAuthCallback());`
**064. Magic Link Request:** `await vella.auth.requestMagicLink(email.value);`
**065. Magic Link Verify:** `await vella.auth.verifyMagicLink(route.query.token);`
**066. Password Reset Req:** `await vella.auth.requestPasswordReset(email.value);`
**067. Password Reset Confirm:** `await vella.auth.confirmPasswordReset(token, newPassword);`
**068. Change Email:** `await vella.auth.requestEmailChange(newEmail);`
**069. Refresh Token:** `await vella.auth.refreshSession();`
**070. Role Check (v-if):** `<div v-if="user?.role === 'Admin'">Admin Panel</div>`
**071. Field-Level RBAC:** `<EditButton v-if="user.permissions.canEdit" />`
**072. Dynamic Class by Role:** `:class="{'hidden': user.role === 'Guest', 'block': user.role !== 'Guest'}"`
**073. Pinia Store Auth Sync:** `authStore.setUser(vella.auth.getUser());`
**074. Remember Me:** `<input type="checkbox" v-model="rememberMe" />`
**075. MFA Prompt:** `if (authRes.requiresMfa) showMfaModal.value = true;`

---

### 🟣 Part 5: AI & RAG Interfaces (076 - 095)
**076. Semantic Search:** `results.value = await vella.ai.searchVector('Knowledge', queryText.value);`
**077. Search Bar UI:** `<input type="search" placeholder="Ask AI natively..." v-model="queryText" @input="search" />`
**078. Streaming Text:** `vella.ai.streamRag(query, (chunk) => streamingText.value += chunk);`
**079. AI Cache Hit Badge:** `<Badge v-if="res.cache_hit">Answered in 1ms (Cached)</Badge>`
**080. Prompt Tokens:** `<span>Tokens used: {{ res.token_usage.total }}</span>`
**081. Highlight RAG Text (v-html):** `<span v-html="highlightAiKeywords(article.text, aiKeywords)"></span>`
**082. Voice to Text:** `recognition.onresult = (e) => vella.ai.searchVector('Docs', e.results[0][0].transcript);`
**083. Confidence Score Bar:** `<div class="bg-green-500 h-1" :style="{ width: (res.confidence * 100) + '%' }" />`
**084. AI Suggestion Chip:** `<button @click="queryText = suggestion">{{ suggestion }}</button>`
**085. Generate Image Hook:** `imageUrl.value = await vella.ai.generateImage(prompt.value);`
**086. Auto-Complete (Ghost Text):** `ghostText.value = await vella.ai.complete(queryText.value);`
**087. AI Scaffolder UI:** `<textarea v-model="schemaDesc" placeholder="Describe database schema..."></textarea>`
**088. Execute Scaffold:** `await vella.admin.generateModel(schemaDesc.value); toast("Deployed!");`
**089. LLM Context Window:** `<div class="overflow-y-auto max-h-96">{{ ragContextChunks }}</div>`
**090. Semantic Chunk Adjuster:** `vella.ai.tuneChunkSize({ size: 1024 });`
**091. Shadow Model UI:** `<select v-model="activeModel"><option>v1-active</option><option>v2-shadow</option></select>`
**092. Shadow Accuracy Diff:** `<div>Variance: {{ shadowStats.variance }}%</div>`
**093. GPU Routing UI:** `<span :class="gpuActive ? 'text-green' : 'text-gray'">CUDA Active</span>`
**094. Vector Upload (Python sync):** `await vella.uploadVector(numpyArrayJson);`
**095. AI Rate Limit Warn:** `if (res.tokens_remaining < 100) showWarning("Rate limit approaching");`

---

### 🔵 Part 6: Complex Tables & Layouts (096 - 120)
**096. Table Header Sort:** `<th @click="setSort('name')">Name <span v-if="sort === 'name'">▲</span></th>`
**097. Data Grid (v-for):** `<tbody><tr v-for="row in rows" :key="row.id"><td>{{ row.name }}</td></tr></tbody>`
**098. Server Pagination:** `watch(currentPage, (page) => vella.collection('Users').getList({ page }));`
**099. Next Page Btn:** `<button @click="currentPage++">Next</button>`
**100. Relational Cell:** `<td>{{ record.expand?.company?.name || 'N/A' }}</td>`
**101. Deep Relational Cell:** `<td>{{ record.expand?.author?.expand?.department?.name }}</td>`
**102. Bulk Select All:** `<input type="checkbox" @change="e => selectedIds = e.target.checked ? allIds : []" />`
**103. Bulk Delete:** `await Promise.all(selectedIds.map(id => vella.delete(id))); refreshList();`
**104. Export CSV:** `const csv = toCsv(data.value); download(csv);`
**105. Export Arrow:** `window.open("https://api.vella.dev/api/d/Data/export?format=arrow");`
**106. Masonry Grid:** `<div class="columns-1 sm:columns-2 md:columns-3"><Card v-for="c in cards" /></div>`
**107. Kanban Column:** `<div class="w-1/3"><Task v-for="t in tasks.filter(t => t.status === col)" /></div>`
**108. Drag Start:** `<div draggable="true" @dragstart="draggedId = task.id">`
**109. Drop Zone:** `<div @dragover.prevent @drop="updateTaskStatus(draggedId, col)">`
**110. Infinite Scroll (VueUse):** `const { arrivedState } = useScroll(el); watch(arrivedState.bottom, () => page++);`
**111. Virtualized List (VueVirtualScroller):** `<RecycleScroller :items="items" :item-size="32" />`
**112. Search Debounce (watch):** `watch(searchTerm, debounce((term) => fetchFiltered(term), 300));`
**113. Sticky Header:** `<thead class="sticky top-0 bg-white shadow-sm">`
**114. Row Hover:** `<tr class="hover:bg-blue-50 transition-colors duration-200">`
**115. Dynamic Status Badge:** `<span :class="['badge', 'badge-' + record.status]">{{ record.status }}</span>`
**116. Avatar Group:** `<div class="flex -space-x-2"><Avatar v-for="u in users" :src="u.img" /></div>`
**117. Accordion Row:** `<tr @click="expandedId = r.id"><ExpandedData v-if="expandedId === r.id" /></tr>`
**118. Context Menu:** `@contextmenu.prevent="showMenu($event.clientX, $event.clientY)"`
**119. Tooltip Cell:** `<td :title="record.long_description">{{ record.short_description }}</td>`
**120. Date Formatting:** `<td>{{ new Date(record.created_at).toLocaleDateString() }}</td>`

---

### ⚫ Part 7: Enterprise, SCADA & F1 Components (121 - 150)
**121. Approval Workflow (Manager):** `<button @click="vella.cms.approve(id)">Approve</button>`
**122. Reject Workflow:** `<button @click="vella.cms.reject(id, reason)">Reject</button>`
**123. Audit Log Viewer:** `logs.value = await vella.collection('AuditLogs').getList();`
**124. Time-Travel Rollback:** `<button @click="vella.audit.rollback(log.id)">⏪ Revert</button>`
**125. SCADA Tank Fill (SVG dynamic binding):** `<rect y="100 - level" :height="level" fill="blue" />`
**126. SCADA ISA-18 Alarm (Unack):** `<div class="animate-pulse bg-red-600 text-white">UNACK_ACTIVE</div>`
**127. SCADA Alarm Acknowledge:** `<button @click="vella.scada.ackAlarm('PUMP_1')">Acknowledge</button>`
**128. SCADA Alarm Cleared:** `<div class="bg-yellow-500">UNACK_CLEARED</div>`
**129. Swinging Door Compression Toggle:** `<button @click="vella.scada.tuneCompression(0.5)">Set Tolerance</button>`
**130. TMR Voter Status:** `<div>Consensus: {{ tmrStatus === 'A_B_C' ? 'Healthy' : 'Diverged' }}</div>`
**131. Modbus Manual Override:** `<button @click="vella.scada.writeCoil(40001, true)">OPEN VALVE</button>`
**132. F1 UDP Stream Status:** `<div :class="udpActive ? 'text-green-500' : 'text-red-500'">Radio Link</div>`
**133. Time-Series Chart (Vue-Chartjs):** `<LineChart :chart-data="tsData" />`
**134. Dynamic Time-Bucket Selector:** `<select v-model="bucketSize"><option value="100">100ms</option></select>`
**135. MPI Cluster Status:** `<div>Nodes Converged: {{ mpi.converged }} / {{ mpi.total }}</div>`
**136. RTOS Priority Indicator:** `<Badge color="purple" v-if="rtosActive">Hard Real-Time Loop Active</Badge>`
**137. 1000Hz IPC Shared Memory Poller:** `requestAnimationFrame(() => uiState.value = vella.ipc.readFrame());`
**138. Wasm UDF Uploader:** `<input type="file" accept=".wasm" @change="e => vella.admin.uploadWasm(e.target.files[0])" />`
**139. Chaos Monkey Config:** `<input type="range" min="0" max="100" v-model="chaosProb" />`
**140. Chaos Monkey Inject:** `<button @click="vella.chaos.triggerPartition()">Simulate Outage</button>`
**141. Cassandra Multi-Region Map:** `<Map :active-regions="['us-east', 'eu-west']" />`
**142. Graph DB Node Graph (D3/Vue):** `<ForceGraph3D :graph-data="cypherResults" />`
**143. HLS Video Player (Video.js):** `<video ref="videoPlayer" class="video-js vjs-default-skin"></video>`
**144. DRM License Key Injector:** `player.src({ src: manifestUrl, type: 'application/x-mpegURL', keySystems: widevine });`
**145. CDN Global Purge Btn:** `<button @click="vella.cdn.purge('movie_meta')">Purge Edge Cache</button>`
**146. Computer Vision Intro Skipper:** `<button v-if="currentTime > introEnd">Skip Intro</button>`
**147. Smart Thumbnail Image:** `<img :src="vella.vision.getSmartThumbnail(videoId)" />`
**148. System CPU Heat Gauge:** `<div :style="{ backgroundColor: temp > 85 ? 'red' : 'green' }">{{ temp }}°C</div>`
**149. AI Tuner Auto-Fix Log:** `<ul><li v-for="l in tunerLogs">{{ l.action_taken }}</li></ul>`
**150. Global Error Boundary (Vue onErrorCaptured):** `onErrorCaptured((err) => { globalError.value = err; return false; });`
