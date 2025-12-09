# BI/Malloy Integration Analysis: Model-View-Dashboard Platform

## Executive Summary

This document analyzes the current architecture and identifies specific areas where Malloy-inspired semantic modeling needs to be enhanced for BI as a "Secondary Target for Logic" - meaning BI uses secondary empirical concepts within the larger AI Platform context.

## Architecture Context

### The Three-Layer System

```
┌─────────────────────────────────────────────────────────┐
│              AI Platform (Primary Target)               │
│                                                         │
│  GDSL (OpenCypher) - Rust                              │
│  ├─ Graph/ML Algorithms                                │
│  ├─ RootAgent Orchestration                            │
│  └─ Logical Forms Processing                           │
│                                                         │
│  Logic (@logic) - "Perfect Server"                      │
│  ├─ Reflection, EssentialRelations                     │
│  ├─ Execution Engine (Polars/Arrow/DuckDB)             │
│  └─ Nondual Semantics                                  │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│         BI Subset (Secondary Target - Empirical)        │
│                                                         │
│  SDSL (Malloy-inspired) - TypeScript                    │
│  ├─ MVC as SDSL                                        │
│  ├─ Model-View-Dashboard Platform                      │
│  └─ Secondary Empirical Concepts                       │
│                                                         │
│  Model (@model) - First Activation of Logic            │
│  ├─ FormModel (State:Structure)                        │
│  ├─ FormView (Representation:Perspective)             │
│  └─ FormController (Action:Rule)                       │
└─────────────────────────────────────────────────────────┘
```

### Key Insight: BI as Secondary Target

- **Primary Target (GDSL)**: AI Platform operations - graph algorithms, ML, logical forms
- **Secondary Target (SDSL)**: BI operations - empirical data modeling, dashboards, views
- **Relationship**: BI uses "secondary empirical concepts" - it's empirical data modeling within the larger AI platform context

## Current State Analysis

### 1. SDSL Implementation (`model/src/data/sdsl.ts`)

**Current Capabilities:**
- ✅ Basic `DataModel` with measures, dimensions, joins
- ✅ `DataView` for query definitions
- ✅ Helper functions: `sum()`, `count()`, `avg()`, `dimension()`
- ✅ Malloy-inspired structure (Source, Dimension, Measure, View, Join)

