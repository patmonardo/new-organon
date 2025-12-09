# Standalone Malloy BI Package

## Goal

Create a **standalone Malloy BI package** that:
- **Zero dependencies** on GDS/GDSL/Logic/Task
- **BI-only** focus (Model-View-Dashboard)
- **Runs standalone** - can be used independently
- **Malloy-inspired** semantic modeling

## Architecture

### Standalone Structure

```
@organon/model (Standalone Malloy BI)
├── src/
│   ├── malloy/              # Malloy BI Core
│   │   ├── model/           # DataModel (Malloy Source)
│   │   ├── view/            # MalloyView (Query Definition)
│   │   ├── dashboard/       # Dashboard (View Collection)
│   │   └── spec/            # Spec types (DataModelSpec, ViewSpec, DashboardSpec)
│   │
│   ├── execution/            # Execution Engine
│   │   ├── polars-engine.ts # Polars execution
│   │   ├── duckdb-engine.ts # DuckDB execution
│   │   └── arrow-engine.ts  # Arrow execution
│   │
│   ├── schema/              # Zod schemas
│   │   ├── model-spec.ts
│   │   ├── view-spec.ts
│   │   └── dashboard-spec.ts
│   │
│   └── index.ts             # Public API
│
└── package.json             # Zero dependencies on GDS/GDSL/Logic/Task
```

### Zero Dependencies

**Allowed:**
- `zod` - Schema validation
- `nodejs-polars` - Execution engine
- `apache-arrow` - Data format
- `duckdb` - SQL execution
- `react` / `react-dom` - UI (devDependencies)

**Not Allowed:**
- `@organon/gds` - No GDS dependency
- `@organon/gdsl` - No GDSL dependency
- `@organon/logic` - No Logic dependency
- `@organon/task` - No Task dependency

## Core Components

### 1. Malloy Model (DataModel)

```typescript
// src/malloy/model/data-model.ts
export interface DataModelSpec {
  name: string;
  source: {
    type: 'table' | 'sql' | 'connection';
    value: string;
  };
  primary_key?: string | string[];
  measures?: Record<string, MeasureDefinition>;
  dimensions?: Record<string, DimensionDefinition>;
  joins?: Record<string, JoinDefinition>;
  where?: FilterDefinition[];
}

export class DataModel {
  constructor(public spec: DataModelSpec) {}
  
  view(viewSpec: ViewSpec): DataView {
    return new DataView(this, viewSpec);
  }
  
  static fromSpec(spec: DataModelSpec): DataModel {
    return new DataModel(spec);
  }
}
```

### 2. Malloy View (Query Definition)

```typescript
// src/malloy/view/malloy-view.ts
export interface ViewSpec {
  id: string;
  name?: string;
  model: string; // Reference to DataModel name
  group_by?: string[];
  aggregate?: string[];
  where?: FilterDefinition[];
  limit?: number;
  order_by?: OrderDefinition[];
}

export class MalloyView {
  constructor(
    public model: DataModel,
    public spec: ViewSpec
  ) {}
  
  async execute(engine: ExecutionEngine): Promise<QueryResult> {
    return await engine.execute(this);
  }
}
```

### 3. Dashboard (View Collection)

```typescript
// src/malloy/dashboard/dashboard.ts
export interface DashboardSpec {
  id: string;
  name: string;
  layout: DashboardLayout;
  views: ViewReference[];
  filters?: DashboardFilter[];
  parameters?: DashboardParameter[];
}

export class Dashboard {
  constructor(
    public spec: DashboardSpec,
    private models: Map<string, DataModel>
  ) {}
  
  async render(engine: ExecutionEngine): Promise<DashboardResult> {
    // Execute all views
    const results = new Map();
    for (const viewRef of this.spec.views) {
      const model = this.models.get(viewRef.model);
      const view = model.view(viewRef.viewSpec);
      const result = await view.execute(engine);
      results.set(viewRef.id, result);
    }
    return { views: results, layout: this.spec.layout };
  }
}
```

### 4. Execution Engine

```typescript
// src/execution/polars-engine.ts
export interface ExecutionEngine {
  execute(view: MalloyView): Promise<QueryResult>;
}

export class PolarsExecutionEngine implements ExecutionEngine {
  async execute(view: MalloyView): Promise<QueryResult> {
    // Execute via Polars
    // No dependency on Logic/GDS/GDSL
  }
}
```

## Public API

```typescript
// src/index.ts
// Standalone Malloy BI - Zero dependencies on GDS/GDSL/Logic/Task

export {
  // Models
  DataModel,
  type DataModelSpec,
  
  // Views
  MalloyView,
  type ViewSpec,
  
  // Dashboards
  Dashboard,
  type DashboardSpec,
  
  // Execution
  ExecutionEngine,
  PolarsExecutionEngine,
  DuckDBExecutionEngine,
  
  // Helpers
  defineModel,
  sum,
  count,
  avg,
  dimension,
} from './malloy';
```

## Usage Example

```typescript
import { DataModel, MalloyView, Dashboard, PolarsExecutionEngine } from '@organon/model';

// Define model (standalone, no Logic/GDS dependency)
const customerModel = DataModel.fromSpec({
  name: 'Customer',
  source: { type: 'table', value: 'customers' },
  measures: {
    totalRevenue: { type: 'sum', field: 'revenue' },
  },
  dimensions: {
    region: { field: 'region' },
  },
});

// Create view
const view = customerModel.view({
  id: 'revenue-by-region',
  model: 'Customer',
  group_by: ['region'],
  aggregate: ['totalRevenue'],
});

// Execute (standalone)
const engine = new PolarsExecutionEngine();
const result = await view.execute(engine);

// Dashboard
const dashboard = new Dashboard({
  id: 'customer-dashboard',
  name: 'Customer Dashboard',
  layout: { gridColumns: 12 },
  views: [
    { id: 'revenue-by-region', model: 'Customer', viewSpec: {...} },
  ],
}, new Map([['Customer', customerModel]]));

const dashboardResult = await dashboard.render(engine);
```

## Key Principles

1. **Standalone**: Zero dependencies on GDS/GDSL/Logic/Task
2. **BI-Only**: Focus on Model-View-Dashboard
3. **Malloy-Inspired**: Semantic modeling like Malloy
4. **Spec-Based**: All definitions via specs
5. **Rust GDS Patterns**: Configuration-driven, not ORM-driven

## Migration Path

1. **Extract BI components** from current code
2. **Remove Logic/GDS/GDSL references** from docs/code
3. **Create standalone package structure**
4. **Update public API** to be BI-only
5. **Test standalone** - verify zero dependencies

---

**Goal**: A standalone Malloy BI package that can be used independently, with zero dependencies on the AI platform (GDS/GDSL/Logic/Task).

