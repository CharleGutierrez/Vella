# Vella Frontend UX/UI Manual

Welcome to the Vella framework! This manual is an incredibly detailed, step-by-step tutorial designed for UX/UI Developers (Frontend). It covers how to consume Vella's dynamic SDKs, manage state effectively, and build real-time components with seamless IDE autocomplete.

## Table of Contents
1. [Introduction](#introduction)
2. [Fetching the Dynamic SDKs](#fetching-the-dynamic-sdks)
3. [State Management](#state-management)
    - [React Context Setup](#react-context-setup)
    - [Vue Pinia Setup](#vue-pinia-setup)
4. [Real-time WebSocket Updates](#real-time-websocket-updates)
5. [IDE Autocomplete & Developer Experience](#ide-autocomplete--developer-experience)

---

## Introduction
Vella empowers UX/UI developers by automatically generating robust SDKs based on the backend schema. This eliminates the need for manual API wiring and allows you to focus on creating beautiful, responsive interfaces.

## Fetching the Dynamic SDKs
When the backend starts, Vella generates a typed SDK that you can import directly into your frontend project.

```bash
npm install @vella/sdk-client
```

```javascript
import { createVellaClient } from '@vella/sdk-client';

const client = createVellaClient({
    endpoint: 'http://localhost:4000/api'
});
```

## State Management
Vella seamlessly integrates with modern state management libraries.

### React Context Setup
Wrap your application in the Vella Provider to make the client available throughout your component tree.

```jsx
import { VellaProvider } from '@vella/sdk-client/react';

function App() {
  return (
    <VellaProvider client={client}>
      <YourAppComponents />
    </VellaProvider>
  );
}
```

### Vue Pinia Setup
For Vue, Vella provides a Pinia plugin to synchronize backend state.

```javascript
import { createPinia } from 'pinia';
import { vellaPiniaPlugin } from '@vella/sdk-client/vue';

const pinia = createPinia();
pinia.use(vellaPiniaPlugin(client));
```

## Real-time WebSocket Updates
Vella makes real-time updates trivial. Use the generated hooks to subscribe to data changes.

```jsx
import { useVellaSubscription } from '@vella/sdk-client/react';

function LiveDataComponent() {
  const { data, error } = useVellaSubscription('MarketData', { symbol: 'BTC_USD' });

  if (error) return <div>Error loading live data</div>;
  if (!data) return <div>Waiting for update...</div>;

  return (
    <div>
      <h3>Live Price: ${data.price}</h3>
    </div>
  );
}
```

## IDE Autocomplete & Developer Experience
Because Vella generates strict TypeScript definitions from the backend schema, your IDE (like VS Code) will provide complete autocomplete, type checking, and inline documentation for all API methods, models, and real-time events.

Happy UI building with Vella!
