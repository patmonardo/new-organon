# Language Comparison: Malloy, Polars, OpenCypher, and GDSL

## Executive Summary

We need to understand how these languages relate to GDSL and where the Workflow AST fits:

1. **Malloy** - Semantic modeling language (Google)
2. **Polars** - DataFrame execution engine (Apache Arrow)
3. **OpenCypher** - Graph query language (Neo4j)
4. **GDSL** - Our graph data science language
5. **Workflow AST** - Standard workflow representation (to be incorporated)

## Malloy Architecture

### Core Concepts

**Malloy is a semantic modeling language** inspired by LookML:

```malloy
source: customers is table('customers') {
  measure: total_revenue is sum(orders.amount)
  measure: order_count is count()
  
  dimension: region is region
  dimension: signup_month is signup_date.month
  
  join: orders is table('orders') on id = orders.customer_id
}
```

**Key Characteristics:**
- ✅ **Semantic layer first** - Models define measures, dimensions, joins
- ✅ **Declarative queries** - Views are queries against models
- ✅ **SQL generation** - Compiles to SQL (BigQuery, Postgres, etc.)
- ✅ **Nested queries** - Views can compose other views
- ✅ **Time intelligence** - Built-in date/time handling

### Execution Model

```
Malloy Model (Semantic)
    ↓
Malloy View (Query)
    ↓
SQL Compiler
    ↓
SQL (BigQuery/Postgres/etc.)
    ↓
Execution Engine
```

**Foundation**: Semantic modeling language → SQL generation

## Polars Architecture

### Core Concepts

**Polars is a DataFrame execution engine** built on Apache Arrow:

```python
import polars as pl

df = pl.DataFrame({
    'customer_id': [1, 2, 3],
    'amount': [100, 200, 300]
})

result = df.group_by('customer_id').agg([
    pl.sum('amount').alias('total_revenue'),
    pl.count().alias('order_count')
])
```

**Key Characteristics:**
- ✅ **Execution-first** - DataFrame operations are the API
- ✅ **Lazy evaluation** - Builds query plan, executes on demand
- ✅ **Arrow-native** - Columnar memory format
- ✅ **Multi-threaded** - Parallel execution
- ✅ **No semantic layer** - Direct DataFrame operations

### Execution Model

```
Polars DataFrame (Data)
    ↓
Polars Expressions (Operations)
    ↓
Query Plan (Lazy)
    ↓
Execution Engine (Arrow)
    ↓
Result DataFrame
```

**Foundation**: Execution engine → DataFrame API

## OpenCypher Architecture

### Core Concepts

