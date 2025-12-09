# SDSL as Species DSL: Relationship to GDSL (Genera DSL)

## The Genera-Species Relationship

### Core Concept

- **GDSL = Genera DSL** (Graph Data Specification Language)
  - The **general, abstract foundation**
  - Rust-based, graph/ML operations
  - Primary target (AI Platform)

- **SDSL = Species DSL** (Species Data Specification Language)
  - The **specific, concrete implementation**
  - TypeScript-based, BI/empirical data
  - Secondary target (BI Platform)

### The Relationship

```
GDSL (Genera) - Abstract/Principle
    ↓
    │ inherits Abstract/Principle
    ↓
SDSL (Species) - Concrete/Specs
    │
    └─→ All based on Specs
```

**Key Insight**: 
- SDSL has its **Abstract/Principle** from GDSL
- But then it's **all based on Specs** (the specific implementation)
- The model should look more like **Rust GDS** than **Prisma**

## Rust GDS Structure (Genera)

### Core Modules

```
gds/src/
├── collections/        # Data structures (Huge, Vec, Arrow)
├── config/            # Configuration system (builder pattern)
├── core/              # Core abstractions
├── projection/        # Projection system (GDSL Runtime)
│   ├── factory/       # Data ingestion (CAR - given data)
│   ├── eval/          # Execution runtime (CDR - derived computations)
│   └── codegen/        # Code generation
├── procedures/        # Algorithm procedures
├── types/             # Type system
└── values/            # Value system
```

### Key Patterns from Rust GDS

1. **Spec-Based Configuration**
   - Builder pattern
   - Type-safe configs
   - Runtime validation

2. **Projection System**
   - Factory (data ingestion)
   - Eval (execution runtime)
   - Codegen (code generation)

3. **Type System**
   - Strong typing
   - Property system
   - Schema definitions

4. **Collections**
   - Unified API
   - Multiple backends (Huge, Vec, Arrow)
   - Adapter pattern

## SDSL Structure (Species) - Based on Rust GDS

### Proposed Structure

```
model/src/
├── sdsl/              # Species DSL Core
│   ├── core/          # Core abstractions (from GDSL)
│   ├── config/        # Configuration system (builder pattern)
│   ├── projection/    # Projection system (SDSL Runtime)
│   │   ├── factory/   # Data ingestion (Malloy sources)
│   │   ├── eval/      # Execution runtime (query execution)
│   │   └── codegen/   # Code generation (query plans)
│   ├── types/         # Type system (measures, dimensions)
│   ├── values/        # Value system (data values)
│   └── procedures/    # BI procedures (aggregations, etc.)
│
├── data/              # Data layer
│   ├── sdsl.ts        # SDSL core (Species DSL)
│   ├── semantic-hydrator.ts
│   └── polars-engine.ts
│
└── schema/            # Schema definitions
    ├── malloy-view.ts
    └── dashboard.ts
```

## Abstract/Principle from GDSL

### 1. Configuration System

**GDSL Pattern** (Rust):
```rust
// config/algo_config.rs
pub struct PageRankConfig {
    max_iterations: u32,
    damping_factor: f64,
    tolerance: f64,
}

impl PageRankConfig {
    pub fn builder() -> PageRankConfigBuilder {
        PageRankConfigBuilder::default()
    }
}
```

**SDSL Pattern** (TypeScript - Species):
```typescript
// sdsl/config/model-config.ts
export interface DataModelConfig<T> {
  name: string;
  source: SourceDefinition;
  primary_key?: string | string[];
  measures?: Record<string, MeasureDefinition>;
  dimensions?: Record<string, DimensionDefinition>;
  joins?: Record<string, JoinDefinition>;
  where?: FilterDefinition[];
}

export class DataModelConfigBuilder<T> {
  private config: Partial<DataModelConfig<T>> = {};

  name(name: string): this {
    this.config.name = name;
    return this;
  }

  source(source: SourceDefinition): this {
    this.config.source = source;
    return this;
  }

  measure(name: string, definition: MeasureDefinition): this {
    this.config.measures = this.config.measures || {};
    this.config.measures[name] = definition;
    return this;
  }

  build(): DataModelConfig<T> {
    // Validation
    if (!this.config.name) throw new Error('name required');
    if (!this.config.source) throw new Error('source required');
    return this.config as DataModelConfig<T>;
  }
}
```

