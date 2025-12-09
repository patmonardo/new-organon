# BI Architecture Summary: Model-View-Dashboard Platform

## Core Architecture

### The Three-Layer System

```
┌─────────────────────────────────────────────────────────┐
│         AI Platform (Primary Target - GDSL)              │
│  • Graph/ML Algorithms (Rust)                           │
│  • RootAgent Orchestration                               │
│  • Logical Forms Processing                              │
│  • Primary Concepts (Graph, ML, Logical Forms)           │
└─────────────────────────────────────────────────────────┘
                        ↓
┌─────────────────────────────────────────────────────────┐
│      BI Subset (Secondary Target - SDSL/Malloy)         │
│  • MVC as SDSL                                          │
│  • Model-View-Dashboard Platform                        │
│  • Secondary Empirical Concepts (Measures, Dimensions)  │
│  • Malloy-inspired Semantic Modeling                    │
└─────────────────────────────────────────────────────────┘
```

### Key Concepts

1. **GDSL (Primary)**: AI Platform language - OpenCypher, Rust, Graph/ML
2. **SDSL (Secondary)**: BI Platform language - Malloy-inspired, TypeScript, Empirical Data
3. **Logic:Model Dyad**: Logic = perfect server, Model = first activation
4. **MVC as SDSL**: Model-View-Controller implemented as Semantic Data Specification Language
5. **Model-View-Dashboard**: BI platform structure - Models define data, Views define queries, Dashboards compose Views

## Current Implementation

### What Exists

**SDSL Core** (`model/src/data/sdsl.ts`)
- ✅ `DataModel` with measures, dimensions, joins
- ✅ `DataView` for query definitions
- ✅ Basic Malloy-inspired structure

**Execution Layer**
- ✅ `PolarsExecutionEngine` - Executes DataView queries
- ✅ `SemanticHydrator` - Bridges data results to FormModel
- ✅ Arrow/JSON result format

**Schema Layer**
- ✅ `DashboardSchema` - Dashboard components and layout
- ✅ `ViewSchema` - Basic view structure (philosophical foundation)
- ✅ `ApplicationSchema` - Application definition with views/dashboards

### What's Missing for Full BI

**SDSL Enhancements**
- ❌ Primary/Foreign key definitions
- ❌ Model-level filters (`where`)
- ❌ Custom SQL blocks
- ❌ Malloy features: `turtle`, `reduce`, `nest`, `explore`

**View Enhancements**
- ❌ Full Malloy View structure
- ❌ View parameters
- ❌ View composition/nesting
- ❌ View-to-Dashboard mapping

**Dashboard Enhancements**
- ❌ Dashboard as View collection
- ❌ Dashboard-level filters/parameters
- ❌ View component type
- ❌ Dashboard-View execution pipeline

## Architecture Principles

### 1. BI as Secondary Target

- **Primary (GDSL)**: AI operations - graph algorithms, ML, logical forms
- **Secondary (SDSL)**: BI operations - empirical data modeling, dashboards
- **Relationship**: BI uses "secondary empirical concepts" - observable, measurable data derived from primary AI operations

**Strategic Position:**
- We're **not a pure BI platform** - BI is secondary within an AI platform
- **Entirely Agential** - agents are primary interface
- Looker/Malloy are **more challenged by AI agents** than we are
- We don't compete on BI terms - we're an AI platform company

### 2. MVC as SDSL

- **Model**: `DataModel` (Malloy Source) = State:Structure
- **View**: `MalloyView` (Query Definition) = Representation:Perspective
- **Controller**: View Execution + Hydration = Action:Rule

### 3. Model-View-Dashboard Platform

```
DataModel (Semantic Data Model)
    ↓ defines
MalloyView (Query Definition)
    ↓ composes
Dashboard (View Collection + Layout)
    ↓ renders
UI (React/Radix Components)
```

### 4. Rust Platform Integration (Post-Prisma)

- **No Prisma**: Rust platform (GDS/GDSL) handles data operations
- **Execution**: Polars/Arrow/DuckDB via Rust platform
- **Interface**: SDSL compiles to execution plans → Rust platform → Arrow/JSON results

## Implementation Roadmap

### Phase 1: Enhance SDSL Core
- Add primary_key/foreign_key
- Add model-level filters
- Add Malloy features (turtle, reduce, nest, explore)
- **Status**: Design complete, ready for implementation

### Phase 2: Create Malloy View Schema
- Full Malloy View structure
- View parameters
- View composition
- **Status**: Schema designed, ready for implementation

### Phase 3: Dashboard-View Integration
- View component type
- Dashboard as View collection
- Dashboard filters/parameters
- **Status**: Design complete, ready for implementation

### Phase 4: Execution Services
- `MalloyViewService` - Execute Malloy views
- `DashboardViewService` - Execute dashboard views
- **Status**: Design complete, ready for implementation

## Key Files

### Current Implementation
- `model/src/data/sdsl.ts` - SDSL core (needs enhancement)
- `model/src/data/semantic-hydrator.ts` - Data-to-form bridge
- `model/src/data/polars-engine.ts` - Execution engine
- `model/src/schema/dashboard.ts` - Dashboard schema (needs View integration)
- `model/src/schema/view.ts` - View schema (philosophical, needs Malloy alignment)
- `model/src/schema/application.ts` - Application schema

### Documentation
- `model/doc/bi-malloy-integration-analysis.md` - Full analysis
- `model/doc/malloy-bi-enhancement-plan.md` - Implementation plan
- `model/doc/adr/001-malloy-integration.md` - ADR on Malloy integration

## Questions for Discussion

1. **Malloy Compatibility**: Full Malloy spec or Malloy-inspired?
2. **View Execution**: TypeScript or Rust compilation?
3. **Dashboard Architecture**: View collection or separate concept?
4. **Rust Interface**: Exact API between SDSL and Rust platform?

## Next Steps

1. Review enhancement plan
2. Prioritize features (which Malloy features are essential?)
3. Implement Phase 1 (SDSL enhancements)
4. Test integration with existing code
5. Iterate on Dashboard-View integration

---

**Key Insight**: BI is a "Secondary Target for Logic" - it uses secondary empirical concepts (measures, dimensions, aggregations) within the larger AI Platform context. The Model-View-Dashboard platform provides the structure for BI operations while remaining embedded in the AI Platform architecture.

