# Dual Adapter Strategy: React + Radix

## The Strategy

Support both **React** and **Radix** adapters as standard clients:

| Adapter | Target | Use Case |
|---------|--------|----------|
| **React Adapter** | Generic React/JSX | Server-side, simple forms, lightweight |
| **Radix Adapter** | Radix/Tailwind | Client interactivity, rich UI, dialogs |

Both build on the same **FORM SDK MVC** foundation (form-model, form-view, form-controller).

## Architecture

```
FormShape IR
    ↓
DisplayDocument
    ├─→ React Adapter → Generic React Components (Server-side)
    └─→ Radix Adapter → Radix Components (Client-side)
```

## React Adapter

**For simple server-side components:**

```typescript
import { reactAdapter } from '@organon/model';

const jsx = reactAdapter.render(displayDocument, {
  handler: controller.createHandler(),
});
```

**Best for:**
- Dashboard (static)
- Card (simple)
- Table (basic)
- Font, Image
- Server-side rendering

**Implementation:** `src/sdsl/react-adapter.tsx`

## Radix Adapter

**For interactive client-side components:**

```typescript
import { radixAdapter } from '@organon/model';

const jsx = radixAdapter.render(displayDocument, {
  snapshot: semanticSnapshot,
  handler: controller.createHandler(),
});
```

**Best for:**
- Card (interactive)
- Dialog, Popover
- Tabs, Accordion
- Table (interactive)
- Rich UI interactions

**Implementation:** `src/sdsl/radix-adapter.tsx`

## Client Architecture

### Generic FORM SDK MVC

```
form-model.ts      # Generic Model (data, validation)
form-view.ts       # Generic View (display transformation)
form-controller.ts # Generic Controller (orchestration)
```

### React Client

```
react-controller.ts  # Extends FormController, Server Actions
react-view.tsx       # Extends FormView, React rendering
react-adapter.tsx    # DisplayDocument → JSX
```

### Radix Client

```
radix-controller.tsx # Radix state management
radix-adapter.tsx    # DisplayDocument → Radix primitives
radix-primitives.tsx # Radix component registry
```

## Benefits

1. **Flexibility** — Choose the right adapter for use case
2. **Server-First** — React adapter for simple server-side
3. **Rich UI** — Radix adapter for interactive client-side
4. **Same Foundation** — Both use DisplayDocument and FORM SDK
5. **Migration Path** — Start simple, add interactivity as needed

## Component Mapping

| Component | React Adapter | Radix Adapter |
|-----------|---------------|---------------|
| Card | Simple HTML | `@radix-ui/react-card` |
| Table | `<table>` | Interactive with sorting |
| Dialog | N/A | `@radix-ui/react-dialog` |
| Tabs | N/A | `@radix-ui/react-tabs` |
| Popover | N/A | `@radix-ui/react-popover` |
| Button | `<button>` | `@radix-ui/react-slot` |
| Form | `<form>` + Server Action | `<form>` + handlers |

## Usage Pattern

```typescript
// Server Component (React Adapter)
export default async function CustomerPage() {
  const mvc = createFormMVC(customerShape, 'view');
  const doc = mvc.display();
  return reactAdapter.render(doc);
}

// Client Component (Radix Adapter)
'use client';
export function CustomerDashboard({ shape }) {
  const mvc = createFormMVC(shape, 'edit');
  const doc = mvc.display();
  return radixAdapter.render(doc, { 
    handler: mvc.createHandler() 
  });
}
```

