# The Vella Framework: Definitive Guide for Frontend Developers

Welcome to the definitive guide for integrating the Vella framework into your frontend applications! Whether you are building with React, Vue, or Angular, Vella provides a seamless, realtime, and **100% type-safe** experience.

Vella's standout feature is its **Zero-Config TypeScript Sync**. This means that your backend models and API contracts are automatically synchronized with your frontend, providing guaranteed type safety and rich IDE autocomplete across all three major frameworks without any manual configuration or boilerplate.

---

## 1. The SDK Endpoints

Vella dynamically generates framework-specific SDKs based on your backend schema. You don't need to manually write API clients or types. Instead, you simply pull the SDK directly from your Vella server.

Use `curl` (or your preferred HTTP client/build step) to download the SDK for your framework:

**For React:**
```bash
curl http://localhost:3000/api/sdk/react > src/vella-sdk.ts
```

**For Vue:**
```bash
curl http://localhost:3000/api/sdk/vue > src/vella-sdk.ts
```

**For Angular:**
```bash
curl http://localhost:3000/api/sdk/angular > src/vella-sdk.ts
```

*Note: In a production workflow, you can integrate this step into your CI/CD pipeline or `package.json` scripts to ensure your types are always up to date.*

---

## 2. React (Hooks & Context)

Vella provides a robust integration for React using Context and Custom Hooks, ensuring that your components react instantly to data changes while maintaining strict type safety.

### Setup: `VellaProvider`

First, wrap your application with the `VellaProvider`.

```tsx
// src/App.tsx
import React from 'react';
import { VellaProvider } from './vella-sdk';
import { Dashboard } from './Dashboard';

const App = () => {
  return (
    <VellaProvider endpoint="http://localhost:3000">
      <Dashboard />
    </VellaProvider>
  );
};

export default App;
```

### Usage: `useVellaQuery`

Use the auto-generated `useVellaQuery` hook inside your components. Thanks to Zero-Config TypeScript Sync, `data` will be fully typed according to your backend schema.

```tsx
// src/Dashboard.tsx
import React from 'react';
import { useVellaQuery } from './vella-sdk';

export const Dashboard = () => {
  // Autocomplete will suggest available queries (e.g., 'getUsers')
  const { data, loading, error } = useVellaQuery('getUsers');

  if (loading) return <div>Loading...</div>;
  if (error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <h1>Realtime Users</h1>
      <ul>
        {data.map((user) => (
          // 'user' is fully typed! user.id, user.name, etc.
          <li key={user.id}>{user.name} ({user.email})</li>
        ))}
      </ul>
    </div>
  );
};
```

---

## 3. Vue 3 (Composition API)

For Vue 3 developers, Vella leverages the power of the Composition API, integrating smoothly with `ref` and Vue's reactivity system.

### Usage: `useVella` composable

Here is a full example of a realtime component using the auto-generated Vue composable.

```vue
<!-- src/components/Dashboard.vue -->
<template>
  <div>
    <h1>Realtime Users</h1>
    <div v-if="loading">Loading...</div>
    <div v-else-if="error">Error: {{ error.message }}</div>
    <ul v-else>
      <li v-for="user in users" :key="user.id">
        {{ user.name }} ({{ user.email }})
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useVella } from '../vella-sdk';

const { query } = useVella({ endpoint: 'http://localhost:3000' });

// Fully typed reactive reference
const users = ref([]);
const loading = ref(true);
const error = ref(null);

onMounted(async () => {
  try {
    // IDE autocomplete knows 'getUsers' and its return type!
    const response = await query('getUsers');
    users.value = response.data;
  } catch (err) {
    error.value = err;
  } finally {
    loading.value = false;
  }
});
</script>
```

---

## 4. Angular (Signals & Services)

Vella embraces modern Angular by generating injectable Services that utilize Angular Signals for deeply reactive, type-safe data binding.

### Usage: Injecting the Service and Using Signals

You can inject the Vella service directly into your components. The generated service uses Signals to expose state, making your templates incredibly efficient.

```typescript
// src/app/dashboard.component.ts
import { Component, inject, OnInit } from '@angular/core';
import { CommonModule } from '@angular/common';
import { VellaService } from '../vella-sdk';

@Component({
  selector: 'app-dashboard',
  standalone: true,
  imports: [CommonModule],
  template: `
    <div>
      <h1>Realtime Users</h1>
      
      @if (loading()) {
        <div>Loading...</div>
      }
      
      @if (error()) {
        <div>Error: {{ error()?.message }}</div>
      }
      
      @if (!loading() && !error()) {
        <ul>
          @for (user of users(); track user.id) {
            <li>{{ user.name }} ({{ user.email }})</li>
          }
        </ul>
      }
    </div>
  `
})
export class DashboardComponent implements OnInit {
  // Inject the auto-generated service
  private vellaService = inject(VellaService);

  // Expose signals to the template
  // Type safety is guaranteed end-to-end!
  users = this.vellaService.usersSignal;
  loading = this.vellaService.loadingSignal;
  error = this.vellaService.errorSignal;

  ngOnInit() {
    // Initialize the realtime connection and fetch data
    this.vellaService.connect('http://localhost:3000');
    this.vellaService.fetchUsers();
  }
}
```

---

## Summary: The Power of Zero-Config TypeScript Sync

Whether you choose React, Vue, or Angular, Vella's **Zero-Config TypeScript Sync** guarantees:

1. **No manual type definitions:** Stop writing interfaces for your API responses.
2. **Instant Feedback:** If a backend model changes, your frontend build will fail, preventing runtime errors.
3. **Developer Velocity:** Rich IDE autocomplete guides you as you build, reducing the need to constantly check API documentation.

Happy building with Vella!
