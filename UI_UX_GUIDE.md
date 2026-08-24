# 🎨 Vella: UI/UX & Frontend Programmer Guide

For a UI/UX Programmer, Vella eliminates backend boilerplate so you can focus entirely on animations, layouts, and user flows. Because Vella auto-generates a strict **TypeScript SDK**, you get perfect IDE autocomplete for your database schemas.

Here are **50 progressive UI/UX React/Tailwind examples**, ranging from basic data fetching to highly complex Real-Time, AI, and Enterprise interfaces.

---

## Level 1: The Basics (1 - 5)

**1. Initialize the SDK**
```typescript
import { VellaClient } from '../types/vella';
export const vella = new VellaClient("https://api.vella.dev");
```

**2. Basic Data Fetching (Simple List)**
```tsx
const [users, setUsers] = useState([]);
useEffect(() => { vella.collection('User').getList().then(setUsers); }, []);
return <ul>{users.map(u => <li key={u.id}>{u.name}</li>)}</ul>;
```

**3. Loading Skeletons (UX)**
```tsx
if (loading) return <div className="animate-pulse h-10 w-full bg-gray-200 rounded"></div>;
```

**4. Empty State UX**
```tsx
if (users.length === 0) return <div className="text-gray-500 text-center">No users found.</div>;
```

**5. Error State UI**
```tsx
if (error) return <div className="bg-red-100 text-red-700 p-4 rounded">{error.message}</div>;
```

---

## Level 2: Layouts & Views (6 - 15)

**6. Glassmorphic Data Card**
```tsx
<div className="backdrop-blur-md bg-white/30 border border-white/20 shadow-xl rounded-xl p-6">
  <h2 className="text-xl font-bold">{record.title}</h2>
</div>
```

**7. Responsive CSS Grid**
```tsx
<div className="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-4">
  {items.map(item => <ProductCard key={item.id} data={item} />)}
</div>
```

**8. Single Record Detail View**
```tsx
vella.collection('Article').getById(id).then(setArticle);
return <article className="prose lg:prose-xl">{article.content}</article>;
```

**9. Hover Animations on Records**
```tsx
<div className="transition transform hover:-translate-y-1 hover:shadow-2xl">{data.name}</div>
```

**10. Truncated Text UX**
```tsx
<p className="truncate w-48 text-sm text-gray-600">{data.long_description}</p>
```

**11. Status Badges (Enums)**
```tsx
const colors = { Draft: 'bg-gray-200', Published: 'bg-green-200 text-green-800' };
<span className={`px-2 py-1 rounded-full ${colors[data.status]}`}>{data.status}</span>
```

**12. Avatar Fallbacks**
```tsx
<img src={data.avatar_url || '/default-avatar.png'} className="w-12 h-12 rounded-full border-2" />
```

**13. Masonry Layout (Pinterest style)**
```tsx
<div className="columns-2 md:columns-3 gap-4">{images.map(img => <ImageCard data={img} />)}</div>
```

**14. Dark Mode Support**
```tsx
<div className="bg-white dark:bg-slate-800 text-black dark:text-white">{data.text}</div>
```

**15. Sticky Header Lists**
```tsx
<div className="sticky top-0 bg-white/80 backdrop-blur pb-2">User Directory</div>
```

---

## Level 3: Forms & Mutations (16 - 25)

**16. Simple Create Form**
```tsx
<form onSubmit={(e) => { e.preventDefault(); vella.collection('Task').create({ title }); }}>
```

**17. Auto-Saving Inputs (Debounced)**
```tsx
<input onChange={(e) => debouncedSave(e.target.value)} placeholder="Type to save..." />
```

**18. Delete Confirmation Modal**
```tsx
<button onClick={() => confirm("Delete?") && vella.collection('Post').delete(id)}>Delete</button>
```

**19. Optimistic UI Updates**
```tsx
// Update UI instantly before DB responds
setItems(prev => [...prev, newItem]); 
vella.collection('Item').create(newItem).catch(() => revertUI());
```

**20. Form Validation State**
```tsx
<input className={`border ${errors.email ? 'border-red-500' : 'border-gray-300'}`} />
```

**21. Toggle Switches (Booleans)**
```tsx
<button className={`${isActive ? 'bg-blue-500' : 'bg-gray-300'} rounded-full w-12 h-6`} 
        onClick={() => vella.collection('Settings').update(id, { isActive: !isActive })}>
```

**22. Image Upload Preview**
```tsx
<input type="file" onChange={(e) => setPreview(URL.createObjectURL(e.target.files[0]))} />
```

**23. Star Rating Component**
```tsx
{[1,2,3,4,5].map(star => <StarIcon onClick={() => vella.collection('Review').update(id, { rating: star })} />)}
```

**24. Multi-Select Tags**
```tsx
<div className="flex gap-2">{tags.map(tag => <Chip onClick={() => removeTag(tag)}>{tag}</Chip>)}</div>
```

**25. Progress Bar Form UX**
```tsx
<div className="w-full bg-gray-200 h-2"><div className="bg-blue-500 h-2" style={{width: `${uploadProgress}%`}}></div></div>
```

---

## Level 4: Vella Realtime WebSockets (26 - 30)

**26. Live Notifications Dropdown**
```tsx
useRealtimeSubscription('Alert', (e) => setUnreadCount(prev => prev + 1));
```