**OpenCypher is a graph query language** (open specification of Neo4j's Cypher):

```cypher
MATCH (c:Customer)-[r:ORDERED]->(o:Order)
WHERE c.region = 'west'
RETURN c.name, sum(o.amount) as total_revenue, count(o) as order_count
```

**Key Characteristics:**
- ✅ **Pattern matching** - Graph patterns are primary
- ✅ **Language-first** - Query language is foundational
- ✅ **Declarative** - Describe what, not how
- ✅ **Graph-native** - Nodes, relationships, properties
- ✅ **AST-based** - Execution built on Cypher AST

### Execution Model

```
Cypher Query (Language)
    ↓
Cypher AST (IR)
    ↓
Query Planner
    ↓
Execution Engine (Neo4j)
    ↓
Result Graph/Table
```

**Foundation**: Query language → AST → Execution

## Comparison Matrix

| Aspect | Malloy | Polars | OpenCypher | GDSL (Current) |
|--------|--------|--------|------------|----------------|
| **Foundation** | Semantic model | Execution engine | Query language | Dialectic state → Cypher AST |
| **IR** | Malloy AST → SQL | Polars plan | Cypher AST | Cypher AST |
| **Primary Op** | Measure/dimension | DataFrame op | Pattern match | Pattern match |
| **Semantic Layer** | ✅ Yes (measures/dims) | ❌ No | ❌ No | ❌ No (yet) |
| **Execution** | SQL → DB | Arrow → Polars | Cypher → Neo4j | Cypher → Neo4j |
| **Composition** | Nested views | Lazy chains | Pattern composition | Dialectic states |
| **Type** | Modeling language | Execution engine | Query language | Graph DSL |

## GDSL's Current Position

### What We Have

**GDSL = OpenCypher** - The graph query language IS GDSL itself.

```
OpenCypher (GDSL)
    ↓
GDS Init/Task Daemon
    ↓
GDS Kernel Components
    ↓
RootAgent
```

**Current State:**
- ✅ GDSL is OpenCypher - graph query language for kernel operations
- ✅ Rust-based - Graph/ML algorithm oriented
- ✅ Kernel operations - Low-level graph/ML algorithms
- ⚠️ Need to clarify GDS Init/Task Daemon architecture

**SDSL (Separate Layer):**
- ✅ Malloy-inspired - TypeScript data modeling
- ✅ Polars/Arrow - Execution engine
- ✅ MVC platform - FormApp development

### What We Need

1. **Semantic Layer** (Malloy-inspired)
   - Measures (sum, count, avg)
   - Dimensions (group by, time intelligence)
   - Joins (relationships)

2. **Execution Options**
   - Neo4j (via Cypher)
   - Polars (via DataFrame operations)
   - SQL (via SQL generation)

3. **IR Strategy**
   - GDSL IR (dialectic state) → Multiple backends
   - Not just Cypher AST

## Workflow AST Integration (OpenAPI Components)

### OpenAPI Workflow Components

OpenAPI 3.1 has a **components** section that can describe workflow-like behavior:

- **callbacks** - Async operations, webhooks
- **links** - Operation references, workflow connections
- **operationRef** - Reference to other operations (creates workflow chains)

**Classic Component Workflow Pattern:**
```yaml
components:
  callbacks:
    OrderStatusCallback:
      '{$request.body#/callbackUrl}':
        post:
          operationId: onOrderStatusChange
          requestBody:
            $ref: '#/components/schemas/OrderStatus'
  
  links:
    OrderDetails:
      operationId: getOrder
      parameters:
        orderId: '$response.body#/orderId'
```

This creates **workflow-like structures** where operations reference each other.

### Workflow AST from OpenAPI

The "Workflow AST" likely refers to:
1. **OpenAPI AST structure** - The parsed OpenAPI spec as an AST
2. **Component workflow patterns** - Using callbacks/links/operationRef to create workflows
3. **Operation orchestration** - Operations that reference other operations

### Integration with GDSL

**Possible Positions:**

1. **OpenAPI Components → GDSL Procedures**
   ```
   OpenAPI Components (callbacks/links)
       ↓
   Workflow AST (operation references)
       ↓
   GDSL Procedures
       ↓
   Execution
   ```

2. **GDSL Queries as OpenAPI Operations**
   ```
   GDSL Query
       ↓
   OpenAPI Operation
       ↓
   Workflow Component (operationRef)
       ↓
   Orchestration
   ```

3. **Workflow AST Orchestrates GDSL**
   ```
   Workflow AST (from OpenAPI)
       ├─→ GDSL Query 1 → Neo4j
       ├─→ GDSL Query 2 → Polars
       └─→ GDSL Procedure → GDS Runtime
   ```

### Current Workflow Implementation

Looking at `task/src/schema/workflow.ts`, we have:
- **Process** - Steps, dependencies, dialectical roles
- **Coordination** - Rules, synchronization, conflict resolution
- **Workflow** - Synthesis of process and coordination

This could align with OpenAPI's component workflow pattern.

### Integration Points

1. **OpenAPI → Workflow AST → GDSL Procedures**
   - Parse OpenAPI components
   - Extract workflow patterns (callbacks/links)
   - Map to GDSL procedure calls

2. **GDSL Queries as OpenAPI Operations**
   - Expose GDSL queries as OpenAPI operations
   - Use operationRef to chain queries
   - Create workflow from operation references

3. **Workflow AST Orchestration**
   - Workflow AST coordinates multiple GDSL operations
   - Uses OpenAPI component patterns for structure
   - Executes via GDSL execution engines

## Recommendations

### 1. Adopt Malloy-Style Semantic Layer

Add to GDSL:
```typescript
interface GDSLModel {
  name: string;
  source: string;
  measures: Record<string, MeasureDefinition>;
  dimensions: Record<string, DimensionDefinition>;
  joins: Record<string, JoinDefinition>;
}
```

### 2. Multi-Backend IR

Don't commit to Cypher AST only:
```
Dialectic State (GDSL IR)
    ├─→ Cypher AST → Neo4j
    ├─→ Polars Plan → Polars
    └─→ SQL → Postgres/DuckDB
```

### 3. Workflow AST as Orchestration

Workflow AST orchestrates GDSL operations:
```
Workflow AST
    ├─→ GDSL Query 1 → Neo4j
    ├─→ GDSL Query 2 → Polars
    └─→ GDSL Procedure → GDS Runtime
```

## Next Steps

- [ ] Study Malloy's semantic model structure
- [ ] Design GDSL semantic layer (measures/dimensions)
- [ ] Create multi-backend IR (not just Cypher)
- [ ] Investigate Workflow AST standard
- [ ] Design Workflow AST → GDSL integration
- [ ] Bridge Polars (execution-first) with GDSL (language-first)

