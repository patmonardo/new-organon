# Logic:Model Dyad - Regeneration Plan

## Core Vision

The **Logic:Model Dyad** is the claim to fame. It's not a standard API contract; it's a semantic dyad where:
- **Logic (GDS/GDSL)**: The "perfect server"—reflection, EssentialRelations, execution engine (Polars/Arrow/DuckDB), nondual semantics.
- **Model (@model)**: The first activation of Logic's power—the declarative DSL that *uses* what Logic provides.

The dyad is sublated into the **Task/Agent Workflow layer**, where agents engage the dyad: declare an Application, execute it, hydrate it, persist it.

## Architecture Hierarchy

```
Agent (Task Workflow Layer)
  ↓ engages
Logic:Model Dyad
  ├─ Logic: reflection, execution, persistence
  └─ Model: declarative schemas (Application, Dashboard, Forms, Views, Navigation)
  ↓ powers
UI Rendering (React/Radix/Tailwind)
```

## @model Package Structure (React-First → Radix)

### What Survives (Keep)
- **SDSL contracts**: form-*, view-*, controller-* types (cleaned up, your naming).
- **Hydrator + Polars/Arrow/DuckDB**: semantic bridge from data to forms.
- **DisplayDocument/DisplayElement**: the generic rendering contract.
- **Application schema** (`application.ts`): the apex—declares dashboard, views, forms, models, navigation.
- **Zod-based validation**: tight schema validation throughout.

### What We Drop/Rebuild
- AI-era file naming and folder layouts (replace with your aesthetic).
- Legacy Prisma references.
- Redundant or unclear data services (rebuild Polars/Arrow/DuckDB path lean and clear).
- Speculative adapters/widgets (keep only high-conviction patterns).

### Folder Layout (Proposal)

```
@organon/model/
├── src/
│   ├── sdsl/
│   │   ├── form-model.ts       # FormModel: state, validation, persistence
│   │   ├── form-view.ts        # FormView: emits DisplayDocument
│   │   ├── form-controller.ts  # FormController: orchestration, hydration hooks
│   │   ├── agent-model.ts      # AgentModel (parallels FormModel for agents)
│   │   ├── agent-view.ts       # AgentView (parallels FormView for agents)
│   │   ├── agent-controller.ts # AgentController (parallels FormController)
│   │   ├── types.ts            # DisplayDocument, DisplayElement, FormShape, etc.
│   │   ├── adapter-react.ts    # React adapter: DisplayDocument → JSX (no Radix)
│   │   ├── adapter-radix.ts    # Radix adapter: DisplayDocument → Radix JSX
│   │   └── index.ts            # SDSL exports
│   │
│   ├── data/
│   │   ├── semantic-hydrator.ts       # SemanticHydrator: core orchestrator
│   │   ├── semantic-data-service.ts   # Interface: execute(view, opts)
│   │   ├── polars-engine.ts          # PolarsExecutionEngine: Polars/Arrow
│   │   ├── duckdb-explain.ts         # DuckDB EXPLAIN integration (optional)
│   │   ├── snapshot-persistence.ts   # Export/import HydratorSnapshot (Arrow/JSON)
│   │   └── index.ts                  # Data exports
│   │
│   ├── schema/
│   │   ├── shape.ts            # FormShape, FormField, FormLayout, FormAction
│   │   ├── application.ts      # Application, Dashboard, View, Navigation (apex)
│   │   ├── dashboard.ts        # DashboardShape, DashboardSection, StatCard
│   │   ├── form.ts             # Form-specific schemas
│   │   ├── view.ts             # View-specific schemas
│   │   ├── list.ts             # ListShape (breadcrumbs, navbars)
│   │   ├── link.ts             # LinkShape
│   │   ├── radix.ts            # Radix component schemas (dialog, tabs, etc.)
│   │   └── index.ts            # Schema exports
│   │
│   ├── ui/
│   │   ├── primitives/
│   │   │   ├── button.tsx      # Your button component (Tailwind-based)
│   │   │   ├── input.tsx       # Your input component
│   │   │   ├── card.tsx        # Your card component
│   │   │   ├── layout.tsx      # Stack, grid, page layout
│   │   │   └── index.ts
│   │   │
│   │   ├── radix/
│   │   │   ├── dialog.tsx      # Radix Dialog wrapper (your tokens)
│   │   │   ├── tabs.tsx        # Radix Tabs wrapper
│   │   │   ├── accordion.tsx   # Radix Accordion wrapper
│   │   │   └── index.ts
│   │   │
│   │   └── index.ts
│   │
│   └── index.ts                # Main package export
│
├── examples/
│   ├── customer/
│   │   ├── customer.model.ts      # Customer semantic model (Malloy-inspired)
│   │   ├── customer.schema.ts     # Customer FormShape
│   │   ├── customer.controller.ts # CustomerController with hydration
│   │   ├── customer.view.tsx      # Optional: React-specific view
│   │   ├── data.service.ts        # CustomerDataService (mocked)
│   │   ├── app.ts                 # Example Application definition
│   │   └── README.md
│   │
│   ├── dashboard/
│   │   ├── README.md
│   │   └── ...
│   │
│   └── index.ts                  # Examples exports/discovery
│
├── test/
│   ├── sdsl/
│   │   ├── form-model.test.ts
│   │   ├── form-view.test.ts
│   │   ├── form-controller.test.ts
│   │   ├── adapter-react.test.tsx
│   │   ├── adapter-radix.test.tsx
│   │   └── ...
│   │
│   ├── data/
│   │   ├── semantic-hydrator.test.ts
│   │   ├── polars-engine.test.ts
│   │   ├── snapshot-persistence.test.ts
│   │   └── ...
│   │
│   ├── schema/
│   │   ├── application.test.ts
│   │   └── ...
│   │
│   └── examples/
│       ├── customer.integration.test.tsx
│       └── ...
│
├── tsconfig.json               # rootDir: ".", includes src + examples
├── package.json
└── README.md
```