### 2. Projection System

**GDSL Pattern** (Rust):
```rust
// projection/mod.rs
// Factory (CAR - given data): Ingestion
// Eval (CDR - derived computations): Execution
// Codegen: Code generation
```

**SDSL Pattern** (TypeScript - Species):
```typescript
// sdsl/projection/factory.ts - Data Ingestion (CAR)
export class DataModelFactory {
  fromTable(name: string, table: string): DataModel {
    // Ingest from table
  }

  fromSQL(name: string, sql: string): DataModel {
    // Ingest from SQL
  }

  fromArrow(name: string, arrow: ArrowTable): DataModel {
    // Ingest from Arrow
  }
}

// sdsl/projection/eval.ts - Execution Runtime (CDR)
export class QueryEvaluator {
  async evaluate(view: DataView, context: ExecutionContext): Promise<SemanticResult> {
    // Execute query
  }
}

// sdsl/projection/codegen.ts - Code Generation
export class QueryCodegen {
  generatePlan(view: DataView): ExecutionPlan {
    // Generate execution plan
  }
}
```

### 3. Type System

**GDSL Pattern** (Rust):
```rust
// types/properties/ - Property system
// types/schema/ - Schema definitions
```

**SDSL Pattern** (TypeScript - Species):
```typescript
// sdsl/types/measures.ts
export interface MeasureDefinition {
  type: 'sum' | 'count' | 'avg' | 'min' | 'max' | 'custom';
  field?: string;
  sql?: string;
  label?: string;
}

// sdsl/types/dimensions.ts
export interface DimensionDefinition {
  field: string;
  truncation?: 'year' | 'quarter' | 'month' | 'day' | 'hour';
  label?: string;
}

// sdsl/types/schema.ts
export interface DataModelSchema {
  name: string;
  source: SourceDefinition;
  primary_key?: string | string[];
  measures: Record<string, MeasureDefinition>;
  dimensions: Record<string, DimensionDefinition>;
  joins?: Record<string, JoinDefinition>;
}
```

### 4. Collections/Values

**GDSL Pattern** (Rust):
```rust
// collections/ - Unified collections API
// values/ - Value system
```

**SDSL Pattern** (TypeScript - Species):
```typescript
// sdsl/values/data-values.ts
export type DataValue = string | number | boolean | Date | null;

export interface DataRow {
  [key: string]: DataValue;
}

export interface DataTable {
  columns: string[];
  rows: DataRow[];
  schema: DataModelSchema;
}

// sdsl/collections/arrow-collection.ts
export class ArrowCollection {
  fromArrow(arrow: ArrowTable): DataTable {
    // Convert Arrow to DataTable
  }

  toArrow(table: DataTable): ArrowTable {
    // Convert DataTable to Arrow
  }
}
```

## All Based on Specs

### Spec-Driven Architecture

The SDSL model is **all based on Specs** - configuration-driven, not code-driven:

```typescript
// Spec-based model definition
const CustomerModelSpec: DataModelSpec = {
  name: 'Customer',
  source: {
    type: 'table',
    value: 'customers',
  },
  primary_key: 'id',
  measures: {
    totalRevenue: {
      type: 'sum',
      field: 'revenue',
    },
    customerCount: {
      type: 'count',
    },
  },
  dimensions: {
    region: {
      field: 'region',
    },
    signupMonth: {
      field: 'createdAt',
      truncation: 'month',
    },
  },
  joins: {
    invoices: {
      model: 'Invoice',
      on: 'customers.id = invoices.customerId',
      type: 'left',
    },
  },
};

// Spec → Model
const CustomerModel = DataModel.fromSpec(CustomerModelSpec);
```

### View Specs

