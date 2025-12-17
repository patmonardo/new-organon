# @organon/model Documentation Index

## Overview

The `@model` package implements the **Form Processor Client SDSL**—the client-side MVC that consumes the `@logic` Form Processor. It embodies the **Logic:Model Dyad**, where Logic is the "perfect server" and Model is the first activation of Logic's power.

---

## Architecture

### Package Structure

```
@organon/model/
├── src/
│   ├── sdsl/           # MVC SDSL Core (form-model, form-view, form-controller)
│   ├── data/           # Data services (FactStore, Polars, Hydrator)
│   └── schema/         # Zod schemas (FormShape, Application, Dashboard)
├── examples/           # Working examples (customer MVC)
└── test/               # Test suite
```

### System Flow

```
@logic/FactStore (First Speaker)
      │
      │ speaks FormShape
      ▼
Form Processor (Host)
      │
      │ runs
      ▼
MVC SDSL (Client)
      │
      ├── FormModel    → State : Structure
      ├── FormView     → Representation : Perspective
      └── FormController → Action : Rule
                │
                │ outputs
                ▼
          DisplayDocument
                │
                │ adapted by
                ▼
          Adapters (React, Radix)
                │
                ▼
          Runtime UI
```

---

## Core Concepts

### 1. MVC SDSL

The MVC SDSL is a **Specification DSL** that schematizes the Form Process in its action/usage:

| Component | Dyad | Responsibility |
|-----------|------|----------------|
| **FormModel** | State : Structure | Hold data, validate, persist |
| **FormView** | Representation : Perspective | Transform to display, filter, format |
| **FormController** | Action : Rule | Orchestrate, handle actions, apply rules |

See: `src/sdsl/form-model.ts`, `src/sdsl/form-view.ts`, `src/sdsl/form-controller.ts`

### 2. Agent MVC

Parallel MVC for agent reasoning:

| Component | Role |
|-----------|------|
| **AgentModel** | Facts + overlays (relevance, confidence, provenance) |
| **AgentView** | ContextDocument, prompts, function calls |
| **AgentController** | Query, infer, assert, retract, hypothesize |

See: `src/sdsl/agent-model.ts`, `src/sdsl/agent-view.ts`, `src/sdsl/agent-controller.ts`

### 3. Adapters

Adapters translate `DisplayDocument` into runtime-specific output:

| Adapter | Target | Use Case |
|---------|--------|----------|
| **ReactAdapter** | React/JSX | Server-side, simple forms |
| **RadixAdapter** | Radix/Tailwind | Interactive, rich UI |
| **JSONAdapter** | JSON | API responses, serialization |
| **HTMLAdapter** | HTML string | Static rendering |

See: `src/sdsl/react-adapter.tsx`, `src/sdsl/radix-adapter.tsx`

### 4. Data Layer

The data layer bridges semantic models to forms:

| Component | Role |
|-----------|------|
| **DataModel/DataView** | Semantic modeling (measures, dimensions) |
| **SemanticHydrator** | Maps data results to FormModel fields |
| **PolarsExecutionEngine** | Executes DataView via Polars/Arrow |
| **FactStore** | Mock fact store for @logic integration |

See: `src/data/sdsl.ts`, `src/data/semantic-hydrator.ts`, `src/data/polars-engine.ts`

---

## Schemas

### Core Schemas

| Schema | Purpose |
|--------|---------|
| `FormShapeSchema` | Universal shape flowing through SDSL |
| `ApplicationSchema` | Complete application definition |
| `DashboardSchema` | Dashboard components |
| `DisplayDocumentSchema` | Generic display output |

See: `src/schema/shape.ts`, `src/schema/application.ts`, `src/schema/dashboard.ts`

### Component Schemas

| Schema | Purpose |
|--------|---------|
| `TableShapeSchema` | Data tables |
| `ListShapeSchema` | Lists and navigation |
| `LinkShapeSchema` | Navigation links |
| `ButtonShapeSchema` | Actions and buttons |
| `TextShapeSchema` | Text content |
| `RadixShapeSchema` | Radix primitive mappings |

See: `src/schema/table.ts`, `src/schema/list.ts`, `src/schema/link.ts`

---

## Testing

```bash
pnpm test           # Run all tests
pnpm test:watch     # Watch mode
```

Test coverage:
- `sdsl.test.ts` — MVC core
- `agent-mvc.test.ts` — Agent MVC
- `react-adapter.test.tsx` — React adapter
- `radix-adapter.test.tsx` — Radix adapter
- `semantic-hydrator.test.ts` — Data hydration
- `polars-engine.test.ts` — Execution engine

---

## Dependencies

### Runtime
- `zod` — Schema validation
- `nodejs-polars` — Columnar data processing
- `apache-arrow` — Arrow buffers
- `duckdb` — Query planning (EXPLAIN)

### Development
- `react` / `react-dom` — UI rendering
- `vitest` — Testing
- `@testing-library/react` — React testing

---

## FPU:RealityPipe Architecture

| Term | Full Name | Role |
|------|-----------|------|
| **FPU** | Form Processor Unit | CPU:GPU sublation (Qualitative:Quantitative) |
| **RealityPipe** | Reality Pipe | In-process envelope + adapter interface |

```
FPU (Form Processor Unit)
  ├─ CPU: Qualitative Properties (Logic, Relations)
  └─ GPU: Quantitative Properties (Numbers, Analytics)
      │
      │ FormShape
      ▼
RealityPipe (in-process)
      │
      ├─→ React Adapter → UI
      ├─→ Radix Adapter → Rich UI
      ├─→ Agent Adapter → Reasoning
      └─→ Chain... → Results flow through
```

Both UI components and Agents are adapters on the same RealityPipe. The `@logic:@model` dyad (`FPU:RealityPipe`) serves them equally—agents create Logical Models the same way UI creates displays.

See: `doc/fpu-fci-architecture.md`

---

## Related Packages

| Package | Role |
|---------|------|
| `@organon/logic` | Form Processor (CPU) |
| `@organon/task` | Workflow/Agent layer |
| `@organon/core` | Shared utilities |

---

## Quick Reference

### Create MVC Stack

```typescript
import { createFormMVC, reactAdapter } from '@organon/model';

const mvc = createFormMVC(customerShape, 'edit');
const doc = mvc.display();
const jsx = reactAdapter.render(doc, { handler: mvc.createHandler() });
```

### Define Semantic Model

```typescript
import { defineModel, sum, avg } from '@organon/model';

const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  fields: { id: z.string(), name: z.string(), revenue: z.number() },
  measures: {
    totalRevenue: sum('revenue'),
    avgRevenue: avg('revenue'),
  },
  dimensions: {
    region: 'region',
  },
});
```

### Hydrate Form

```typescript
const snapshot = await hydrator.hydrate(formModel, {
  id: 'customer-profile',
  view: (ctx) => CustomerModel.view({ filter: { id: ctx.params.id } }),
  fields: [{ fieldId: 'name', source: 'name' }],
  metrics: [{ name: 'total', source: 'totalRevenue', fieldId: 'revenue' }],
});
```

