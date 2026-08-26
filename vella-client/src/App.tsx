import React, { useEffect, useRef, useState } from 'react';
import * as Y from 'yjs';
import { WebsocketProvider } from 'y-websocket';
import { Pencil, Users, Circle, Loader2 } from 'lucide-react';

function App() {
  const [status, setStatus] = useState<string>('connecting');
  const [users, setUsers] = useState<number>(1);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const ydocRef = useRef<Y.Doc | null>(null);
  const providerRef = useRef<WebsocketProvider | null>(null);
  const isUpdatingRef = useRef<boolean>(false);

  useEffect(() => {
    // Initialize Yjs Document
    const ydoc = new Y.Doc();
    ydocRef.current = ydoc;

    // Connect to websocket provider
    // Using serverUrl as the base and roomname as 'ws' to form ws://localhost:8080/api/realtime/ws
    const provider = new WebsocketProvider(
      'ws://localhost:8080/api/realtime',
      'ws',
      ydoc
    );
    providerRef.current = provider;

    const ytext = ydoc.getText('content');

    provider.on('status', (event: { status: string }) => {
      setStatus(event.status);
    });

    provider.awareness.on('change', () => {
      setUsers(provider.awareness.getStates().size);
    });

    ytext.observe(() => {
      if (textareaRef.current && !isUpdatingRef.current) {
        const cursor = textareaRef.current.selectionStart;
        textareaRef.current.value = ytext.toString();
        // Restore cursor
        textareaRef.current.setSelectionRange(cursor, cursor);
      }
    });

    return () => {
      provider.destroy();
      ydoc.destroy();
    };
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    if (!ydocRef.current) return;

    const ytext = ydocRef.current.getText('content');
    const newText = e.target.value;
    const oldText = ytext.toString();

    // Prevent observe callback from overriding while we are typing
    isUpdatingRef.current = true;

    // Simple diff algorithm to preserve cursor positions better
    let prefix = 0;
    while (prefix < oldText.length && prefix < newText.length && oldText[prefix] === newText[prefix]) {
      prefix++;
    }

    let suffix = 0;
    while (
      suffix < oldText.length - prefix && 
      suffix < newText.length - prefix && 
      oldText[oldText.length - 1 - suffix] === newText[newText.length - 1 - suffix]
    ) {
      suffix++;
    }

    const lengthToRemove = oldText.length - prefix - suffix;
    const textToInsert = newText.slice(prefix, newText.length - suffix);

    ydocRef.current.transact(() => {
      if (lengthToRemove > 0) {
        ytext.delete(prefix, lengthToRemove);
      }
      if (textToInsert.length > 0) {
        ytext.insert(prefix, textToInsert);
      }
    });

    isUpdatingRef.current = false;
  };

  return (
    <div className="min-h-screen bg-slate-50 text-slate-900 flex flex-col font-sans">
      <header className="bg-white border-b border-slate-200 px-6 py-4 flex items-center justify-between shadow-sm">
        <div className="flex items-center gap-2">
          <div className="bg-indigo-600 p-2 rounded-lg">
            <Pencil className="w-5 h-5 text-white" />
          </div>
          <h1 className="text-xl font-semibold text-slate-800">Vella Collab</h1>
        </div>
        
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2 text-sm text-slate-600 font-medium">
            <Users className="w-4 h-4" />
            <span>{users} {users === 1 ? 'user' : 'users'} online</span>
          </div>
          <div className="flex items-center gap-2 text-sm font-medium">
            {status === 'connected' ? (
              <span className="flex items-center gap-1.5 text-emerald-600 bg-emerald-50 px-2.5 py-1 rounded-full">
                <Circle className="w-3 h-3 fill-emerald-500" />
                Connected
              </span>
            ) : (
              <span className="flex items-center gap-1.5 text-amber-600 bg-amber-50 px-2.5 py-1 rounded-full">
                <Loader2 className="w-3 h-3 animate-spin" />
                Connecting...
              </span>
            )}
          </div>
        </div>
      </header>

      <main className="flex-1 max-w-5xl w-full mx-auto p-6 flex flex-col">
        <div className="bg-white flex-1 rounded-xl shadow-sm border border-slate-200 overflow-hidden flex flex-col focus-within:ring-2 focus-within:ring-indigo-500 focus-within:border-indigo-500 transition-shadow">
          <textarea
            ref={textareaRef}
            onChange={handleChange}
            placeholder="Start typing to collaborate in real-time..."
            className="flex-1 w-full p-6 resize-none outline-none text-lg text-slate-700 leading-relaxed bg-transparent"
            spellCheck="false"
          />
        </div>
        <p className="text-center text-slate-400 text-sm mt-4">
          Changes are synced automatically with all connected users.
        </p>
      </main>
    </div>
  );
}

export default App;