```typescript
// View spec
const RevenueByRegionViewSpec: ViewSpec = {
  id: 'revenue-by-region',
  model: 'Customer',
  group_by: ['region'],
  aggregate: ['totalRevenue', 'customerCount'],
  where: [
    {
      field: 'region',
      operator: '!=',
      value: null,
    },
  ],
  limit: 100,
  order_by: [
    {
      field: 'totalRevenue',
      direction: 'desc',
    },
  ],
};

// Spec → View
const view = DataModel.fromSpec(CustomerModelSpec)
  .view(RevenueByRegionViewSpec);
```

### Dashboard Specs

```typescript
// Dashboard spec
const CustomerDashboardSpec: DashboardSpec = {
  id: 'customer-dashboard',
  layout: {
    gridColumns: 12,
  },
  views: [
    {
      viewId: 'revenue-by-region',
      position: { x: 0, y: 0, w: 6, h: 4 },
    },
    {
      viewId: 'customer-growth',
      position: { x: 6, y: 0, w: 6, h: 4 },
    },
  ],
  filters: [
    {
      field: 'region',
      operator: 'in',
      value: ['North', 'South'],
    },
  ],
  parameters: [
    {
      name: 'startDate',
      type: 'date',
      default: new Date('2024-01-01'),
    },
  ],
};
```

## Comparison: Rust GDS vs Prisma

### Rust GDS Pattern (What We Want)

```rust
// Spec-based, configuration-driven
let config = PageRankConfig::builder()
    .max_iterations(50)
    .damping_factor(0.9)
    .build()?;

// Execute via projection system
let result = projection::eval::execute(config)?;
```

### Prisma Pattern (What We're Moving Away From)

```typescript
// Code-driven, ORM-based
const user = await prisma.user.findUnique({
  where: { id: userId },
  include: { posts: true },
});
```

### SDSL Pattern (Species - Based on Rust GDS)

```typescript
// Spec-based, configuration-driven (like Rust GDS)
const modelSpec: DataModelSpec = {
  name: 'Customer',
  source: { type: 'table', value: 'customers' },
  measures: {
    totalRevenue: { type: 'sum', field: 'revenue' },
  },
};

const model = DataModel.fromSpec(modelSpec);

// Execute via projection system (like Rust GDS)
const viewSpec: ViewSpec = {
  model: 'Customer',
  group_by: ['region'],
  aggregate: ['totalRevenue'],
};

const result = await projection.eval.execute(model.view(viewSpec));
```

## Architecture: Abstract/Principle → Specs

### The Flow

```
GDSL (Genera) - Abstract/Principle
    │
    │ Provides:
    │ - Configuration patterns
    │ - Projection system
    │ - Type system
    │ - Execution model
    │
    ↓
SDSL (Species) - Concrete/Specs
    │
    │ Implements:
    │ - DataModel specs
    │ - View specs
    │ - Dashboard specs
    │ - Execution specs
    │
    └─→ All based on Specs
```

### Implementation Strategy

1. **Abstract/Principle from GDSL**
   - Configuration system (builder pattern)
   - Projection system (factory/eval/codegen)
   - Type system (measures, dimensions)
   - Execution model (spec-based)

2. **All Based on Specs**
   - DataModelSpec
   - ViewSpec
   - DashboardSpec
   - ExecutionSpec

3. **Rust GDS-Inspired Structure**
   - Spec-based configuration
   - Projection system
   - Type-safe
   - Builder pattern
   - Runtime execution

## Next Steps

1. **Refactor SDSL to Spec-Based**
   - DataModelSpec
   - ViewSpec
   - DashboardSpec

2. **Implement Projection System**
   - Factory (data ingestion)
   - Eval (execution runtime)
   - Codegen (query plans)

3. **Configuration System**
   - Builder pattern
   - Type-safe configs
   - Runtime validation

4. **Type System**
   - Measures
   - Dimensions
   - Joins
   - Filters

---

**Key Insight**: SDSL (Species DSL) inherits its Abstract/Principle from GDSL (Genera DSL), but then it's all based on Specs - configuration-driven, spec-based architecture inspired by Rust GDS, not Prisma.

