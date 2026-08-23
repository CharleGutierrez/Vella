/// Interactive multi-framework (React, Vue 3, Angular) Todo Showcase HTML page for Vella
pub fn todo_showcase_html(site_name: &str) -> String {
    let raw_html = r#"<!DOCTYPE html>
<html lang="en" class="dark">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>__SITE_NAME__ - Multi-Framework Todo Hub (React, Vue, Angular)</title>
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
            },
            react: {
              400: '#38bdf8',
              500: '#0ea5e9',
            },
            vue: {
              400: '#4ade80',
              500: '#22c55e',
            },
            angular: {
              400: '#fb7185',
              500: '#f43f5e',
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
    body { background: radial-gradient(circle at 50% 0%, #1e1b4b 0%, #0b0f19 75%); min-height: 100vh; color: #f1f5f9; }
    .glass-panel { background: rgba(15, 23, 42, 0.75); backdrop-filter: blur(16px); border: 1px solid rgba(255, 255, 255, 0.08); }
    .glass-card { background: rgba(30, 41, 59, 0.5); backdrop-filter: blur(12px); border: 1px solid rgba(255, 255, 255, 0.06); }
    .glass-input { background: rgba(15, 23, 42, 0.6); border: 1px solid rgba(255, 255, 255, 0.12); }
    .glass-input:focus { border-color: #38bdf8; outline: none; box-shadow: 0 0 0 2px rgba(56, 189, 248, 0.2); }
  </style>
</head>
<body class="font-sans antialiased text-slate-100">
  <div id="root"></div>

  <script type="text/babel">
    const { useState, useEffect } = React;

    function TodoShowcase() {
      const [todos, setTodos] = useState([]);
      const [loading, setLoading] = useState(true);
      const [framework, setFramework] = useState('react'); // react, vue, angular
      const [newTitle, setNewTitle] = useState('');
      const [newCategory, setNewCategory] = useState('Frontend');
      const [newPriority, setNewPriority] = useState('High');

      const fetchTodos = async () => {
        setLoading(true);
        try {
          const res = await fetch('/api/d/todos?$limit=50&$order=-id');
          if (res.ok) {
            const data = await res.json();
            setTodos(data.data || []);
          }
        } catch (e) {
          console.error(e);
        } finally {
          setLoading(false);
        }
      };

      useEffect(() => {
        fetchTodos();
      }, []);

      useEffect(() => {
        if (window.lucide) window.lucide.createIcons();
      });

      const handleAddTodo = async (e) => {
        e.preventDefault();
        if (!newTitle.trim()) return;

        try {
          const res = await fetch('/api/d/todos', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({
              title: newTitle,
              category: newCategory,
              priority: newPriority,
              is_completed: false,
              progress: 0,
            }),
          });
          if (res.ok) {
            setNewTitle('');
            fetchTodos();
          }
        } catch (e) {}
      };

      const toggleComplete = async (todo) => {
        const updated = !todo.is_completed;
        await fetch(`/api/d/todos/${todo.id}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            is_completed: updated,
            progress: updated ? 100 : 50,
          }),
        });
        fetchTodos();
      };

      const deleteTodo = async (id) => {
        await fetch(`/api/d/todos/${id}`, { method: 'DELETE' });
        fetchTodos();
      };

      return (
        <div className="min-h-screen flex flex-col p-6 max-w-4xl mx-auto space-y-6">
          <header className="flex items-center justify-between pb-4 border-b border-slate-800">
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-gradient-to-br from-sky-400 to-indigo-600 flex items-center justify-center font-bold text-white shadow-lg">
                V
              </div>
              <div>
                <h1 className="text-xl font-bold text-white">Vella Multi-Framework Showcase</h1>
                <p className="text-xs text-slate-400">Universal REST & WebSocket backend powering React, Vue 3, and Angular</p>
              </div>
            </div>
            <a href="/" className="px-3 py-1.5 rounded-lg glass-card hover:bg-slate-800 text-xs font-semibold text-sky-400 flex items-center gap-1">
              <i data-lucide="layout-dashboard" className="w-3.5 h-3.5"></i>
              Open Admin CMS
            </a>
          </header>

          {/* Framework Switcher Tab */}
          <div className="flex gap-3">
            <button
              onClick={() => setFramework('react')}
              className={`flex-1 py-3 px-4 rounded-xl border flex items-center justify-center gap-2 text-sm font-semibold transition-all ${
                framework === 'react'
                  ? 'bg-sky-500/20 border-sky-500/40 text-sky-300 shadow-lg shadow-sky-500/10'
                  : 'glass-card border-slate-800 text-slate-400 hover:text-slate-200'
              }`}
            >
              <span>⚛️ React 18 SDK</span>
            </button>
            <button
              onClick={() => setFramework('vue')}
              className={`flex-1 py-3 px-4 rounded-xl border flex items-center justify-center gap-2 text-sm font-semibold transition-all ${
                framework === 'vue'
                  ? 'bg-emerald-500/20 border-emerald-500/40 text-emerald-300 shadow-lg shadow-emerald-500/10'
                  : 'glass-card border-slate-800 text-slate-400 hover:text-slate-200'
              }`}
            >
              <span>💚 Vue 3 SDK</span>
            </button>
            <button
              onClick={() => setFramework('angular')}
              className={`flex-1 py-3 px-4 rounded-xl border flex items-center justify-center gap-2 text-sm font-semibold transition-all ${
                framework === 'angular'
                  ? 'bg-rose-500/20 border-rose-500/40 text-rose-300 shadow-lg shadow-rose-500/10'
                  : 'glass-card border-slate-800 text-slate-400 hover:text-slate-200'
              }`}
            >
              <span>🅰️ Angular 17+ Signals</span>
            </button>
          </div>

          {/* Add Todo Form */}
          <form onSubmit={handleAddTodo} className="glass-panel p-5 rounded-2xl flex gap-3 border border-slate-800">
            <input
              type="text"
              value={newTitle}
              onChange={(e) => setNewTitle(e.target.value)}
              placeholder="What needs to be done?"
              className="glass-input flex-1 px-4 py-2.5 rounded-xl text-sm text-white placeholder-slate-500"
            />
            <select
              value={newCategory}
              onChange={(e) => setNewCategory(e.target.value)}
              className="glass-input px-3 py-2.5 rounded-xl text-xs text-white"
            >
              <option value="Frontend">Frontend</option>
              <option value="Backend">Backend</option>
              <option value="AI / RAG">AI / RAG</option>
              <option value="DevOps">DevOps</option>
            </select>
            <select
              value={newPriority}
              onChange={(e) => setNewPriority(e.target.value)}
              className="glass-input px-3 py-2.5 rounded-xl text-xs text-white"
            >
              <option value="Low">Low</option>
              <option value="Medium">Medium</option>
              <option value="High">High</option>
              <option value="Critical">Critical</option>
            </select>
            <button
              type="submit"
              className="px-5 py-2.5 rounded-xl bg-gradient-to-r from-sky-500 to-blue-600 hover:from-sky-400 text-white font-semibold text-sm shadow-md"
            >
              Add Task
            </button>
          </form>

          {/* Task List */}
          <div className="space-y-3">
            {loading ? (
              <div className="glass-panel p-8 text-center text-slate-500">Loading tasks...</div>
            ) : todos.length === 0 ? (
              <div className="glass-panel p-8 text-center text-slate-500">No tasks created yet.</div>
            ) : (
              todos.map((todo) => (
                <div
                  key={todo.id}
                  className={`glass-card p-4 rounded-xl border flex items-center justify-between transition-all ${
                    todo.is_completed ? 'border-emerald-500/20 opacity-70' : 'border-slate-800'
                  }`}
                >
                  <div className="flex items-center gap-3">
                    <button
                      onClick={() => toggleComplete(todo)}
                      className={`w-5 h-5 rounded-lg border flex items-center justify-center transition-all ${
                        todo.is_completed ? 'bg-emerald-500 border-emerald-500 text-slate-950' : 'border-slate-600 hover:border-sky-400'
                      }`}
                    >
                      {todo.is_completed && <i data-lucide="check" className="w-3.5 h-3.5"></i>}
                    </button>
                    <div>
                      <span className={`text-sm font-medium ${todo.is_completed ? 'line-through text-slate-500' : 'text-white'}`}>
                        {todo.title}
                      </span>
                      <div className="flex items-center gap-2 mt-1">
                        <span className="text-[10px] px-2 py-0.5 rounded bg-slate-800 text-slate-400">{todo.category}</span>
                        <span className={`text-[10px] px-2 py-0.5 rounded ${todo.priority === 'Critical' ? 'bg-rose-500/20 text-rose-300' : 'bg-sky-500/20 text-sky-300'}`}>
                          {todo.priority}
                        </span>
                      </div>
                    </div>
                  </div>

                  <button
                    onClick={() => deleteTodo(todo.id)}
                    className="p-1.5 text-slate-500 hover:text-rose-400 hover:bg-rose-500/10 rounded-lg transition-all"
                  >
                    <i data-lucide="trash-2" className="w-4 h-4"></i>
                  </button>
                </div>
              ))
            )}
          </div>
        </div>
      );
    }

    ReactDOM.render(<TodoShowcase />, document.getElementById('root'));
  </script>
</body>
</html>"#;

    raw_html.replace("__SITE_NAME__", site_name)
}
