/// Auto-generated TypeScript / React SDK for frontend developers connecting to Vella dAPI.
pub fn generate_react_sdk(base_url: &str) -> String {
    format!(
        r#"/**
 * ⚡ Vella React Client SDK & Realtime Hooks
 * Auto-generated React Hooks, WebSocket Sync & API Client for Vella backend.
 * Compatible with React 18+, Next.js 14+, Remix, Vite, and React Native.
 */

import React, {{ createContext, useContext, useState, useEffect, useCallback, useRef }} from 'react';

export interface VellaUser {{
  id: number;
  username: string;
  email: string;
  role: 'Admin' | 'Manager' | 'Editor' | 'Viewer';
  is_active: boolean;
  oauth_provider?: string | null;
}}

export interface QueryOptions {{
  limit?: number;
  offset?: number;
  order?: string;
  search?: string;
  filters?: Record<string, string | number | boolean>;
}}

export interface QueryResult<T = any> {{
  data: T[];
  total: number;
  limit: number;
  offset: number;
  isLoading: boolean;
  error: Error | null;
  refetch: () => Promise<void>;
}}

export interface VectorSearchOptions {{
  vector_field?: string;
  top_k?: number;
  metric?: 'Cosine' | 'Euclidean' | 'DotProduct';
}}

export interface VectorSearchResult<T = any> {{
  id: number;
  score: number;
  record: T;
}}

export class VellaClient {{
  private baseUrl: string;
  private token: string | null = null;
  private ws: WebSocket | null = null;
  private listeners: Map<string, Set<(event: any) => void>> = new Map();

  constructor(baseUrl: string = '{base_url}') {{
    this.baseUrl = baseUrl.replace(/\/$/, '');
  }}

  setToken(token: string | null) {{
    this.token = token;
  }}

  async request<T = any>(path: string, options: RequestInit = {{}}): Promise<T> {{
    const headers: Record<string, string> = {{
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string> || {{}}),
    }};

    if (this.token) {{
      headers['Authorization'] = `Bearer ${{this.token}}`;
    }}

    const res = await fetch(`${{this.baseUrl}}${{path}}`, {{
      ...options,
      headers,
      credentials: 'include',
    }});

    if (!res.ok) {{
      const errBody = await res.json().catch(() => ({{ message: res.statusText }}));
      throw new Error(errBody.error?.message || errBody.message || `Request failed with status ${{res.status}}`);
    }}

    return res.json();
  }}

  // Auth methods
  async login(username: string, password: string) {{
    const res = await this.request('/api/auth/login', {{
      method: 'POST',
      body: JSON.stringify({{ username, password }}),
    }});
    if (res.session?.token) {{
      this.setToken(res.session.token);
    }}
    return res;
  }}

  async logout() {{
    await this.request('/api/auth/logout', {{ method: 'POST' }});
    this.setToken(null);
  }}

  async getMe(): Promise<{{ success: boolean; user: VellaUser }}> {{
    return this.request('/api/auth/me');
  }}

  async requestMagicLink(email: string) {{
    return this.request('/api/auth/magic-link/request', {{
      method: 'POST',
      body: JSON.stringify({{ email }}),
    }});
  }}

  // Model CRUD methods
  async list<T = any>(model: string, options: QueryOptions = {{}}): Promise<{{ success: boolean; total: number; limit: number; offset: number; data: T[] }}> {{
    const params = new URLSearchParams();
    if (options.limit) params.set('$limit', options.limit.toString());
    if (options.offset) params.set('$offset', options.offset.toString());
    if (options.order) params.set('$order', options.order);
    if (options.search) params.set('$search', options.search);

    if (options.filters) {{
      for (const [k, v] of Object.entries(options.filters)) {{
        params.set(k, String(v));
      }}
    }}

    const query = params.toString() ? `?${{params.toString()}}` : '';
    return this.request(`/api/d/${{model.toLowerCase()}}${{query}}`);
  }}

  async get<T = any>(model: string, id: number): Promise<{{ success: boolean; data: T }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`);
  }}

  async create<T = any>(model: string, payload: Partial<T>): Promise<{{ success: boolean; data: T }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}`, {{
      method: 'POST',
      body: JSON.stringify(payload),
    }});
  }}

  async update<T = any>(model: string, id: number, payload: Partial<T>): Promise<{{ success: boolean; data: T }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{
      method: 'PUT',
      body: JSON.stringify(payload),
    }});
  }}

  async delete(model: string, id: number): Promise<{{ success: boolean; message: string }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{
      method: 'DELETE',
    }});
  }}

  // Native Vector Similarity Search
  async searchVector<T = any>(model: string, queryVector: number[], options: VectorSearchOptions = {{}}): Promise<{{ success: boolean; results: VectorSearchResult<T>[] }}> {{
    return this.request(`/api/d/${{model.toLowerCase()}}/search-vector`, {{
      method: 'POST',
      body: JSON.stringify({{
        model,
        query_vector: queryVector,
        vector_field: options.vector_field || 'embedding',
        top_k: options.top_k || 10,
        metric: options.metric || 'Cosine',
      }}),
    }});
  }}

  // RAG & Semantic Cache AI Query
  async ragQuery<T = any>(model: string, query: string, queryVector: number[], topK: number = 5) {{
    return this.request('/api/ai/rag/query', {{
      method: 'POST',
      body: JSON.stringify({{
        model_name: model,
        query,
        query_vector: queryVector,
        top_k: topK,
      }}),
    }});
  }}

  // Realtime WebSocket Subscription
  subscribe(topic: string, callback: (event: any) => void): () => void {{
    if (!this.ws || this.ws.readyState === WebSocket.CLOSED) {{
      const wsProtocol = this.baseUrl.startsWith('https') ? 'wss:' : 'ws:';
      const host = this.baseUrl.replace(/^https?:\/\//, '');
      this.ws = new WebSocket(`${{wsProtocol}}//${{host}}/api/realtime/ws`);
      this.ws.onmessage = (e) => {{
        try {{
          const msg = JSON.parse(e.data);
          const listeners = this.listeners.get(msg.topic);
          if (listeners) {{
            listeners.forEach((cb) => cb(msg));
          }}
          const allListeners = this.listeners.get('*');
          if (allListeners) {{
            allListeners.forEach((cb) => cb(msg));
          }}
        }} catch (err) {{}}
      }};
    }}

    if (!this.listeners.has(topic)) {{
      this.listeners.set(topic, new Set());
    }}
    this.listeners.get(topic)!.add(callback);

    return () => {{
      const set = this.listeners.get(topic);
      if (set) {{
        set.delete(callback);
      }}
    }};
  }}
}}

