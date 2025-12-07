# Execution Stack: SDSL + Polars + DuckDB

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│ MVC / SDSL (TypeScript)                                 │
│ - Semantic data modeling layer                          │
│ - DataModel, DataView, measures, dimensions             │
└─────────────────────────────────────────────────────────┘
                         ↓
┌─────────────────────────────────────────────────────────┐
│ Execution Layer                                         │
│                                                         │
│ ┌──────────────┐   ┌──────────────┐   ┌──────────────┐ │
│ │ nodejs-polars│   │ DuckDB       │   │ Postgres     │ │
│ │ DataFrame    │   │ SQL Engine   │   │ Database     │ │
│ │ Operations   │   │ Analytics    │   │ Backend      │ │
│ └──────────────┘   └──────────────┘   └──────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Execution Engines

### nodejs-polars (DataFrame Operations)

**What it provides:**
- Fast DataFrame operations
- Arrow-native
- TypeScript bindings
- No custom C++ needed

**Use for:**
- In-memory analytics
- Data transformations
- Feature engineering
- ETL operations

### DuckDB (SQL Execution)

**What it provides:**
- Fast SQL execution
- Embedded database
- Arrow-native
- Query planning (EXPLAIN)

**Use for:**
- Complex SQL queries
- Aggregations
- Analytical queries
- Query plan observability

### Postgres (Database Backend)

**What it provides:**
- Persistent storage
- ACID transactions
- Production-ready

**Use for:**
- Application data storage
- Persistent state
- Multi-user access

## SDSL: Semantic Layer

### DataModel and DataView

```typescript
import { defineModel, sum, count, avg } from '@model/data';

const customerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  fields: {
    id: z.string(),
    name: z.string(),
    region: z.string(),
    revenue: z.number(),
  },
  measures: {
    total_revenue: sum('revenue'),
    customer_count: count(),
    avg_revenue: avg('revenue'),
  },
  dimensions: {
    region: 'region',
  },
});

// Create a view (query)
const view = customerModel.view({
  group_by: ['region'],
  aggregate: ['total_revenue', 'customer_count'],
  filter: { region: 'North' },
  limit: 100,
});

// Execute with PolarsExecutionEngine
const result = await engine.execute(view);
```

### PolarsExecutionEngine

```typescript
const engine = new PolarsExecutionEngine(dataset);
const result = await engine.execute(view);

// Result includes:
// - rows: Array of result records
// - plan: Query plan (from DuckDB EXPLAIN)
// - meta: Execution metadata
```

## Data Flow

```
SDSL Model Definition
    ↓
DataView (query spec)
    ↓
PolarsExecutionEngine
    ├─ Compiles to Polars operations
    ├─ Uses DuckDB for EXPLAIN
    └─ Returns Arrow-native results
    ↓
SemanticResult
    ↓
SemanticHydrator
    ↓
FormModel (populated)
```

## Key Insights

1. **No custom C++ NAPI** — nodejs-polars provides performance
2. **SDSL is our semantic layer** — DataModel/DataView API
3. **Multiple engines** — Route based on operation type
4. **TypeScript-first** — High-level code stays in TypeScript
5. **Arrow interop** — Efficient data transfer between engines