**27. Real-Time Kanban Drag & Drop**
```tsx
// Listens to Vella WS, instantly moves card across screen when another user drags it
useRealtimeSubscription('Task', (e) => updateBoardState(e.record));
```

**28. Live Viewer Count (Presence)**
```tsx
<div className="flex items-center gap-2"><span className="animate-ping bg-red-500 rounded-full w-2 h-2"></span> {liveUsers} watching</div>
```

**29. Typing Indicators**
```tsx
useRealtimeSubscription('ChatEvent', (e) => e.isTyping ? showIndicator() : hideIndicator());
```

**30. Auto-Scrolling Chat Window**
```tsx
useEffect(() => { chatEndRef.current?.scrollIntoView({ behavior: "smooth" }) }, [messages]);
```

---

## Level 5: Auth, Security & Routing (31 - 35)

**31. Magic Link Login UI**
```tsx
<button onClick={() => vella.auth.requestMagicLink(email)}>Send Login Link</button>
```

**32. OAuth Button Group**
```tsx
<button className="bg-white border rounded shadow flex items-center" onClick={vella.auth.google}>
  <GoogleIcon /> Sign in with Google
</button>
```

**33. Role-Based Component Visibility**
```tsx
{vella.auth.user.role === 'Admin' && <AdminSettingsPanel />}
```

**34. Locked Route UX**
```tsx
if (!vella.auth.isAuthenticated) return <Navigate to="/login" replace />;
```

**35. Profile Dropdown Menu**
```tsx
<div className="absolute right-0 bg-white shadow-lg"><button onClick={vella.auth.logout}>Sign Out</button></div>
```

---

## Level 6: AI & RAG Interfaces (36 - 42)

**36. Semantic Search Bar**
```tsx
<input placeholder="Describe what you're looking for natively..." 
       onChange={(e) => vella.ai.searchVector('Knowledge', e.target.value).then(setResults)} />
```

**37. ChatGPT-Style Streaming UI**
```tsx
// Appending typing chunks to state to simulate streaming response
<div className="bg-gray-100 p-4 rounded-xl font-mono">{streamingAiText}<span className="animate-pulse">|</span></div>
```

**38. AI Cache Hit Indicator**
```tsx
{response.cache_hit && <span className="text-xs text-green-500">⚡ Answered in 0.5ms (AI Cache)</span>}
```

**39. Smart Highlight Text (RAG)**
```tsx
<p dangerouslySetInnerHTML={{ __html: highlightAiKeywords(article.text, aiExtractedKeywords) }}></p>
```

**40. AI "Generate Image" Button**
```tsx
<button className="bg-gradient-to-r from-purple-500 to-pink-500 text-white animate-pulse">✨ Generate Image</button>
```

**41. AI Confidence Score Bar**
```tsx
<div className="h-1 bg-green-500" style={{ width: `${aiConfidence * 100}%` }}></div>
```

**42. Voice to Text Search Toggle**
```tsx
<button className={isRecording ? 'text-red-500 animate-bounce' : 'text-gray-500'}><MicIcon/></button>
```

---

## Level 7: Advanced Data Tables & Dashboards (43 - 47)

**43. Deep Relational Expansion Table**
```tsx
// Fetches Author and Company joined in one request via Vella GraphQL-lite
vella.collection('Article').getList({ expand: 'author.company' });
<td>{record.expand.author.expand.company.name}</td>
```

**44. Server-Side Pagination Controls**
```tsx
<button disabled={page === 1} onClick={() => setPage(p => p - 1)}>Prev</button>
<button disabled={page === totalPages} onClick={() => setPage(p => p + 1)}>Next</button>
```

**45. Debounced Server-Side Filtering**
```tsx
vella.collection('Users').getList({ filter: `email~'${debouncedSearch}'` });
```

**46. Sorting Headers UX**
```tsx
<th onClick={() => setSort('-created_at')} className="cursor-pointer hover:bg-gray-100">Date ⬇️</th>
```

**47. Polars/Arrow Chart Integration (Recharts/Chart.js)**
```tsx
// Assuming Python processed the Arrow stream and output JSON summary
<LineChart data={chartData}><Line type="monotone" dataKey="sales" stroke="#8884d8" /></LineChart>
```

---

## Level 8: Enterprise & Industrial "Master" UX (48 - 50)

**48. Enterprise Approval Workflow UI**
```tsx
{record.status === 'Pending_Approval' && (
  <div className="flex gap-2">
    <button className="bg-green-500 text-white" onClick={() => vella.cms.approve(record.id)}>Approve</button>
    <button className="bg-red-500 text-white" onClick={() => vella.cms.reject(record.id)}>Reject</button>
  </div>
)}
```

**49. SCADA Industrial Dashboard (HMI Canvas)**
```tsx
// SVG tank fills up based on realtime SCADA OPC UA data from Vella
<svg viewBox="0 0 100 100">
  <rect x="10" y={100 - scadaLevel} width="80" height={scadaLevel} fill={scadaLevel > 90 ? 'red' : 'blue'} className="transition-all duration-300"/>
</svg>
```

**50. 1-Click Time-Travel Rollback UI**
```tsx
<div className="bg-yellow-50 border-l-4 border-yellow-400 p-4">
  <p>Warning: System state altered.</p>
  <button onClick={() => vella.audit.rollback(logId)} className="text-yellow-700 underline font-bold">
    ⏪ Undo Change (Time Travel)
  </button>
</div>
```
