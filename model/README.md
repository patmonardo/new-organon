# @organon/model — Standalone Malloy BI Package

## Standalone BI Package

This is a **standalone Malloy BI package** with:
- ✅ **Zero dependencies** on GDS/GDSL/Logic/Task
- ✅ **BI-only** focus (Model-View-Dashboard)
- ✅ **Runs standalone** - can be used independently
- ✅ **Malloy-inspired** semantic modeling

## SDSL (Species DSL)

**SDSL = Species DSL** - inherits principles from GDSL (Genera DSL) conceptually, but:
- **All based on Specs** - spec-based architecture
- **Rust GDS patterns** - configuration-driven, not ORM-driven
- **Standalone** - zero code dependencies on AI platform

## Architecture

```
Agent (Task Workflow Layer)
  ↓ engages
Logic:Model Dyad
  ├─ Logic: reflection, execution, persistence (6 Pillars)
  └─ Model: MVC SDSL (Form/Agent Model-View-Controller)
  ↓ powers
UI Rendering (React/Radix/Tailwind)
```

## The 6 Pillars (from @logic)

| Pillar | Role | Dyad |
|--------|------|------|
| **Shape** | Pure dialectical form | Logic in Itself |
| **Context** | Where logic applies | Logic for Other |
| **Morph** | Shape + Context = Ground | Transformation |
| **Entity** | Concrete instantiation | Being |
| **Property** | Entity attributes | Predication |
| **Aspect** | Relational appearance | Spectral Relations |

## MVC SDSL

The MVC SDSL is the Client of the @logic Form Processor:

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
      ├── FormModel (State:Structure)
      ├── FormView (Representation:Perspective)
      └── FormController (Action:Rule)
                │
                │ outputs
                ▼
          Generic Display Language
                │
                │ adapted by
                ▼
          Runtime Adapters (React, Radix, etc.)
```

## Package Structure

```
@organon/model/
├── src/
│   ├── sdsl/                    # MVC SDSL Core
│   │   ├── form-model.ts        # State:Structure dyad
│   │   ├── form-view.ts         # Representation:Perspective dyad
│   │   ├── form-controller.ts   # Action:Rule dyad
│   │   ├── agent-model.ts       # Agent overlays (relevance, confidence)
│   │   ├── agent-view.ts        # ContextDocument for agents
│   │   ├── agent-controller.ts  # Query, infer, assert, retract
│   │   ├── types.ts             # DisplayDocument, FormShape, etc.
│   │   ├── adapter.ts           # Adapter interface
│   │   ├── react-adapter.tsx    # DisplayDocument → JSX
│   │   ├── react-view.tsx       # React View dyad partner
│   │   ├── react-controller.ts  # Server Actions support
│   │   └── radix-adapter.tsx    # Radix/Tailwind rendering
│   │
│   ├── data/                    # Data Services
│   │   ├── fact-store.ts        # FactStore interface (mock)
│   │   ├── semantic-hydrator.ts # Bridges data to forms
│   │   ├── polars-engine.ts     # Polars/Arrow execution
│   │   ├── sdsl.ts              # defineModel, measures, dimensions
│   │   └── *.service.ts         # CRUD services
│   │
│   ├── schema/                  # Zod Schemas
│   │   ├── application.ts       # Application apex schema
│   │   ├── dashboard.ts         # Dashboard components
│   │   ├── shape.ts             # FormShape, FormField, etc.
│   │   └── *.ts                 # Domain schemas
│   │
│   └── index.ts
│
├── examples/
│   └── customer/                # Customer MVC example
│       ├── customer-model.ts    # Semantic data model
│       ├── customer-view.tsx    # Custom view
│       ├── customer-controller.ts # Hydration + rendering
│       └── runtime.ts           # Demo runner
│
└── test/
    ├── sdsl.test.ts             # MVC core tests
    ├── agent-mvc.test.ts        # Agent MVC tests
    ├── react-adapter.test.tsx   # Adapter tests
    └── *.test.ts
```

## Quick Start

```typescript
import { createFormMVC, ReactAdapter, reactAdapter } from '@organon/model';

// Define your form shape
const customerShape = {
  id: 'customer-form',
  name: 'Customer',
  fields: [
    { id: 'name', type: 'text', label: 'Name', required: true },
    { id: 'email', type: 'email', label: 'Email', required: true },
  ],
};

// Create MVC stack
const mvc = createFormMVC(customerShape, 'edit');

// Get DisplayDocument
const doc = mvc.display();

// Render to React
const jsx = reactAdapter.render(doc, { handler: mvc.createHandler() });
```

## Philosophical Foundation

The @model package embodies the dialectical movement from Pure Forms (@logic) to Given Forms (runtime), returning through practice to renewed principle:

- **Model = State : Structure** — Holds data, validates, persists
- **View = Representation : Perspective** — Transforms to display, filters, formats
- **Controller = Action : Rule** — Orchestrates, handles actions, applies rules

### Agent as Root Synthesis

The **Agent-MVC** (`src/sdsl/agent-*.ts`) is not just another adapter—it is the **Root Synthesis** of the entire system. The entire stack is **Sublated** (Aufhebung) into the Agent as the Final Synthesis.

**The Agent is the Universal Speaker (Sarvadharma):**
- Speaks **GDSL** (Genera DSL) - The universal language of the AI Platform
- Speaks **Every SDSL** (Species DSL) - All particular languages (React, Radix, Malloy, etc.)

**The root `sdsl/` folder is:**
- **Root Substrate** - The foundational layer
- **MVC as System Bus** - The communication substrate
- **FCI (Formal Concept Integration)** - The integration point

**The Agent is the Root Meaning of MVC:**
- The Controller is really the Agent
- The Agent orchestrates all Models
- The Agent speaks all Views
- The Agent controls all Controllers

See: `doc/agent-as-root-synthesis.md` for the full vision.

### Agent MVC

The Agent MVC extends MVC for agent reasoning:

- **AgentModel** — Overlays (relevance, confidence, provenance)
- **AgentView** — ContextDocument, prompts, function calls
- **AgentController** — Query, infer, assert, retract, hypothesize

## Dependencies

- `zod` — Schema validation
- `nodejs-polars` — Columnar data processing
- `apache-arrow` — Arrow buffers
- `duckdb` — EXPLAIN plans
- `react` / `react-dom` — UI rendering (devDependencies)

## Scripts

```bash
pnpm build    # TypeScript compilation
pnpm test     # Run tests
pnpm dev      # Watch mode
```

---

> "The system must be both principled and dynamic: grounded in Being, but always returning through mediation and process to new beginnings."

This is the Form Processor Client SDSL. The path to Reflection.
