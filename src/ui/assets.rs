pub fn admin_react_spa_html(site_name: &str) -> String {
    let raw_html = r#"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>__SITE_NAME__ - Vella Headless CMS & LLM Engine</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Plus+Jakarta+Sans:wght@300;400;500;600;700;800&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {
      darkMode: 'class',
      theme: {
        extend: {
          fontFamily: {
            sans: ['"Plus Jakarta Sans"', 'sans-serif'],
            mono: ['"JetBrains Mono"', 'monospace'],
          },
          colors: {
            brand: {
              400: '#38bdf8',
              500: '#0284c7',
              600: '#0369a1',
              700: '#075985',
            },
            ai: {
              400: '#c084fc',
              500: '#a855f7',
              600: '#9333ea',
            },
            vella: {
              50: '#f8fafc',
              800: '#1e293b',
              900: '#0f172a',
              950: '#020617',
            }
          }
        }
      }
    }
  </script>
  <!-- React 18 & Babel Standalone -->
  <script src="https://unpkg.com/react@18/umd/react.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/react-dom@18/umd/react-dom.production.min.js" crossorigin></script>
  <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
  <!-- Lucide Icons -->
  <script src="https://unpkg.com/lucide@latest"></script>
  <style>
    body {
      background: radial-gradient(circle at 50% 0%, #1e1b4b 0%, #0b0f19 75%);
      min-height: 100vh;
      color: #f1f5f9;
    }
    .glass-panel {
      background: rgba(15, 23, 42, 0.75);
      backdrop-filter: blur(16px);
      -webkit-backdrop-filter: blur(16px);
      border: 1px solid rgba(255, 255, 255, 0.08);
    }
    .glass-card {
      background: rgba(30, 41, 59, 0.5);
      backdrop-filter: blur(12px);
      border: 1px solid rgba(255, 255, 255, 0.06);
    }
    .glass-input {
      background: rgba(15, 23, 42, 0.6);
      border: 1px solid rgba(255, 255, 255, 0.12);
    }
    .glass-input:focus {
      border-color: #38bdf8;
      outline: none;
      box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.2);
    }
    /* Custom Scrollbar */
    ::-webkit-scrollbar { width: 6px; height: 6px; }
    ::-webkit-scrollbar-track { background: transparent; }
    ::-webkit-scrollbar-thumb { background: rgba(255, 255, 255, 0.15); border-radius: 9999px; }
    ::-webkit-scrollbar-thumb:hover { background: rgba(255, 255, 255, 0.25); }
    @keyframes pulse-slow {
      0%, 100% { opacity: 1; }
      50% { opacity: 0.4; }
    }
    .animate-pulse-slow { animation: pulse-slow 3s cubic-bezier(0.4, 0, 0.6, 1) infinite; }
  </style>