// React Context & Hooks
const VellaContext = createContext<VellaClient | null>(null);

export function VellaProvider({{ client, children }}: {{ client: VellaClient; children: React.ReactNode }}) {{
  return <VellaContext.Provider value={{client}} >{{children}}</VellaContext.Provider>;
}}

export function useVella(): VellaClient {{
  const client = useContext(VellaContext);
  if (!client) {{
    throw new Error('useVella must be used within a VellaProvider');
  }}
  return client;
}}

export function useVellaQuery<T = any>(model: string, options: QueryOptions = {{}}): QueryResult<T> {{
  const client = useVella();
  const [data, setData] = useState<T[]>([]);
  const [total, setTotal] = useState(0);
  const [limit, setLimit] = useState(options.limit || 50);
  const [offset, setOffset] = useState(options.offset || 0);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  const fetchRecords = useCallback(async () => {{
    setIsLoading(true);
    setError(null);
    try {{
      const res = await client.list<T>(model, options);
      setData(res.data);
      setTotal(res.total);
      setLimit(res.limit);
      setOffset(res.offset);
    }} catch (err: any) {{
      setError(err);
    }} finally {{
      setIsLoading(false);
    }}
  }}, [client, model, JSON.stringify(options)]);

  useEffect(() => {{
    fetchRecords();
  }}, [fetchRecords]);

  // Realtime sync auto-refetch
  useEffect(() => {{
    const unsubscribe = client.subscribe(`models:${{model.toLowerCase()}}`, () => {{
      fetchRecords();
    }});
    return unsubscribe;
  }}, [client, model, fetchRecords]);

  return {{ data, total, limit, offset, isLoading, error, refetch: fetchRecords }};
}}

export function useRealtimeSubscription(topic: string, callback: (event: any) => void) {{
  const client = useVella();
  useEffect(() => {{
    return client.subscribe(topic, callback);
  }}, [client, topic, callback]);
}}
"#
    )
}
