/// Auto-generated Vue 3 Composition API SDK for Vella backend.
pub fn generate_vue_sdk(base_url: &str) -> String {
    format!(
        r#"/**
 * ⚡ Vella Vue 3 SDK (Composition API & Pinia Ready)
 * Auto-generated Vue Composables & Realtime WebSocket Client for Vella.
 */

import {{ ref, shallowRef, onMounted, onUnmounted, watch }} from 'vue';

export interface VellaUser {{
  id: number;
  username: string;
  email: string;
  role: 'Admin' | 'Manager' | 'Editor' | 'Viewer';
  is_active: boolean;
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
    const res = await fetch(`${{this.baseUrl}}${{path}}`, {{ ...options, headers, credentials: 'include' }});
    if (!res.ok) {{
      const err = await res.json().catch(() => ({{ message: res.statusText }}));
      throw new Error(err.error?.message || err.message || 'Request failed');
    }}
    return res.json();
  }}

  async list<T = any>(model: string, queryParams: Record<string, any> = {{}}) {{
    const qs = new URLSearchParams(queryParams).toString();
    return this.request(`/api/d/${{model.toLowerCase()}}${{qs ? '?' + qs : ''}}`);
  }}

  async get<T = any>(model: string, id: number) {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`);
  }}

  async create<T = any>(model: string, data: Partial<T>) {{
    return this.request(`/api/d/${{model.toLowerCase()}}`, {{
      method: 'POST',
      body: JSON.stringify(data),
    }});
  }}

  async update<T = any>(model: string, id: number, data: Partial<T>) {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{
      method: 'PUT',
      body: JSON.stringify(data),
    }});
  }}

  async delete(model: string, id: number) {{
    return this.request(`/api/d/${{model.toLowerCase()}}/${{id}}`, {{ method: 'DELETE' }});
  }}

  async searchVector<T = any>(model: string, queryVector: number[], topK: number = 10) {{
    return this.request(`/api/d/${{model.toLowerCase()}}/search-vector`, {{
      method: 'POST',
      body: JSON.stringify({{ model, query_vector: queryVector, top_k: topK }}),
    }});
  }}

  subscribe(topic: string, callback: (event: any) => void) {{
    if (!this.ws || this.ws.readyState === WebSocket.CLOSED) {{
      const wsProtocol = this.baseUrl.startsWith('https') ? 'wss:' : 'ws:';
      const host = this.baseUrl.replace(/^https?:\/\//, '');
      this.ws = new WebSocket(`${{wsProtocol}}//${{host}}/api/realtime/ws`);
      this.ws.onmessage = (e) => {{
        try {{
          const msg = JSON.parse(e.data);
          this.listeners.get(msg.topic)?.forEach((cb) => cb(msg));
        }} catch (err) {{}}
      }};
    }}
    if (!this.listeners.has(topic)) {{
      this.listeners.set(topic, new Set());
    }}
    this.listeners.get(topic)!.add(callback);
    return () => this.listeners.get(topic)?.delete(callback);
  }}
}}

export const defaultClient = new VellaClient();

/**
 * Vue 3 Composable: useVellaQuery
 */
export function useVellaQuery<T = any>(model: string, queryParams: Record<string, any> = {{}}, client: VellaClient = defaultClient) {{
  const data = ref<T[]>([]);
  const total = ref(0);
  const isLoading = ref(true);
  const error = ref<Error | null>(null);

  const fetchRecords = async () => {{
    isLoading.value = true;
    error.value = null;
    try {{
      const res = await client.list<T>(model, queryParams);
      data.value = res.data;
      total.value = res.total;
    }} catch (err: any) {{
      error.value = err;
    }} finally {{
      isLoading.value = false;
    }}
  }};

  onMounted(() => {{
    fetchRecords();
    const unsub = client.subscribe(`models:${{model.toLowerCase()}}`, () => fetchRecords());
    onUnmounted(() => unsub());
  }});

  return {{ data, total, isLoading, error, refetch: fetchRecords }};
}}
"#
    )
}