## Regeneration Passes (Guided, Sequential)

### Pass 1: Scaffold + Naming
- Rename/move folders to match the layout above.
- Drop legacy files (AI-era, Prisma, redundant services).
- Update tsconfig to reflect new rootDir + paths.
- Ensure `examples/` is type-checkable.

### Pass 2: React Adapter (Adapter-First)
- Build `adapter-react.ts`: clean DisplayDocument → JSX mapping.
- No Radix; no Tailwind classes hard-coded. Just React nodes.
- Define minimal, reusable primitives in `ui/primitives/` (button, input, card, layout).
- Add basic Tailwind tokens (spacing, colors, typography).
- Test: verify DisplayDocument renders as expected React.

### Pass 3: SDSL Contracts (Clean + Clear)
- Review + rename `form-model.ts`, `form-view.ts`, `form-controller.ts`.
- Ensure hydrator hooks are explicit and minimal.
- Verify FormShape, DisplayDocument APIs are stable and your style.
- Write inline tests for each.

### Pass 4: Radix Adapter
- Build `adapter-radix.ts`: same DisplayDocument → Radix JSX.
- Define wrappers in `ui/radix/` (dialog, tabs, accordion, etc.).
- Layer Radix primitives + your Tailwind tokens.
- Test: verify Radix markup matches DisplayDocument expectations.

### Pass 5: Data Services (Polars/Arrow/DuckDB)
- Harden `semantic-hydrator.ts` (clean, minimal API).
- Rebuild `polars-engine.ts` with your naming.
- Add `snapshot-persistence.ts`: export/import HydratorSnapshot (Arrow buffer + JSON meta).
- Remove Prisma entirely; keep mock/in-memory for tests.
- Test: verify hydrator → snapshot → Radix render works end-to-end.

### Pass 6: Examples + Integration Tests
- Maintain `examples/dashboard/` as the primary demo.
- Write integration tests: Application → Controller → Hydrate → Render (aligned to dashboard demo).

### Pass 7: Final Polish
- TSLint, format, doc comments.
- Update README with vision (Logic:Model Dyad, Polars/Arrow, Radix/Tailwind).
- Archive old code; clean up.

## Key Invariants (Hold These)

1. **Application is the apex**: Everything flows from and back to `Application` schema.
2. **DisplayDocument is the rendering contract**: Adapters consume it; they don't leak Radix/React specifics back.
3. **Hydrator is the data bridge**: Controllers use it; it's agnostic to UI.
4. **SDSL types are tight**: No bloat; validation is strict (Zod).
5. **Examples are golden**: They prove the flow works end-to-end.

## Archived Resources Strategy

- Keep `@organon/model/archive/` for reference (old Sankara forms, widget patterns, etc.).
- Port selectively into `ui/` with your naming; link back to source.
- Document "greenlit" patterns in comments.

## Next Steps

1. Confirm folder/naming layout (adjust as needed).
2. Begin Pass 1: Scaffold.
3. Move to React-first (Pass 2) before Radix.
4. Test frequently; keep integration tests green.
