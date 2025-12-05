# Model Package Documentation

## Overview

The `@model` package implements a **Form Application Server Standard Edition** with:
- **SDSL (Semantic Data Science Language)** - Our own semantic modeling layer
- **Graph modeling** (OpenCypher IR)
- **Analytics** (DataModel/DataView with measures, dimensions)
- **Machine learning** (ML extensions)
- **Ontology** (OWL/SHACL/SPIN/SPARQL)

## Start Here

### If You're New
**Read:** `README-DIGEST.md` - Quick overview of the architecture

### If You Want the Complete Vision
**Read:** `language-design-vision.md` - Complete language design philosophy

### If You're Implementing
**Read:** `unified-ir-design.md` - IR architecture and implementation strategy

## Core Concepts

### 1. Three IRs, One Language
- **OpenCypher IR** - Graph structure, patterns, ontology
- **SDSL IR** - Analytics, measures, dimensions (our own semantic layer)
- **ML IR** - Features, embeddings, models

### 2. IR-First Design
- IR defines the language semantics
- Syntax is UI for the IR
- Multiple backends compile from IR

### 3. Property Graphs = Ontologies
- `NodePattern` → OWL classes
- `RelationshipPattern` → OWL properties
- `PathPattern` → SHACL shapes

## Documentation Structure

### Language Design
- `language-design-vision.md` - Complete vision and philosophy
- `README-DIGEST.md` - Quick overview for digestion
- `unified-ir-design.md` - Unified IR architecture

### Extensions
- `opencypher-ir-ontology.md` - OpenCypher IR for ontology modeling

### Architecture
- `form-app-server-standard.md` - Form Application Server architecture
- `unified-ir-design.md` - Unified IR architecture
- `execution-stack.md` - SDSL + Polars + DuckDB + Rust GDS
- `absolute-logic-interface.md` - TypeScript to Rust boundary
- `mvc-architecture.md` - MVC as general-purpose platform
- `client-hierarchy.md` - Client hierarchy (GDS → GDSL → Logic → MVC)

### Schema System
- `entity-property-relation-mvc.md` - Entity:Property:Relation in MVC
- `sankara-integration.md` - Integration from sankara archive

### Other
- `strategic-direction.md` - Strategic direction: beyond "Semantic Data Analysis"
- `personal-vs-big-data-science.md` - GDSL vs SDSL distinction
- `full-office-capabilities.md` - Full Office capabilities vision
- `mvc-as-logic-sdk.md` - MVC as part of Logic SDK
- `mvc-userland-protocol.md` - MVC as UserLand Application Layer Protocol
- `graph-db-app-server.md` - Graph DB + App Server architecture
- `semantic-hydrator.md` - SemanticHydrator bridge

## Key Files

### Schemas (`src/schema/`)
- `malloy-ir.ts` - IR schemas (legacy, may be removed)
- `malloy-ontology-ir.ts` - Ontology IR (OWL/SHACL/SPIN/SPARQL)
- `malloy-expression-language.ts` - Expression language schemas
- `entity-property-relation.ts` - Entity:Property:Relation DSL
- `shape-context-morph.ts` - Shape:Context:Morph Form API
- `form-display.ts` - Forms as DisplayShapes
- `application.ts` - Application structure (Desktop Neo4j concept)

### Data Layer (`src/data/`)
- `sdsl.ts` - Semantic Data Specification Language (our own)
- `polars-engine.ts` - Polars/Apache Arrow execution engine
- `semantic-hydrator.ts` - Bridge between Data SDSL and MVC

### SDSL Components (`src/sdsl/`)
- `react-controller.ts` - React/Next.js controller with Server Actions
- `radix-adapter.tsx` - Radix UI integration
- `radix-primitives.tsx` - Radix primitive components

## Examples

### Customer Example (`examples/customer/`)
The primary example demonstrating the Semantic MVC approach:
- `model.ts` - CustomerModel with SDSL semantic definitions
- `view.tsx` - React view components (Radix UI)
- `controller.ts` - FormController for backend operations
- `schema.ts` - Zod schema for Customer entity
- `data-service.ts` - Data access layer
- `runtime.ts` - Runtime execution

### MVC Examples (`examples/mvc/`)
Additional MVC examples showing different patterns:
- Traditional MVC (archived)
- Agent MVC patterns

## Architecture Layers

### Layer 1: GDS Kernel (Rust)
- Graph algorithms
- GNN operations
- ML pipelines
- OpenCypher execution

### Layer 2: Logic Processor (TypeScript)
- Form processing
- Transformations
- Relationships
- Entity:Property:Relation

### Layer 3: MVC SDSL (TypeScript)
- Data modeling (our own SDSL)
- Form applications
- Analytics (measures, dimensions)
- ML (features, embeddings)
- Server Actions (React/Next.js)

### Layer 4: Task Agents (TypeScript/NestJS)
- Workflow orchestration
- Multi-agent coordination
- Complex task management

## Execution Stack

### Our Stack
- **SDSL** - Our own semantic data modeling layer
- **nodejs-polars** - DataFrame operations (no custom C++ NAPI needed)
- **DuckDB** - SQL execution and query planning
- **Postgres** - Persistent storage
- **Rust GDS** - Graph/ML (via Absolute Logic interface)

### Strategic Direction
**Our stack meets our needs** - We're not competing with enterprise tech stacks or building "Semantic Data Analysis" workstations.

**Key insights:**
- ✅ No custom C++ extensions needed (nodejs-polars provides it)
- ✅ Our own semantic layer (SDSL, not Malloy)
- ✅ TypeScript-first architecture
- ✅ Battle-tested components
- ✅ Focus on Semantic Web level (OWL/SHACL/SPIN/SPARQL), not data analysis level
- ✅ AI MultiAgent systems will handle semantic drilldown (not analyst workstations)

**See:** `strategic-direction.md` for our position on Malloy and data analysis tools.

## Next Steps

### To Understand
1. Read `README-DIGEST.md` for overview
2. Read `language-design-vision.md` for complete vision
3. Explore `unified-ir-design.md` for IR architecture
4. Review `execution-stack.md` for execution layer

### To Implement
1. Study SDSL in `src/data/sdsl.ts`
2. Study PolarsExecutionEngine in `src/data/polars-engine.ts`
3. Review schema files in `src/schema/`

### To Use
1. Check `examples/customer/` for Semantic MVC example
2. Review schema files in `src/schema/`
3. Explore tests in `test/`

## Resources

### External
- [OpenCypher](https://opencypher.org/) - Property graph query language
- [Apache Arrow](https://arrow.apache.org/) - Columnar data format
- [Polars](https://pola.rs/) - DataFrame library
- [DuckDB](https://duckdb.org/) - SQL analytics engine

### Internal
- `@logic` - Logic Form Processor
- `@gds` - Graph Data Science Kernel
- `@task` - Task Agent orchestration

---

**This is our stack** - SDSL + Polars + DuckDB + Rust GDS via Absolute Logic.