</head>
<body class="font-sans antialiased text-slate-100 selection:bg-brand-500 selection:text-white">
  <div id="root"></div>

  <script type="text/babel">
    const { useState, useEffect, useRef } = React;

    // Toast Notification helper
    function Toast({ message, type, onClose }) {
      useEffect(() => {
        const t = setTimeout(onClose, 4000);
        return () => clearTimeout(t);
      }, [onClose]);

      const colors = {
        success: 'bg-emerald-500/20 border-emerald-500/50 text-emerald-300',
        error: 'bg-rose-500/20 border-rose-500/50 text-rose-300',
        info: 'bg-sky-500/20 border-sky-500/50 text-sky-300',
      };

      return (
        <div className={`fixed bottom-6 right-6 z-50 px-4 py-3 rounded-xl border backdrop-blur-xl shadow-2xl flex items-center gap-3 transition-all ${colors[type] || colors.info}`}>
          <span className="text-sm font-medium">{message}</span>
          <button onClick={onClose} className="opacity-60 hover:opacity-100">&times;</button>
        </div>
      );
    }

    function VellaAdminApp() {
      const [user, setUser] = useState(null);
      const [authChecking, setAuthChecking] = useState(true);
      const [schemas, setSchemas] = useState([]);
      const [activeTab, setActiveTab] = useState('models'); // models, ai-scaffolder, vector-playground, ai-tuner, semantic-cache, approvals, audit, health
      const [selectedModel, setSelectedModel] = useState(null);
      const [records, setRecords] = useState([]);
      const [totalRecords, setTotalRecords] = useState(0);
      const [page, setPage] = useState(1);
      const [searchQuery, setSearchQuery] = useState('');
      const [loading, setLoading] = useState(false);
      const [toast, setToast] = useState(null);

      // Modals
      const [isEditModalOpen, setIsEditModalOpen] = useState(false);
      const [editingRecord, setEditingRecord] = useState(null);
      const [formData, setFormData] = useState({});

      // Realtime state
      const [realtimeActive, setRealtimeActive] = useState(false);
      const [recentEvents, setRecentEvents] = useState([]);

      // Scaffolder state
      const [scaffoldPrompt, setScaffoldPrompt] = useState('');
      const [scaffoldModelName, setScaffoldModelName] = useState('');
      const [scaffoldResult, setScaffoldResult] = useState(null);
      const [scaffoldLoading, setScaffoldLoading] = useState(false);

      // Vector Playground state
      const [vectorQueryStr, setVectorQueryStr] = useState('[0.05, -0.12, 0.88, 0.42, -0.15]');
      const [vectorResults, setVectorResults] = useState([]);
      const [vectorLoading, setVectorLoading] = useState(false);

      // AI Tuner state
      const [tunerReport, setTunerReport] = useState(null);

      // Approvals state
      const [approvals, setApprovals] = useState([]);

      // Audit logs
      const [auditLogs, setAuditLogs] = useState([]);

      // System Health
      const [healthData, setHealthData] = useState(null);

      // Show toast
      const notify = (message, type = 'success') => {
        setToast({ message, type });
      };

      // Check auth status
      const checkAuth = async () => {
        try {
          const res = await fetch('/api/auth/me');
          if (res.ok) {
            const data = await res.json();
            setUser(data.user);
          } else {
            setUser(null);
          }
        } catch (e) {
          setUser(null);
        } finally {
          setAuthChecking(false);
        }
      };

      // Load Schemas
      const loadSchemas = async () => {
        try {
          const res = await fetch('/api/d/schema');
          if (res.ok) {
            const data = await res.json();
            setSchemas(data.schemas || []);
            if (data.schemas?.length > 0 && !selectedModel) {
              setSelectedModel(data.schemas[0]);
            }
          }
        } catch (e) {
          console.error(e);
        }
      };

      // Load records for selected model
      const loadRecords = async (schema, p = 1, search = '') => {
        if (!schema) return;
        setLoading(true);
        try {
          const limit = 20;
          const offset = (p - 1) * limit;
          let url = `/api/d/${schema.name.toLowerCase()}?$limit=${limit}&$offset=${offset}`;
          if (search) url += `&$search=${encodeURIComponent(search)}`;

          const res = await fetch(url);
          if (res.ok) {
            const data = await res.json();
            setRecords(data.data || []);
            setTotalRecords(data.total || 0);
          }
        } catch (e) {
          notify('Failed to load records', 'error');
        } finally {
          setLoading(false);
        }
      };

      // Setup Realtime WebSocket
      useEffect(() => {
        const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
        const wsUrl = `${protocol}//${window.location.host}/api/realtime/ws`;
        let ws;

        try {
          ws = new WebSocket(wsUrl);
          ws.onopen = () => setRealtimeActive(true);
          ws.onclose = () => setRealtimeActive(false);
          ws.onmessage = (evt) => {
            try {
              const msg = JSON.parse(evt.data);
              setRecentEvents((prev) => [msg, ...prev.slice(0, 19)]);
              if (selectedModel && msg.topic === `models:${selectedModel.name.toLowerCase()}`) {
                loadRecords(selectedModel, page, searchQuery);
              }
            } catch (err) {}
          };
        } catch (e) {
          console.log('Realtime WS error', e);
        }

        return () => {
          if (ws) ws.close();
        };
      }, [selectedModel, page, searchQuery]);

      useEffect(() => {
        checkAuth();
        loadSchemas();
      }, []);

      useEffect(() => {
        if (selectedModel) {
          loadRecords(selectedModel, page, searchQuery);
        }
      }, [selectedModel, page, searchQuery]);

      useEffect(() => {
        if (window.lucide) {
          window.lucide.createIcons();
        }
      });

      // Handle Login
      const handleLogin = async (e) => {
        e.preventDefault();
        const username = e.target.username.value;
        const password = e.target.password.value;
        try {
          const res = await fetch('/api/auth/login', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password }),
          });
          const data = await res.json();
          if (res.ok) {
            notify('Welcome back, ' + data.session.username);
            checkAuth();
          } else {
            notify(data.error?.message || 'Login failed', 'error');
          }
        } catch (err) {
          notify('Network error during login', 'error');
        }
      };

      // Handle Logout
      const handleLogout = async () => {
        await fetch('/api/auth/logout', { method: 'POST' });
        setUser(null);
        notify('Logged out successfully');
      };

      // Handle Scaffolding Run
      const handleRunScaffolder = async () => {
        if (!scaffoldPrompt) {
          notify('Please enter an AI prompt description', 'error');
          return;
        }
        setScaffoldLoading(true);
        try {
          const res = await fetch('/api/ai/generate-model', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ name: scaffoldModelName || 'Item', prompt: scaffoldPrompt }),
          });
          const data = await res.json();
          if (res.ok) {
            setScaffoldResult(data.result);
            notify('Schema scaffolded successfully!');
          }
        } catch (err) {
          notify('AI Scaffolding error', 'error');
        } finally {
          setScaffoldLoading(false);
        }
      };

      // Handle Vector Search
      const handleVectorSearch = async () => {
        if (!selectedModel) return;
        setVectorLoading(true);
        try {
          let vec;
          try {
            vec = JSON.parse(vectorQueryStr);
          } catch (e) {
            vec = vectorQueryStr.split(',').map((v) => parseFloat(v.trim())).filter((n) => !isNaN(n));
          }

          const res = await fetch(`/api/d/${selectedModel.name.toLowerCase()}/search-vector`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              model: selectedModel.name,
              query_vector: vec,
              top_k: 5,
              metric: 'Cosine',
            }),
          });
          const data = await res.json();
          if (res.ok) {
            setVectorResults(data.results || []);
            notify(`Retrieved ${data.results?.length || 0} vector matches`);
          } else {
            notify(data.error?.message || 'Vector search failed', 'error');
          }
        } catch (e) {
          notify('Vector search execution error', 'error');
        } finally {
          setVectorLoading(false);
        }
      };

      // Load AI Tuner Report
      const loadTunerReport = async () => {
        try {
          const res = await fetch('/api/ai/report');
          if (res.ok) {
            const data = await res.json();
            setTunerReport(data.report);
          }
        } catch (e) {}
      };

      // Load Approvals
      const loadApprovals = async () => {
        try {
          const res = await fetch('/api/d/approvals');
          if (res.ok) {
            const data = await res.json();
            setApprovals(data.data || []);
          }
        } catch (e) {}
      };

      // Load Audit Logs
      const loadAuditLogs = async () => {
        try {
          const res = await fetch('/api/d/audit-logs?limit=30');
          if (res.ok) {
            const data = await res.json();
            setAuditLogs(data.data || []);
          }
        } catch (e) {}
      };

      // Load Health Data
      const loadHealth = async () => {
        try {
          const res = await fetch('/health');
          if (res.ok) {
            const data = await res.json();
            setHealthData(data);
          }
        } catch (e) {}
      };

      useEffect(() => {
        if (activeTab === 'ai-tuner') loadTunerReport();
        if (activeTab === 'approvals') loadApprovals();
        if (activeTab === 'audit') loadAuditLogs();
        if (activeTab === 'health') loadHealth();
      }, [activeTab]);

      // Save Form (Create / Edit Record)
      const handleSaveRecord = async (e) => {
        e.preventDefault();
        const isEditing = !!editingRecord;
        const method = isEditing ? 'PUT' : 'POST';
        const url = isEditing
          ? `/api/d/${selectedModel.name.toLowerCase()}/${editingRecord.id}`
          : `/api/d/${selectedModel.name.toLowerCase()}`;

        try {
          const res = await fetch(url, {
            method,
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(formData),
          });
          const data = await res.json();
          if (res.ok) {
            notify(isEditing ? 'Record updated' : 'Record created');
            setIsEditModalOpen(false);
            setEditingRecord(null);
            setFormData({});
            loadRecords(selectedModel, page, searchQuery);
          } else {
            notify(data.error?.message || 'Save failed', 'error');
          }
        } catch (err) {
          notify('Error saving record', 'error');
        }
      };

      // Delete Record
      const handleDeleteRecord = async (id) => {
        if (!confirm(`Delete record #${id}? This action will be logged in the audit trail.`)) return;
        try {
          const res = await fetch(`/api/d/${selectedModel.name.toLowerCase()}/${id}`, {
            method: 'DELETE',
          });
          if (res.ok) {
            notify(`Record #${id} deleted`);
            loadRecords(selectedModel, page, searchQuery);
          } else {
            notify('Delete failed', 'error');
          }
        } catch (err) {
          notify('Error deleting record', 'error');
        }
      };

      // Time-Travel Rollback
      const handleRollback = async (logId) => {
        if (!confirm(`Execute 1-Click Time-Travel Rollback to restore state at Log #${logId}?`)) return;
        try {
          const res = await fetch(`/api/d/rollback/${logId}`, { method: 'POST' });
          if (res.ok) {
            notify('Time-travel rollback applied successfully!');
            loadAuditLogs();
            if (selectedModel) loadRecords(selectedModel, page, searchQuery);
          } else {
            notify('Rollback failed', 'error');
          }
        } catch (e) {
          notify('Rollback error', 'error');
        }
      };

      if (authChecking) {
        return (
          <div className="min-h-screen flex items-center justify-center">
            <div className="flex items-center gap-3 text-sky-400 font-medium">
              <i data-lucide="loader" className="w-5 h-5 animate-spin"></i>
              <span>Initializing Vella Engine...</span>
            </div>
          </div>
        );
      }

      if (!user) {
        return (
          <div className="min-h-screen flex items-center justify-center p-4">
            <div className="glass-panel w-full max-w-md p-8 rounded-2xl shadow-2xl relative overflow-hidden">
              <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-sky-400 via-indigo-500 to-purple-500"></div>
              <div className="text-center mb-8">
                <div className="inline-flex items-center justify-center w-14 h-14 rounded-2xl bg-sky-500/10 border border-sky-500/30 text-sky-400 mb-4 shadow-inner">
                  <i data-lucide="sparkles" className="w-7 h-7"></i>
                </div>
                <h1 className="text-2xl font-bold tracking-tight text-white">Vella Headless CMS</h1>
                <p className="text-sm text-slate-400 mt-1">Sign in with superadmin or OAuth credentials</p>
              </div>

              <form onSubmit={handleLogin} className="space-y-4">
                <div>
                  <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Username or Email</label>
                  <input
                    type="text"
                    name="username"
                    defaultValue="admin"
                    required
                    className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white placeholder-slate-500"
                    placeholder="admin or admin@vella.dev"
                  />
                </div>
                <div>
                  <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Password</label>
                  <input
                    type="password"
                    name="password"
                    defaultValue="admin"
                    required
                    className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white placeholder-slate-500"
                    placeholder="••••••••"
                  />
                </div>
                <button
                  type="submit"
                  className="w-full py-3 px-4 rounded-xl bg-gradient-to-r from-sky-500 to-blue-600 hover:from-sky-400 hover:to-blue-500 text-white font-semibold text-sm shadow-lg shadow-sky-500/25 transition-all"
                >
                  Sign In to Dashboard
                </button>
              </form>

              <div className="mt-6 pt-6 border-t border-slate-800/80 text-center">
                <p className="text-xs text-slate-500">Default Superadmin: <code className="text-sky-400 bg-sky-950/50 px-1.5 py-0.5 rounded">admin / admin</code></p>
              </div>
            </div>
            {toast && <Toast {...toast} onClose={() => setToast(null)} />}
          </div>
        );
      }

      return (
        <div className="min-h-screen flex flex-col">
          {/* Top Bar */}
          <header className="glass-panel sticky top-0 z-40 border-b border-slate-800 px-6 py-3.5 flex items-center justify-between">
            <div className="flex items-center gap-4">
              <div className="flex items-center gap-3">
                <div className="w-9 h-9 rounded-xl bg-gradient-to-br from-sky-400 to-indigo-600 flex items-center justify-center font-bold text-white shadow-md">
                  V
                </div>
                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-bold tracking-tight text-white">__SITE_NAME__</span>
                    <span className="text-xs px-2 py-0.5 rounded-full bg-sky-500/10 border border-sky-500/30 text-sky-400 font-medium">v0.1.0</span>
                  </div>
                  <div className="flex items-center gap-2 text-xs text-slate-400">
                    <span>SQLite / PostgreSQL Scale</span>
                    <span>•</span>
                    <span className="flex items-center gap-1">
                      <span className={`w-2 h-2 rounded-full ${realtimeActive ? 'bg-emerald-400 animate-pulse' : 'bg-amber-400'}`}></span>
                      {realtimeActive ? 'Realtime Live' : 'Polling'}
                    </span>
                  </div>
                </div>
              </div>
            </div>

            <div className="flex items-center gap-3">
              <a
                href="/swagger"
                target="_blank"
                className="px-3 py-1.5 rounded-lg glass-card hover:bg-slate-800 text-xs font-medium text-slate-300 flex items-center gap-1.5 transition-all"
              >
                <i data-lucide="file-code" className="w-3.5 h-3.5 text-sky-400"></i>
                Swagger API
              </a>
              <a
                href="/api/types/typescript.d.ts"
                target="_blank"
                className="px-3 py-1.5 rounded-lg glass-card hover:bg-slate-800 text-xs font-medium text-slate-300 flex items-center gap-1.5 transition-all"
              >
                <i data-lucide="code" className="w-3.5 h-3.5 text-indigo-400"></i>
                .d.ts Types
              </a>
              <div className="h-5 w-px bg-slate-800 mx-1"></div>
              <div className="flex items-center gap-2 px-3 py-1.5 rounded-lg glass-card text-xs">
                <div className="w-2 h-2 rounded-full bg-emerald-400"></div>
                <span className="text-slate-300 font-medium">{user.username}</span>
                <span className="text-slate-500">({user.role})</span>
              </div>
              <button
                onClick={handleLogout}
                className="p-1.5 rounded-lg glass-card hover:bg-rose-500/20 hover:text-rose-300 text-slate-400 transition-all"
                title="Logout"
              >
                <i data-lucide="log-out" className="w-4 h-4"></i>
              </button>
            </div>
          </header>

          {/* Main Layout */}
          <div className="flex-1 flex overflow-hidden">
            {/* Sidebar */}
            <aside className="w-64 glass-panel border-r border-slate-800/80 p-4 flex flex-col gap-6 overflow-y-auto">
              <div>
                <div className="text-xs font-semibold uppercase tracking-wider text-slate-400 px-3 mb-2">Content Models</div>
                <nav className="space-y-1">
                  {schemas.map((s) => (
                    <button
                      key={s.name}
                      onClick={() => {
                        setSelectedModel(s);
                        setActiveTab('models');
                      }}
                      className={`w-full flex items-center justify-between px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                        activeTab === 'models' && selectedModel?.name === s.name
                          ? 'bg-sky-500/15 border border-sky-500/30 text-sky-300 shadow-sm'
                          : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                      }`}
                    >
                      <div className="flex items-center gap-2.5">
                        <i data-lucide={s.icon || 'box'} className="w-4 h-4 text-sky-400"></i>
                        <span>{s.display_name || s.name}</span>
                      </div>
                      {s.has_vectors && (
                        <span className="text-[10px] px-1.5 py-0.5 rounded bg-purple-500/20 text-purple-300 font-mono">vec</span>
                      )}
                    </button>
                  ))}
                </nav>
              </div>

              <div>
                <div className="text-xs font-semibold uppercase tracking-wider text-slate-400 px-3 mb-2">AI & LLM Native</div>
                <nav className="space-y-1">
                  <button
                    onClick={() => setActiveTab('ai-scaffolder')}
                    className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                      activeTab === 'ai-scaffolder'
                        ? 'bg-purple-500/15 border border-purple-500/30 text-purple-300'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                    }`}
                  >
                    <i data-lucide="sparkles" className="w-4 h-4 text-purple-400"></i>
                    <span>AI Scaffolder</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('vector-playground')}
                    className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                      activeTab === 'vector-playground'
                        ? 'bg-purple-500/15 border border-purple-500/30 text-purple-300'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                    }`}
                  >
                    <i data-lucide="target" className="w-4 h-4 text-indigo-400"></i>
                    <span>Vector & RAG Studio</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('ai-tuner')}
                    className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                      activeTab === 'ai-tuner'
                        ? 'bg-purple-500/15 border border-purple-500/30 text-purple-300'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                    }`}
                  >
                    <i data-lucide="gauge" className="w-4 h-4 text-amber-400"></i>
                    <span>AI Tuner & DDL</span>
                  </button>
                </nav>
              </div>

              <div>
                <div className="text-xs font-semibold uppercase tracking-wider text-slate-400 px-3 mb-2">Governance & Ops</div>
                <nav className="space-y-1">
                  <button
                    onClick={() => setActiveTab('approvals')}
                    className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                      activeTab === 'approvals'
                        ? 'bg-sky-500/15 border border-sky-500/30 text-sky-300'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                    }`}
                  >
                    <i data-lucide="shield-check" className="w-4 h-4 text-emerald-400"></i>
                    <span>Approval Queue</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('audit')}
                    className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                      activeTab === 'audit'
                        ? 'bg-sky-500/15 border border-sky-500/30 text-sky-300'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                    }`}
                  >
                    <i data-lucide="history" className="w-4 h-4 text-sky-400"></i>
                    <span>Audit & Rollback</span>
                  </button>
                  <button
                    onClick={() => setActiveTab('health')}
                    className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-xl text-sm font-medium transition-all ${
                      activeTab === 'health'
                        ? 'bg-sky-500/15 border border-sky-500/30 text-sky-300'
                        : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800/50'
                    }`}
                  >
                    <i data-lucide="activity" className="w-4 h-4 text-rose-400"></i>
                    <span>Self-Healing Health</span>
                  </button>
                </nav>
              </div>

              {/* Realtime Live Pulse */}
              <div className="mt-auto pt-4 border-t border-slate-800/80">
                <div className="text-[11px] uppercase tracking-wider text-slate-500 font-semibold mb-2 flex items-center justify-between">
                  <span>Live Stream</span>
                  <span className="text-[10px] text-emerald-400 font-mono">WS Active</span>
                </div>
                <div className="space-y-1.5 max-h-32 overflow-y-auto text-xs text-slate-400">
                  {recentEvents.length === 0 ? (
                    <div className="text-slate-600 text-xs italic">Waiting for events...</div>
                  ) : (
                    recentEvents.slice(0, 5).map((ev, i) => (
                      <div key={i} className="flex items-center justify-between px-2 py-1 rounded bg-slate-900/60 border border-slate-800/60 text-[11px]">
                        <span className="font-mono text-sky-400">{ev.event}</span>
                        <span className="text-slate-400 truncate max-w-[100px]">{ev.topic}</span>
                      </div>
                    ))
                  )}
                </div>
              </div>
            </aside>

            {/* Content Area */}
            <main className="flex-1 overflow-y-auto p-8">
              {/* Tab 1: Models CRUD Table */}
              {activeTab === 'models' && selectedModel && (
                <div className="space-y-6">
                  {/* Model Header */}
                  <div className="flex items-center justify-between">
                    <div>
                      <div className="flex items-center gap-3">
                        <h2 className="text-2xl font-bold text-white">{selectedModel.display_name || selectedModel.name}</h2>
                        <span className="text-xs px-2.5 py-0.5 rounded-full bg-slate-800 border border-slate-700 text-slate-300 font-mono">
                          table: {selectedModel.table_name}
                        </span>
                        {selectedModel.has_vectors && (
                          <span className="text-xs px-2.5 py-0.5 rounded-full bg-purple-500/20 border border-purple-500/40 text-purple-300 flex items-center gap-1 font-medium">
                            <i data-lucide="cpu" className="w-3 h-3"></i> pgvector / sqlite-vec
                          </span>
                        )}
                      </div>
                      <p className="text-sm text-slate-400 mt-1">{selectedModel.description || `Manage ${selectedModel.display_name} records with auto-audit & validation.`}</p>
                    </div>

                    <div className="flex items-center gap-3">
                      <div className="relative">
                        <i data-lucide="search" className="w-4 h-4 absolute left-3 top-3 text-slate-500"></i>
                        <input
                          type="text"
                          value={searchQuery}
                          onChange={(e) => {
                            setSearchQuery(e.target.value);
                            setPage(1);
                          }}
                          placeholder="Search records..."
                          className="glass-input pl-9 pr-4 py-2 rounded-xl text-sm text-white placeholder-slate-500 w-64"
                        />
                      </div>
                      <button
                        onClick={() => {
                          setEditingRecord(null);
                          setFormData({});
                          setIsEditModalOpen(true);
                        }}
                        className="px-4 py-2 rounded-xl bg-gradient-to-r from-sky-500 to-blue-600 hover:from-sky-400 hover:to-blue-500 text-white font-semibold text-sm shadow-md shadow-sky-500/20 flex items-center gap-2 transition-all"
                      >
                        <i data-lucide="plus" className="w-4 h-4"></i>
                        Create {selectedModel.name}
                      </button>
                    </div>
                  </div>

                  {/* Data Table */}
                  <div className="glass-panel rounded-2xl overflow-hidden border border-slate-800 shadow-xl">
                    <div className="overflow-x-auto">
                      <table className="w-full text-left text-sm text-slate-300">
                        <thead className="bg-slate-900/80 text-xs uppercase font-semibold text-slate-400 border-b border-slate-800">
                          <tr>
                            {selectedModel.fields.filter((f) => f.list_display).map((f) => (
                              <th key={f.name} className="px-6 py-4">{f.display_name || f.name}</th>
                            ))}
                            <th className="px-6 py-4 text-right">Actions</th>
                          </tr>
                        </thead>
                        <tbody className="divide-y divide-slate-800/60">
                          {loading ? (
                            <tr>
                              <td colSpan="10" className="px-6 py-12 text-center text-slate-500">
                                <i data-lucide="loader" className="w-6 h-6 animate-spin mx-auto mb-2 text-sky-400"></i>
                                Loading records...
                              </td>
                            </tr>
                          ) : records.length === 0 ? (
                            <tr>
                              <td colSpan="10" className="px-6 py-12 text-center text-slate-500">
                                No records found in <code className="text-sky-400">{selectedModel.table_name}</code>
                              </td>
                            </tr>
                          ) : (
                            records.map((rec) => (
                              <tr key={rec.id} className="hover:bg-slate-800/40 transition-colors">
                                {selectedModel.fields.filter((f) => f.list_display).map((f) => {
                                  const val = rec[f.name];
                                  return (
                                    <td key={f.name} className="px-6 py-4">
                                      {typeof val === 'boolean' ? (
                                        <span className={`inline-flex items-center px-2 py-0.5 rounded text-xs font-medium ${val ? 'bg-emerald-500/20 text-emerald-300' : 'bg-slate-800 text-slate-400'}`}>
                                          {val ? 'TRUE' : 'FALSE'}
                                        </span>
                                      ) : f.name === 'status' ? (
                                        <span className="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold bg-sky-500/15 border border-sky-500/30 text-sky-300">
                                          {String(val)}
                                        </span>
                                      ) : typeof val === 'object' && val !== null ? (
                                        <code className="text-xs text-purple-300 font-mono">{Array.isArray(val) ? `[Vector (${val.length}d)]` : JSON.stringify(val)}</code>
                                      ) : (
                                        <span className="truncate max-w-xs block">{val !== undefined && val !== null ? String(val) : '—'}</span>
                                      )}
                                    </td>
                                  );
                                })}
                                <td className="px-6 py-4 text-right whitespace-nowrap">
                                  <button
                                    onClick={() => {
                                      setEditingRecord(rec);
                                      setFormData(rec);
                                      setIsEditModalOpen(true);
                                    }}
                                    className="p-1.5 text-slate-400 hover:text-sky-300 hover:bg-slate-800 rounded-lg transition-all mr-1"
                                    title="Edit"
                                  >
                                    <i data-lucide="edit-2" className="w-4 h-4"></i>
                                  </button>
                                  <button
                                    onClick={() => handleDeleteRecord(rec.id)}
                                    className="p-1.5 text-slate-400 hover:text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
                                    title="Delete"
                                  >
                                    <i data-lucide="trash-2" className="w-4 h-4"></i>
                                  </button>
                                </td>
                              </tr>
                            ))
                          )}
                        </tbody>
                      </table>
                    </div>

                    {/* Pagination */}
                    <div className="px-6 py-4 bg-slate-900/60 border-t border-slate-800 flex items-center justify-between text-xs text-slate-400">
                      <div>
                        Showing {records.length} of {totalRecords} total records
                      </div>
                      <div className="flex items-center gap-2">
                        <button
                          disabled={page <= 1}
                          onClick={() => setPage((p) => Math.max(1, p - 1))}
                          className="px-3 py-1.5 rounded-lg glass-card hover:bg-slate-800 disabled:opacity-40 disabled:pointer-events-none"
                        >
                          Previous
                        </button>
                        <span className="px-2 font-medium text-slate-300">Page {page}</span>
                        <button
                          disabled={page * 20 >= totalRecords}
                          onClick={() => setPage((p) => p + 1)}
                          className="px-3 py-1.5 rounded-lg glass-card hover:bg-slate-800 disabled:opacity-40 disabled:pointer-events-none"
                        >
                          Next
                        </button>
                      </div>
                    </div>
                  </div>
                </div>
              )}

              {/* Tab 2: AI Scaffolder */}
              {activeTab === 'ai-scaffolder' && (
                <div className="space-y-6 max-w-4xl">
                  <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                      <i data-lucide="sparkles" className="w-6 h-6 text-purple-400"></i>
                      Agentic AI Schema Scaffolder
                    </h2>
                    <p className="text-sm text-slate-400 mt-1">
                      Describe your model in plain English. The AI Tuner will generate fields, vector embeddings, validation, Rust code, and TypeScript definitions.
                    </p>
                  </div>

                  <div className="glass-panel p-6 rounded-2xl space-y-4 border border-purple-500/20 shadow-xl">
                    <div className="grid grid-cols-3 gap-4">
                      <div className="col-span-1">
                        <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Model Name</label>
                        <input
                          type="text"
                          value={scaffoldModelName}
                          onChange={(e) => setScaffoldModelName(e.target.value)}
                          placeholder="e.g. Article, Customer, Doc"
                          className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white placeholder-slate-500"
                        />
                      </div>
                      <div className="col-span-2">
                        <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Natural Language Prompt</label>
                        <input
                          type="text"
                          value={scaffoldPrompt}
                          onChange={(e) => setScaffoldPrompt(e.target.value)}
                          placeholder="A user with stripe billing, oauth, and vector embeddings for semantic search"
                          className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white placeholder-slate-500"
                        />
                      </div>
                    </div>

                    <div className="flex items-center justify-between pt-2">
                      <div className="flex gap-2">
                        <button
                          type="button"
                          onClick={() => {
                            setScaffoldModelName('KnowledgeDoc');
                            setScaffoldPrompt('A technical documentation article with title, markdown content, 1536d vector embedding, category, and published status');
                          }}
                          className="px-3 py-1 rounded-lg bg-slate-800 text-[11px] text-slate-400 hover:text-sky-300"
                        >
                          ⚡ RAG Doc Preset
                        </button>
                        <button
                          type="button"
                          onClick={() => {
                            setScaffoldModelName('Customer');
                            setScaffoldPrompt('Customer with email, stripe billing, discount requiring approval, and avatar image');
                          }}
                          className="px-3 py-1 rounded-lg bg-slate-800 text-[11px] text-slate-400 hover:text-sky-300"
                        >
                          ⚡ Stripe User Preset
                        </button>
                      </div>

                      <button
                        onClick={handleRunScaffolder}
                        disabled={scaffoldLoading}
                        className="px-5 py-2.5 rounded-xl bg-gradient-to-r from-purple-500 to-indigo-600 hover:from-purple-400 hover:to-indigo-500 text-white font-semibold text-sm shadow-lg shadow-purple-500/25 flex items-center gap-2 transition-all disabled:opacity-50"
                      >
                        {scaffoldLoading ? <i data-lucide="loader" className="w-4 h-4 animate-spin"></i> : <i data-lucide="zap" className="w-4 h-4"></i>}
                        Generate Schema & Code
                      </button>
                    </div>
                  </div>

                  {scaffoldResult && (
                    <div className="glass-panel p-6 rounded-2xl space-y-6 border border-slate-800">
                      <div className="flex items-center justify-between border-b border-slate-800 pb-4">
                        <div>
                          <h3 className="text-lg font-bold text-white">{scaffoldResult.schema.name}</h3>
                          <div className="flex gap-2 mt-1">
                            {scaffoldResult.detected_features.map((f, i) => (
                              <span key={i} className="text-xs px-2 py-0.5 rounded bg-purple-500/20 text-purple-300 font-medium">
                                ✓ {f}
                              </span>
                            ))}
                          </div>
                        </div>
                      </div>

                      <div className="space-y-4">
                        <div>
                          <h4 className="text-xs font-semibold uppercase tracking-wider text-sky-400 mb-2">Generated Rust Builder Code</h4>
                          <pre className="p-4 rounded-xl bg-slate-950 font-mono text-xs text-sky-300 overflow-x-auto border border-slate-800">
                            {scaffoldResult.rust_code}
                          </pre>
                        </div>

                        <div>
                          <h4 className="text-xs font-semibold uppercase tracking-wider text-indigo-400 mb-2">TypeScript Definition</h4>
                          <pre className="p-4 rounded-xl bg-slate-950 font-mono text-xs text-indigo-300 overflow-x-auto border border-slate-800">
                            {scaffoldResult.typescript_definition}
                          </pre>
                        </div>
                      </div>
                    </div>
                  )}
                </div>
              )}

              {/* Tab 3: Vector & RAG Studio */}
              {activeTab === 'vector-playground' && (
                <div className="space-y-6 max-w-4xl">
                  <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                      <i data-lucide="target" className="w-6 h-6 text-indigo-400"></i>
                      Vector & RAG Studio
                    </h2>
                    <p className="text-sm text-slate-400 mt-1">
                      Execute cosine similarity searches against embedded vectors across SQLite and PostgreSQL pgvector.
                    </p>
                  </div>

                  <div className="glass-panel p-6 rounded-2xl space-y-4 border border-indigo-500/20">
                    <div>
                      <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Target Model with Vector Field</label>
                      <select
                        value={selectedModel?.name}
                        onChange={(e) => {
                          const m = schemas.find((s) => s.name === e.target.value);
                          if (m) setSelectedModel(m);
                        }}
                        className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white"
                      >
                        {schemas.map((s) => (
                          <option key={s.name} value={s.name} className="bg-slate-900 text-white">
                            {s.name} {s.has_vectors ? '(Has Vector Embeddings)' : ''}
                          </option>
                        ))}
                      </select>
                    </div>

                    <div>
                      <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">Query Vector (JSON Array or Float CSV)</label>
                      <textarea
                        rows="3"
                        value={vectorQueryStr}
                        onChange={(e) => setVectorQueryStr(e.target.value)}
                        className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white font-mono placeholder-slate-500"
                        placeholder="[0.05, -0.12, 0.88, 0.42, -0.15]"
                      ></textarea>
                    </div>

                    <button
                      onClick={handleVectorSearch}
                      disabled={vectorLoading}
                      className="px-5 py-2.5 rounded-xl bg-gradient-to-r from-indigo-500 to-blue-600 hover:from-indigo-400 text-white font-semibold text-sm shadow-lg shadow-indigo-500/25 flex items-center gap-2 transition-all disabled:opacity-50"
                    >
                      {vectorLoading ? <i data-lucide="loader" className="w-4 h-4 animate-spin"></i> : <i data-lucide="search" className="w-4 h-4"></i>}
                      Search Top-K Neighbors
                    </button>
                  </div>

                  {vectorResults.length > 0 && (
                    <div className="space-y-3">
                      <h3 className="text-sm font-semibold text-slate-300">Ranked Similarity Matches</h3>
                      {vectorResults.map((match, i) => (
                        <div key={match.id} className="glass-card p-4 rounded-xl flex items-center justify-between border border-slate-800">
                          <div>
                            <div className="flex items-center gap-2">
                              <span className="font-bold text-white">Record #{match.id}</span>
                              <span className="text-xs px-2 py-0.5 rounded bg-indigo-500/20 text-indigo-300 font-mono">
                                Cosine Score: {(match.score * 100).toFixed(2)}%
                              </span>
                            </div>
                            <pre className="mt-2 text-xs font-mono text-slate-400 max-w-xl truncate">
                              {JSON.stringify(match.record)}
                            </pre>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}

              {/* Tab 4: AI Tuner */}
              {activeTab === 'ai-tuner' && (
                <div className="space-y-6 max-w-4xl">
                  <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                      <i data-lucide="gauge" className="w-6 h-6 text-amber-400"></i>
                      AI Tuner & Database Advisor
                    </h2>
                    <p className="text-sm text-slate-400 mt-1">Real-time query telemetry, latency percentiles, and automated DDL index recommendation.</p>
                  </div>

                  {tunerReport && (
                    <div className="space-y-6">
                      {/* KPI Cards */}
                      <div className="grid grid-cols-4 gap-4">
                        <div className="glass-card p-4 rounded-xl border border-slate-800">
                          <div className="text-xs font-semibold text-slate-400 uppercase">Queries Analyzed</div>
                          <div className="text-2xl font-bold text-white mt-1">{tunerReport.total_queries_analyzed}</div>
                        </div>
                        <div className="glass-card p-4 rounded-xl border border-slate-800">
                          <div className="text-xs font-semibold text-slate-400 uppercase">p50 Latency</div>
                          <div className="text-2xl font-bold text-emerald-400 mt-1">{tunerReport.p50_latency_ms} ms</div>
                        </div>
                        <div className="glass-card p-4 rounded-xl border border-slate-800">
                          <div className="text-xs font-semibold text-slate-400 uppercase">p99 Latency</div>
                          <div className="text-2xl font-bold text-sky-400 mt-1">{tunerReport.p99_latency_ms} ms</div>
                        </div>
                        <div className="glass-card p-4 rounded-xl border border-slate-800">
                          <div className="text-xs font-semibold text-slate-400 uppercase">Throughput</div>
                          <div className="text-2xl font-bold text-indigo-400 mt-1">{tunerReport.qps} QPS</div>
                        </div>
                      </div>

                      <div className="glass-panel p-6 rounded-2xl border border-slate-800 space-y-4">
                        <h3 className="font-bold text-white text-base">Workload Diagnosis</h3>
                        <p className="text-sm text-slate-300">{tunerReport.workload_summary}</p>
                      </div>

                      <div className="space-y-4">
                        <h3 className="font-bold text-white text-base">Recommended Indexes ({tunerReport.recommendations.length})</h3>
                        {tunerReport.recommendations.map((rec) => (
                          <div key={rec.id} className="glass-card p-5 rounded-2xl border border-slate-800 flex items-center justify-between">
                            <div>
                              <div className="flex items-center gap-2">
                                <span className="font-bold text-white">{rec.table_name}.{rec.column}</span>
                                <span className="text-xs px-2 py-0.5 rounded bg-emerald-500/20 text-emerald-300 font-medium">{rec.estimated_speedup}</span>
                              </div>
                              <p className="text-xs text-slate-400 mt-1">{rec.reason}</p>
                              <code className="text-[11px] font-mono text-sky-300 block mt-2">{rec.ddl}</code>
                            </div>
                            <button
                              onClick={async () => {
                                const res = await fetch(`/api/ai/indexes/apply?table=${rec.table_name}&column=${rec.column}`, { method: 'POST' });
                                if (res.ok) {
                                  notify(`Applied index on ${rec.table_name}.${rec.column}`);
                                  loadTunerReport();
                                }
                              }}
                              className="px-4 py-2 rounded-xl bg-sky-500/20 hover:bg-sky-500/30 text-sky-300 font-semibold text-xs border border-sky-500/40 transition-all"
                            >
                              1-Click Apply
                            </button>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              )}

              {/* Tab 5: Approvals */}
              {activeTab === 'approvals' && (
                <div className="space-y-6 max-w-4xl">
                  <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                      <i data-lucide="shield-check" className="w-6 h-6 text-emerald-400"></i>
                      Two-Person Rule Approval Queue
                    </h2>
                    <p className="text-sm text-slate-400 mt-1">Review sensitive mutations scored by the AI Decision Engine.</p>
                  </div>

                  {approvals.length === 0 ? (
                    <div className="glass-panel p-12 text-center rounded-2xl text-slate-500">
                      No pending approvals in queue. All sensitive changes are verified.
                    </div>
                  ) : (
                    <div className="space-y-4">
                      {approvals.map((app) => (
                        <div key={app.id} className="glass-card p-5 rounded-2xl border border-slate-800 flex items-center justify-between">
                          <div>
                            <div className="flex items-center gap-3">
                              <span className="font-bold text-white">{app.model_name} #{app.record_id}</span>
                              <span className="text-xs font-mono text-slate-400">field: {app.field_name}</span>
                              <span
                                className="text-xs px-2.5 py-0.5 rounded-full font-bold uppercase tracking-wider"
                                style={{ backgroundColor: `${app.ai_risk?.risk_level === 'Critical' ? '#ef4444' : '#f59e0b'}20`, color: app.ai_risk?.risk_level === 'Critical' ? '#ef4444' : '#f59e0b' }}
                              >
                                {app.ai_risk?.risk_level || 'REVIEW'}
                              </span>
                            </div>
                            <div className="mt-2 text-xs text-slate-300 flex items-center gap-2">
                              <span className="line-through text-rose-400">{app.old_value || 'null'}</span>
                              <span>➔</span>
                              <span className="font-bold text-emerald-400">{app.new_value}</span>
                            </div>
                            <p className="text-xs text-slate-400 mt-2 italic">{app.ai_risk?.recommendation}</p>
                          </div>

                          <div className="flex items-center gap-2">
                            <button
                              onClick={async () => {
                                const res = await fetch(`/api/d/approvals/${app.id}/approve`, { method: 'POST' });
                                if (res.ok) {
                                  notify('Change approved & applied');
                                  loadApprovals();
                                }
                              }}
                              className="px-4 py-2 rounded-xl bg-emerald-500 hover:bg-emerald-400 text-white font-semibold text-xs shadow-md shadow-emerald-500/20"
                            >
                              Approve
                            </button>
                            <button
                              onClick={async () => {
                                const res = await fetch(`/api/d/approvals/${app.id}/reject`, { method: 'POST' });
                                if (res.ok) {
                                  notify('Change rejected');
                                  loadApprovals();
                                }
                              }}
                              className="px-4 py-2 rounded-xl bg-rose-500/20 hover:bg-rose-500/30 text-rose-300 font-semibold text-xs border border-rose-500/30"
                            >
                              Reject
                            </button>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              )}

              {/* Tab 6: Audit & Time-Travel Rollback */}
              {activeTab === 'audit' && (
                <div className="space-y-6 max-w-4xl">
                  <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                      <i data-lucide="history" className="w-6 h-6 text-sky-400"></i>
                      Audit Trail & 1-Click Time-Travel Rollback
                    </h2>
                    <p className="text-sm text-slate-400 mt-1">Immutable mutation logs capturing diffs and full snapshots for instant restoration.</p>
                  </div>

                  <div className="glass-panel rounded-2xl overflow-hidden border border-slate-800">
                    <table className="w-full text-left text-sm text-slate-300">
                      <thead className="bg-slate-900/80 text-xs uppercase font-semibold text-slate-400 border-b border-slate-800">
                        <tr>
                          <th className="px-6 py-4">Action</th>
                          <th className="px-6 py-4">Model & ID</th>
                          <th className="px-6 py-4">User</th>
                          <th className="px-6 py-4">Timestamp</th>
                          <th className="px-6 py-4 text-right">Time-Travel</th>
                        </tr>
                      </thead>
                      <tbody className="divide-y divide-slate-800/60">
                        {auditLogs.map((log) => (
                          <tr key={log.id} className="hover:bg-slate-800/30">
                            <td className="px-6 py-4 font-mono font-bold text-xs text-sky-400">{log.action}</td>
                            <td className="px-6 py-4">{log.model_name} #{log.record_id}</td>
                            <td className="px-6 py-4 text-slate-400">{log.username || 'system'}</td>
                            <td className="px-6 py-4 text-slate-500 text-xs">{new Date(log.created_at).toLocaleString()}</td>
                            <td className="px-6 py-4 text-right">
                              <button
                                onClick={() => handleRollback(log.id)}
                                className="px-3 py-1 rounded-lg bg-sky-500/15 hover:bg-sky-500/25 border border-sky-500/30 text-sky-300 text-xs font-medium"
                              >
                                Restore Snapshot
                              </button>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                </div>
              )}

              {/* Tab 7: Health & Self-Healing Telemetry */}
              {activeTab === 'health' && (
                <div className="space-y-6 max-w-4xl">
                  <div>
                    <h2 className="text-2xl font-bold text-white flex items-center gap-2">
                      <i data-lucide="activity" className="w-6 h-6 text-rose-400"></i>
                      Self-Healing Health & Watchdog Telemetry
                    </h2>
                    <p className="text-sm text-slate-400 mt-1">Live monitoring of system uptime, database connection watchdog, and circuit breakers.</p>
                  </div>

                  {healthData && (
                    <div className="grid grid-cols-3 gap-6">
                      <div className="glass-panel p-6 rounded-2xl border border-slate-800">
                        <div className="text-xs font-semibold text-slate-400 uppercase">System Status</div>
                        <div className="text-2xl font-bold text-emerald-400 mt-2">{healthData.status}</div>
                        <p className="text-xs text-slate-500 mt-1">Uptime: {healthData.uptime_seconds}s</p>
                      </div>
                      <div className="glass-panel p-6 rounded-2xl border border-slate-800">
                        <div className="text-xs font-semibold text-slate-400 uppercase">Database Driver</div>
                        <div className="text-lg font-bold text-white mt-2">{healthData.system?.database_driver}</div>
                        <p className="text-xs text-purple-400 mt-1">{healthData.system?.vector_engine}</p>
                      </div>
                      <div className="glass-panel p-6 rounded-2xl border border-slate-800">
                        <div className="text-xs font-semibold text-slate-400 uppercase">Panic Recoveries</div>
                        <div className="text-2xl font-bold text-sky-400 mt-2">{healthData.self_healing?.panic_recoveries_count}</div>
                        <p className="text-xs text-slate-500 mt-1">100% Zero-Crash Server Resiliency</p>
                      </div>
                    </div>
                  )}
                </div>
              )}
            </main>
          </div>

          {/* Record Create / Edit Modal */}
          {isEditModalOpen && selectedModel && (
            <div className="fixed inset-0 z-50 bg-slate-950/80 backdrop-blur-sm flex items-center justify-center p-4">
              <div className="glass-panel w-full max-w-2xl p-6 rounded-2xl shadow-2xl border border-slate-800 max-h-[90vh] overflow-y-auto">
                <div className="flex items-center justify-between border-b border-slate-800 pb-4 mb-6">
                  <h3 className="text-lg font-bold text-white">
                    {editingRecord ? `Edit ${selectedModel.name} #${editingRecord.id}` : `Create New ${selectedModel.name}`}
                  </h3>
                  <button onClick={() => setIsEditModalOpen(false)} className="text-slate-400 hover:text-white">&times;</button>
                </div>

                <form onSubmit={handleSaveRecord} className="space-y-4">
                  {selectedModel.fields
                    .filter((f) => f.name !== 'id' && f.name !== 'created_at' && f.name !== 'updated_at')
                    .map((f) => (
                      <div key={f.name}>
                        <label className="block text-xs font-semibold uppercase tracking-wider text-slate-400 mb-1">
                          {f.display_name || f.name} {f.required && <span className="text-rose-400">*</span>}
                        </label>
                        {f.field_type?.kind === 'Enum' ? (
                          <select
                            value={formData[f.name] || ''}
                            onChange={(e) => setFormData({ ...formData, [f.name]: e.target.value })}
                            className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white"
                          >
                            <option value="">Select option...</option>
                            {f.field_type.config?.choices?.map((c) => (
                              <option key={c} value={c} className="bg-slate-900">{c}</option>
                            ))}
                          </select>
                        ) : f.field_type?.kind === 'Boolean' ? (
                          <input
                            type="checkbox"
                            checked={!!formData[f.name]}
                            onChange={(e) => setFormData({ ...formData, [f.name]: e.target.checked })}
                            className="w-5 h-5 rounded border-slate-700 bg-slate-900 text-sky-500"
                          />
                        ) : f.field_type?.kind === 'Html' || f.field_type?.kind === 'Markdown' ? (
                          <textarea
                            rows="4"
                            value={formData[f.name] || ''}
                            onChange={(e) => setFormData({ ...formData, [f.name]: e.target.value })}
                            className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white"
                          ></textarea>
                        ) : (
                          <input
                            type={f.field_type?.kind === 'Integer' || f.field_type?.kind === 'Float' ? 'number' : 'text'}
                            value={formData[f.name] !== undefined ? formData[f.name] : ''}
                            onChange={(e) => {
                              const val = f.field_type?.kind === 'Integer' ? parseInt(e.target.value) || 0 : e.target.value;
                              setFormData({ ...formData, [f.name]: val });
                            }}
                            className="glass-input w-full px-4 py-2.5 rounded-xl text-sm text-white"
                          />
                        )}
                      </div>
                    ))}

                  <div className="flex justify-end gap-3 pt-6 border-t border-slate-800">
                    <button
                      type="button"
                      onClick={() => setIsEditModalOpen(false)}
                      className="px-4 py-2 rounded-xl glass-card hover:bg-slate-800 text-slate-300 text-sm font-medium"
                    >
                      Cancel
                    </button>
                    <button
                      type="submit"
                      className="px-5 py-2 rounded-xl bg-gradient-to-r from-sky-500 to-blue-600 hover:from-sky-400 text-white text-sm font-semibold shadow-md shadow-sky-500/20"
                    >
                      Save Record
                    </button>
                  </div>
                </form>
              </div>
            </div>
          )}

          {toast && <Toast {...toast} onClose={() => setToast(null)} />}
        </div>
      );
    }

    ReactDOM.render(<VellaAdminApp />, document.getElementById('root'));
  </script>
</body>
</html>"#;

    raw_html.replace("__SITE_NAME__", site_name)
}
