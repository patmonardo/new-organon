# GDSL Architecture: Three-Layer System

## Overview

We have **three distinct layers** with different purposes:

1. **GDSL (OpenCypher)** - Graph/ML algorithms, Rust, RootAgent orchestration
2. **SDSL (Malloy-inspired)** - Data modeling, TypeScript, MVC, Polars/Arrow
3. **Task Agents** - Workflow orchestration

## Layer 1: GDSL (OpenCypher) - Graph/ML Foundation

### Purpose
- **Graph algorithms** and ML operations
- **RootAgent orchestration** via GDS Kernel
- **Special GDS Init/Task Daemon** that operates various pieces

### Technology
- **Language**: Rust
- **Query Language**: OpenCypher (as GDSL itself)
- **Domain**: Graph data science, ML algorithms, kernel operations

### Architecture

```
RootAgent (SystemD-like Manager)
    ↓
Operates via OpenCypher (GDSL)
    ↓
GDS Kernel (Pure Form Processor)
    ├─→ Graph/ML Algorithms
    ├─→ Indexing
    ├─→ Caching
    └─→ ML Pipeline Execution
    ↓
UserLand (KB Apps - Given Form Processor)
```

**RootAgent operates GDS Kernel** - Like SystemD operates Linux Kernel. RootAgent manages:
- Indexing
- Caching
- ML Pipeline Execution
- Resource Management
- Service Lifecycle

**GDSL is OpenCypher** - The language RootAgent uses to operate the GDS kernel.

### Key Characteristics
- ✅ **Rust-first** - Performance-critical graph/ML operations
- ✅ **OpenCypher as GDSL** - The language itself is OpenCypher
- ✅ **Kernel operations** - Low-level graph/ML algorithms
- ✅ **RootAgent orchestration** - Coordinates agents via GDS kernel

### Example

```cypher
// GDSL (OpenCypher) - Graph algorithm
CALL gds.pageRank.stream({
  nodeProjection: 'Person',
  relationshipProjection: 'KNOWS'
})
YIELD nodeId, score
RETURN gds.util.asNode(nodeId).name AS name, score
ORDER BY score DESC
```

This is **GDSL** - OpenCypher queries that operate on the GDS kernel.

## Layer 2: SDSL (Malloy-inspired) - Data Modeling Platform

### Purpose
- **Data modeling** with semantic layer (measures, dimensions, joins)
- **MVC applications** (FormApp)
- **Data analysis** and aggregation

### Technology
- **Language**: TypeScript
- **Inspiration**: Malloy (semantic modeling)
- **Execution**: Polars/Apache Arrow
- **Domain**: Business data, forms, analytics

### Architecture

```
Malloy-inspired Model (SDSL)
    ↓
DataView (Query)
    ↓
PolarsExecutionEngine
    ↓
Apache Arrow
    ↓
MVC Controller/View
```

**SDSL is TypeScript-first** - data modeling platform for MVC apps.

### Key Characteristics
- ✅ **TypeScript-first** - Business logic, forms, MVC
- ✅ **Malloy-inspired** - Semantic layer (measures, dimensions)
- ✅ **Polars/Arrow** - Columnar execution engine
- ✅ **MVC platform** - FormApp development

### Example

```typescript
// SDSL (Malloy-inspired) - Data modeling
export const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  measures: {
    totalRevenue: sum('invoices.amount'),
    averageInvoice: avg('invoices.amount'),
  },
  dimensions: {
    region: 'region',
    signupMonth: dimension('createdAt', 'month'),
  },
  joins: {
    invoices: {
      model: InvoiceModel,
      on: 'customers.id = invoices.customerId',
      type: 'left',
    },
  },
});

// Query
const view = CustomerModel.view({
  filter: { id: customerId },
  aggregate: ['totalRevenue', 'averageInvoice'],
});
```

This is **SDSL** - TypeScript data modeling for MVC apps.

## Layer 3: Task Agents - Workflow Orchestration

### Purpose
- **Workflow orchestration** (complex workflows)
- **Multi-agent coordination**
- **Long-running processes**

### Technology
- **Language**: TypeScript (NestJS)
- **Domain**: Workflow management, agent coordination

### Architecture

```
Workflow Definition
    ↓
Task Agent
    ↓
Agent Coordination
    ↓
Workflow Execution
```

**Task Agents handle workflows** - complex orchestration, not simple invocation.

### Key Characteristics
- ✅ **Workflow orchestration** - Complex, dialectical workflows
- ✅ **Agent coordination** - Multi-agent systems
- ✅ **Process management** - Long-running processes

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐      ┌──────────────┐                │
│  │   SDSL       │      │ Task Agents  │                │
│  │ (Malloy)     │      │ (Workflows)  │                │
│  │ TypeScript   │      │ TypeScript   │                │
│  │ MVC/Forms    │      │ NestJS       │                │
│  │ Polars/Arrow │      │              │                │
│  └──────┬───────┘      └──────┬───────┘                │
│         │                     │                         │
│         └──────────┬──────────┘                         │
│                    │                                     │
│         ┌──────────▼──────────┐                         │
│         │   GDSL (OpenCypher) │                         │
│         │   Rust              │                         │
│         │   Graph/ML          │                         │
│         │   GDS Kernel        │                         │
│         │   RootAgent         │                         │
│         └─────────────────────┘                         │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Separation of Concerns

| Layer | Language | Domain | Purpose |
|-------|----------|--------|---------|
| **GDSL** | Rust (OpenCypher) | Graph/ML | Kernel operations, RootAgent |
| **SDSL** | TypeScript (Malloy) | Data Modeling | MVC apps, business data |
| **Task Agents** | TypeScript (NestJS) | Workflows | Orchestration, coordination |

## Integration Points

### 1. SDSL → GDSL

SDSL (data modeling) can query GDSL (graph) when needed:

```typescript
// SDSL query that needs graph data
const graphData = await gdsl.query(`
  MATCH (c:Customer)-[r:ORDERED]->(o:Order)
  WHERE c.id = $customerId
  RETURN c, collect(o) as orders
`);

// Use in SDSL model
const customerModel = CustomerModel.fromGraphData(graphData);
```

### 2. Task Agents → GDSL

Task Agents orchestrate GDSL operations:

```typescript
// Task Agent workflow
const workflow = {
  steps: [
    { type: 'gdsl', query: 'CALL gds.pageRank...' },
    { type: 'sdsl', model: CustomerModel, view: {...} },
    { type: 'agent', agentId: 'analysis-agent' }
  ]
};
```

### 3. Task Agents → SDSL

Task Agents coordinate SDSL operations:

```typescript
// Task Agent orchestrates SDSL queries
const workflow = {
  steps: [
    { operation: 'loadCustomer', model: CustomerModel },
    { operation: 'calculateMetrics', measures: ['totalRevenue'] },
    { operation: 'renderDashboard', view: CustomerView }
  ]
};
```

## Key Insights

1. **GDSL = OpenCypher** - The graph query language IS GDSL
2. **SDSL = Malloy-inspired** - TypeScript data modeling platform
3. **Rust = Graph/ML** - Performance-critical operations
4. **TypeScript = Business Logic** - MVC, workflows, agents
5. **Clear separation** - Each layer has distinct purpose

## Next Steps

- [ ] Document GDSL (OpenCypher) as kernel language
- [ ] Clarify GDS Init/Task Daemon architecture
- [ ] Document SDSL (Malloy) as data modeling platform
- [ ] Define integration points between layers
- [ ] Map RootAgent orchestration via GDSL

