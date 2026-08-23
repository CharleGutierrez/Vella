/// Auto-generated Angular 17+ Signals SDK for Vella backend.
pub fn generate_angular_sdk(base_url: &str) -> String {
    format!(
        r#"/**
 * ⚡ Vella Angular 17+ Client SDK (Signals & Injectable Services)
 * Auto-generated Angular Service & Realtime Client for Vella backend.
 */

import {{ Injectable, signal, computed }} from '@angular/core';

export interface VellaUser {{
  id: number;
  username: string;
  email: string;
  role: 'Admin' | 'Manager' | 'Editor' | 'Viewer';
  is_active: boolean;
}}

@Injectable({{
  providedIn: 'root'
}})
export class VellaService {{
  private baseUrl = '{base_url}'.replace(/\/$/, '');
  private token = signal<string | null>(null);

  constructor() {{}}

  setToken(t: string | null) {{
    this.token.set(t);
  }}

  async request<T = any>(path: string, options: RequestInit = {{}}): Promise<T> {{
    const headers: Record<string, string> = {{
      'Content-Type': 'application/json',
      ...(options.headers as Record<string, string> || {{}}),
    }};
    const currentToken = this.token();
    if (currentToken) {{
      headers['Authorization'] = `Bearer ${{currentToken}}`;
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
}}
"#
    )
}