**Gaps for Full Malloy BI Integration:**
- ❌ No explicit `primary_key` / `foreign_key` definitions
- ❌ Limited view composition (no nested views, no view inheritance)
- ❌ No explicit `explore` concept (Malloy's primary query mechanism)
- ❌ Missing Malloy-specific features:
  - `turtle` (nested aggregations)
  - `reduce` (simplified aggregations)
  - `where` filters at model level
  - `sql` blocks for custom SQL
  - `query` blocks for reusable queries
  - `source` blocks for data source definitions

### 2. View Schema (`model/src/schema/view.ts`)

**Current State:**
- Philosophical foundation (Representation:Perspective dyad)
- Kriya integration (experiential manifestation)
- But **not** aligned with Malloy View specifications

**Needed for BI:**
- Malloy View spec alignment:
  - View as query definition (group_by + aggregate)
  - View composition and nesting
  - View parameters
  - View filters
  - View joins

### 3. Dashboard Schema (`model/src/schema/dashboard.ts`)

**Current State:**
- Basic dashboard components (StatCard, Container, ConceptCloud, etc.)
- Grid layout system
- Extends FormShape

**Gaps for Malloy BI:**
- ❌ No explicit connection to Malloy Views
- ❌ No dashboard-level query definitions
- ❌ Missing Malloy dashboard patterns:
  - Dashboard as collection of Views
  - Dashboard filters/parameters
  - Dashboard-level aggregations
  - Cross-view relationships

### 4. Application Schema (`model/src/schema/application.ts`)

**Current State:**
- Has `ViewSchema` with `type: 'malloy'` option
- References to `DataModelRefSchema`
- Dashboard integration

**Gaps:**
- ❌ View schema doesn't fully capture Malloy View structure
- ❌ No explicit Malloy model-to-view-to-dashboard pipeline
- ❌ Missing Malloy query compilation/execution path

## Required Enhancements

### Priority 1: Enhanced SDSL for Malloy BI

#### 1.1 Complete Malloy Model Structure

```typescript
// Enhanced DataModel with Malloy alignment
export interface MalloyModelConfig {
  name: string;
  source: {
    type: 'table' | 'sql' | 'connection';
    value: string;
  };
  primary_key?: string;
  measures: Record<string, MeasureDefinition>;
  dimensions: Record<string, DimensionDefinition>;
  joins?: Record<string, JoinDefinition>;
  where?: FilterDefinition; // Model-level filters
  sql?: string; // Custom SQL blocks
}
```

#### 1.2 Malloy View Structure

```typescript
// Malloy View as query definition
export interface MalloyView {
  name: string;
  model: string; // Reference to DataModel
  group_by?: string[]; // Dimensions
  aggregate?: string[]; // Measures
  where?: FilterDefinition;
  limit?: number;
  order_by?: OrderDefinition[];
  // Malloy-specific
  turtle?: TurtleDefinition[]; // Nested aggregations
  reduce?: string[]; // Simplified aggregation
  nest?: NestDefinition[]; // Nested structures
}
```

#### 1.3 Dashboard-View Integration

```typescript
// Dashboard as collection of Malloy Views
export interface MalloyDashboard {
  id: string;
  name: string;
  views: MalloyViewReference[]; // References to Views
  filters?: DashboardFilter[]; // Dashboard-level filters
  parameters?: DashboardParameter[]; // Dashboard parameters
  layout: DashboardLayout;
}
```

### Priority 2: View-Dashboard Pipeline

#### 2.1 View Execution Path

```
DataModel (Malloy Source)
    ↓
MalloyView (Query Definition)
    ↓
ViewQuery (Compiled Query)
    ↓
PolarsExecutionEngine
    ↓
SemanticResult
    ↓
SemanticHydrator
    ↓
FormModel / Dashboard Component
```

#### 2.2 Dashboard as View Collection

- Dashboard = Collection of Views
- Each Dashboard component = Rendered View
- Dashboard-level filters apply to all Views
- Dashboard parameters pass to View queries

### Priority 3: Rust Platform Integration (Post-Prisma)

**Current Understanding:**
- Moving away from Prisma
- Rust platform (GDS/GDSL) handles data operations
- Polars/Arrow/DuckDB execution engine

**Integration Points:**
- SDSL models compile to execution plans
- Execution plans run on Rust platform
- Results return as Arrow/JSON
- No Prisma dependency

## Specific Implementation Areas

### Area 1: Enhanced `sdsl.ts`

**File:** `model/src/data/sdsl.ts`

**Enhancements Needed:**
1. Add `primary_key` / `foreign_key` to `DataModelConfig`
2. Add `where` filters at model level
3. Add `sql` blocks for custom SQL
4. Enhance `ViewQuery` to support:
   - `turtle` (nested aggregations)
   - `reduce` (simplified aggregations)
   - `nest` (nested structures)
   - View parameters
5. Add `explore` concept (Malloy's primary query)

### Area 2: Malloy View Schema

**File:** `model/src/schema/view.ts` (or new `malloy-view.ts`)

**New Schema:**
- `MalloyViewSchema` aligned with Malloy View structure
- View composition (nested views)
- View parameters
- View filters
- View joins

### Area 3: Dashboard-View Bridge

**File:** `model/src/schema/dashboard.ts`

**Enhancements:**
- `MalloyViewReference` type
- Dashboard as collection of Views
- Dashboard-level filters/parameters
- View-to-component mapping

### Area 4: Application Schema Integration

**File:** `model/src/schema/application.ts`

**Enhancements:**
- Explicit Malloy model → view → dashboard pipeline
- View execution configuration
- Dashboard view binding

## Architecture Principles

### 1. MVC as SDSL

- **Model**: DataModel (Malloy Source) = State:Structure
- **View**: MalloyView (Query Definition) = Representation:Perspective
- **Controller**: View Execution + Hydration = Action:Rule

### 2. Model-View-Dashboard Platform

- **Model**: Malloy DataModel (semantic data model)
- **View**: MalloyView (query definition)
- **Dashboard**: Collection of Views with layout

### 3. Secondary Empirical Concepts

- BI operates on empirical data (not pure logical forms)
- Uses secondary concepts (measures, dimensions) vs primary (graph, ML)
- Empirical = observable, measurable data
- Secondary = derived from primary AI platform operations

## Next Steps (1 Hour Focus)

### Immediate Actions

1. **Review Current SDSL** (`model/src/data/sdsl.ts`)
   - Identify exact gaps vs Malloy spec
   - Document missing features

2. **Design Malloy View Schema**
   - Create `MalloyViewSchema` aligned with Malloy
   - Integrate with existing `ViewSchema`

3. **Design Dashboard-View Integration**
   - How Dashboard references Views
   - How Dashboard filters apply to Views
   - How Dashboard parameters pass to Views

4. **Document Rust Platform Integration**
   - How SDSL compiles to execution plans
   - How execution plans run on Rust platform
   - How results flow back to BI layer

### Questions to Answer

1. **Malloy Compatibility Level**
   - Full Malloy compatibility or Malloy-inspired?
   - Which Malloy features are essential vs nice-to-have?

2. **View Execution**
   - Where does View compilation happen? (TypeScript or Rust?)
   - How do Views execute? (Polars, DuckDB, or both?)

3. **Dashboard Architecture**
   - Is Dashboard a first-class concept or just View collection?
   - How do Dashboard components map to Views?

4. **Rust Platform Integration**
   - What's the exact interface between SDSL and Rust platform?
   - How do we avoid Prisma while maintaining data access?

## References

- [ADR 001: Malloy-Compatible Semantic Data Integration](./adr/001-malloy-integration.md)
- [GDSL Architecture](../../gdsl/doc/gdsl-architecture.md)
- [Semantic Hydrator Design](./semantic-hydrator.md)
- [Model README](../README.md)

