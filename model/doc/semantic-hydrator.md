# Semantic Hydrator Design

## Goals

1. **Reusable bridge** between the Data SDSL (Malloy-style semantic models) and the MVC Form stack (FormModel + ReactController).
2. **Zero-Prisma** data path that works with any execution engine (Polars, SQL, mock JSON) and produces field-ready values.
3. **Deterministic mappings** so controllers can describe how semantic columns populate form fields, action payloads, and derived KPIs.
4. **Composable snapshots** that bundle the semantic plan, raw rows (Arrow/JSON), derived metrics, and display-ready aggregates.
5. **Adapter-friendly output** for both Radix/Shadcn dashboards and classic form layouts.
6. **Observability** hooks (plan text + DuckDB explain) so adapters and controllers can surface execution lineage in UI/debug logs.

## Constraints & Requirements

- Must operate without synchronous DB access on the server—hydration should run in controllers/server actions.
- Needs to support **single-record** loads (e.g., customer profile) plus **collections** (invoice lists) and **measures** (totals, counts).
- Should accept either a prepared `DataView` or a factory function so controllers can inject filters (customerId, time range).
- Must expose **typed hooks** for post-processing (currency formatting, percentage derivations) before writing to the FormModel.
- Should cache/stage the semantic payload so adapters (Radix) can render tables/cards without re-running the query.

## Proposed Architecture

```
DataModel  --view()-->  DataView  --execute-->  SemanticResult
                                      |                        
                                      v                         
                               SemanticHydrator --------> FormModel
                                      |                         |
                                      v                         v
                               HydratorSnapshot        ReactController.reactView
```

### Key Components

| Component | Responsibility |
|-----------|----------------|
| `SemanticDataService` | Thin wrapper that knows how to execute a `DataView` (Polars, SQL, mock JSON). Returns `SemanticResult` with rows + plan. In practice this is backed by the Polars engine + Arrow buffers. |
| `SemanticHydrator` | Core orchestrator. Accepts a `HydratorSpec` describing view factories, field bindings, derived metrics, and list mappings. Produces a `HydratorSnapshot`. |
| `HydratorSnapshot` | Immutable object containing `plan`, `rows`, `metrics`, `collections`, and `formAssignments`. Stored on controller/model for adapters. |
| `FormBinding` | Declarative mapping between semantic fields (column paths) and FormModel field IDs, including transforms. |
| `CollectionBinding` | Describes array outputs (e.g., invoices) with optional shaping (select columns, rename, limit). |
| `MetricBinding` | Shorthand for measures (`sum`, `avg`, etc.) that should populate hidden numeric fields or KPI cards. |

## API Sketch

```ts
export interface SemanticDataService {
  execute(view: DataView, options?: ExecutionOptions): Promise<SemanticResult>;
}

export interface HydratorSpec {
  id: string;
  view: (ctx: HydratorContext) => DataView;
  fields?: FormBinding[];
  collections?: CollectionBinding[];
  metrics?: MetricBinding[];
  metaFields?: Record<string, string>; // e.g., semanticPlan -> form field id
}

export class SemanticHydrator {
  constructor(private readonly service: SemanticDataService) {}

  async hydrate(
    model: FormModel,
    spec: HydratorSpec,
    ctx: HydratorContext = {}
  ): Promise<HydratorSnapshot> {
    const view = spec.view(ctx);
    const result = await this.service.execute(view, ctx.execution);
    // bind fields + build snapshot (see src/data/semantic-hydrator.ts)
  }
}
```

### Example Spec (Customer Profile)

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
  metaFields: { '$plan': 'semanticPlan' },
};

const snapshot = await hydrator.hydrate(formModel, customerProfileSpec, {
  params: { customerId: 'cust_100' },
});
```

### Binding Details

- **FormBinding**
  ```ts
  interface FormBinding {
    fieldId: string;            // FormModel field to populate
    source: string;             // column key, dotted path, or metric name
    transform?: (value: any, row: RowLike) => unknown;
    fallback?: unknown;
  }
  ```

- **CollectionBinding**
  ```ts
  interface CollectionBinding {
    id: string;                 // snapshot key & optional form field id
    source?: string;            // column containing nested JSON/array
    select?: string[];          // columns to pick from rows
    limit?: number;
    fieldId?: string;           // optional FormModel field to set (hidden JSON)
  }
  ```

- **MetricBinding**
  ```ts
  interface MetricBinding {
    name: string;
    source: string;             // measure column
    fieldId?: string;           // optional hidden field
    format?: 'currency' | 'number' | ((value: number) => string);
  }
  ```

## Controller Integration Steps

1. **Register Spec**: Each controller exports a `HydratorSpec` describing its semantic requirements (customer profile, invoice list, etc.).
2. **Compose Hydrator**: Instantiate `SemanticHydrator` with a `SemanticDataService` (mock, SQL, Polars).
3. **Hydrate on load**: In controller constructors or dedicated `loadProfile` methods, call `hydrator.hydrate(controller, spec, ctx)`.
4. **Persist Snapshot**: Store the returned snapshot on the controller (`this.semanticSnapshot = snapshot;`) so `ReactView`/Radix adapter can render KPIs without re-querying.
5. **Expose to client**: Server actions can serialize the snapshot (plan + metrics) for optimistic UI or debugging.
6. **Adapter render**: Pass `{ snapshot, handler }` into `radixAdapter.render(document, context)` to hydrate KPI cards/tables.

## Snapshot Structure
```ts
interface HydratorSnapshot {
  id: string;
  plan: string;          // Semantic plan + DuckDB explain (JSON string)
  rows: RowLike[];       // Arrow → JSON rows returned by execution engine
  metrics: Record<string, unknown>;
  collections: Record<string, RowLike[]>;
  assignedFields: string[]; // Form field IDs touched
  timestamp: number;
  meta?: Record<string, unknown>;
}
```

The snapshot lives on the controller (`CustomerController.getSemanticSnapshot()`) so views/adapters can access collections and plan metadata without another fetch.

## Execution Layer (Polars + Arrow + DuckDB)

The default `SemanticDataService` for MVC examples wraps the shared `PolarsExecutionEngine`:

1. **Dataset** – `CustomerDataService` builds an in-memory dataset (customers + invoices) and exposes it as Arrow-compatible JSON.
2. **Arrow → Polars** – `PolarsExecutionEngine` converts rows into Arrow Tables, then into Polars DataFrames.
3. **Aggregation** – customer + invoice frames are filtered, grouped, and joined via Polars expressions.
4. **Observability** – results include the semantic plan plus a lazily generated DuckDB `EXPLAIN` string (if the native binding is available).
5. **Return** – the engine returns enriched rows + meta, which `SemanticHydrator` maps into the FormModel and Radix snapshot.

This keeps the hydrator decoupled from the underlying engine—swap in DuckDB/SQL or remote APIs by providing a different `SemanticDataService` implementation.
```

## Error Handling

- Hydrator should surface semantic errors with context (view id, filter payload). Controllers can map to `OperationResult` errors.
- Partial hydration: if metrics succeed but collection fails, snapshot should capture partial state + reason.

## Testing & Regression Coverage

- `model/test/semantic-hydrator.test.ts` covers field/collection/metric bindings and snapshot metadata.
- `model/test/polars-engine.test.ts` exercises the Polars/DuckDB execution path and verifies metrics/collections.
- `model/test/radix-adapter.test.tsx` confirms snapshots flow into the Radix adapter so KPI cards/tables render semantic data.

Add new specs by composing a `HydratorSpec` + `SemanticDataService` combo, then reuse the same regression harness to guarantee consistency.
```}
