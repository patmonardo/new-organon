# Semantic Hydrator Design

## Goals

1. **Reusable bridge** between the Data SDSL (semantic models) and the MVC Form stack (FormModel + Controller).
2. **Zero-Prisma** data path that works with any execution engine (Polars, SQL, mock JSON) and produces field-ready values.
3. **Deterministic mappings** so controllers can describe how semantic columns populate form fields, action payloads, and derived KPIs.
4. **Composable snapshots** that bundle the semantic plan, raw rows (Arrow/JSON), derived metrics, and display-ready aggregates.
5. **Adapter-friendly output** for both Radix/Shadcn dashboards and classic form layouts.
6. **Observability** hooks (plan text + DuckDB explain) so adapters and controllers can surface execution lineage in UI/debug logs.

## Architecture

```
DataModel  --view()-->  DataView  --execute-->  SemanticResult
                                     |                        
                                     v                         
                              SemanticHydrator --------> FormModel
                                     |                         |
                                     v                         v
                              HydratorSnapshot        Controller.render()
```

### Key Components

| Component | Responsibility |
|-----------|----------------|
| `SemanticDataService` | Thin wrapper that executes a `DataView` (Polars, SQL, mock JSON). Returns `SemanticResult` with rows + plan. |
| `SemanticHydrator` | Core orchestrator. Accepts a `HydratorSpec` describing view factories, field bindings, derived metrics, and list mappings. Produces a `HydratorSnapshot`. |
| `HydratorSnapshot` | Immutable object containing `plan`, `rows`, `metrics`, `collections`, and `formAssignments`. |
| `FormBinding` | Declarative mapping between semantic fields (column paths) and FormModel field IDs. |
| `CollectionBinding` | Describes array outputs (e.g., invoices) with optional shaping (select columns, rename, limit). |
| `MetricBinding` | Shorthand for measures (`sum`, `avg`, etc.) that populate hidden numeric fields or KPI cards. |

## API

```ts
export interface HydratorSpec {
  id: string;
  view: (ctx: HydratorContext) => DataView;
  fields?: FormBinding[];
  collections?: CollectionBinding[];
  metrics?: MetricBinding[];
  metaFields?: Record<string, string>;
}

export class SemanticHydrator {
  constructor(private readonly service: SemanticDataService) {}

  async hydrate(
    model: FormModel,
    spec: HydratorSpec,
    ctx: HydratorContext = {}
  ): Promise<HydratorSnapshot>;
}
```

### Example Spec

```ts
const customerProfileSpec: HydratorSpec = {
  id: 'customer-profile',
  view: ({ params }) => CustomerModel.view({
    filter: { id: params?.customerId },
    aggregate: ['totalRevenue', 'averageInvoice', 'count'],
    limit: 1,
  }),
  fields: [
    { fieldId: 'id', source: 'id' },
    { fieldId: 'name', source: 'name' },
    { fieldId: 'email', source: 'email' },
  ],
  collections: [
    { id: 'invoices', source: 'invoices', fieldId: 'invoices' },
  ],
  metrics: [
    { name: 'invoiceCount', source: 'metrics.invoiceCount', fieldId: 'invoiceCount' },
    { name: 'totalRevenue', source: 'metrics.totalRevenue', fieldId: 'totalRevenue' },
  ],
};
```

## Binding Details

### FormBinding

```ts
interface FormBinding {
  fieldId: string;            // FormModel field to populate
  source: string;             // column key, dotted path, or metric name
  transform?: (value: any, row: RowLike) => unknown;
  fallback?: unknown;
}
```

### CollectionBinding

```ts
interface CollectionBinding {
  id: string;                 // snapshot key & optional form field id
  source?: string;            // column containing nested JSON/array
  select?: string[];          // columns to pick from rows
  limit?: number;
  fieldId?: string;           // optional FormModel field
}
```

### MetricBinding

```ts
interface MetricBinding {
  name: string;
  source: string;             // measure column
  fieldId?: string;           // optional hidden field
  format?: 'currency' | 'number' | ((value: number) => string);
}
```

## Snapshot Structure

```ts
interface HydratorSnapshot {
  id: string;
  plan: string;               // Semantic plan + DuckDB explain
  rows: RowLike[];            // Arrow → JSON rows
  metrics: Record<string, unknown>;
  collections: Record<string, RowLike[]>;
  assignedFields: string[];   // Form field IDs touched
  timestamp: number;
  meta?: Record<string, unknown>;
}
```

## Controller Integration

1. **Register Spec**: Each controller exports a `HydratorSpec` describing its semantic requirements.
2. **Compose Hydrator**: Instantiate `SemanticHydrator` with a `SemanticDataService`.
3. **Hydrate on load**: In controller constructors or `loadProfile` methods, call `hydrator.hydrate(model, spec, ctx)`.
4. **Persist Snapshot**: Store the returned snapshot on the controller for view rendering.
5. **Adapter render**: Pass `{ snapshot }` into adapter context for KPI cards/tables.

## Execution Layer

The default `SemanticDataService` wraps the `PolarsExecutionEngine`:

1. **Dataset** — Data service builds an in-memory dataset and exposes it as Arrow-compatible JSON.
2. **Arrow → Polars** — `PolarsExecutionEngine` converts rows into Arrow Tables, then Polars DataFrames.
3. **Aggregation** — Frames are filtered, grouped, and joined via Polars expressions.
4. **Observability** — Results include the semantic plan plus DuckDB `EXPLAIN` string.
5. **Return** — Engine returns enriched rows + meta, which `SemanticHydrator` maps into FormModel.

## Testing

- `test/semantic-hydrator.test.ts` — Field/collection/metric bindings
- `test/polars-engine.test.ts` — Polars/DuckDB execution path
- `test/radix-adapter.test.tsx` — Snapshot flow to Radix adapter

