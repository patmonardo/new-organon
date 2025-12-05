# What to Digest: The Form Application Server Architecture

## TL;DR

**We're building a Form Application Server Standard Edition** with:
- **SDSL (Semantic Data Science Language)** - Our own semantic modeling layer
- **Graph** (OpenCypher IR)
- **Analytics** (DataModel/DataView with measures, dimensions)
- **ML** (Our extensions)
- **Ontology** (OWL/SHACL/SPIN/SPARQL)

Into a single coherent platform with modern tooling (TypeScript/Rust, not Java).

## The Core Insight

**Three IRs, One Language:**

```
OpenCypher IR (Graph/Ontology)
    + SDSL IR (Analytics - our own)
    + ML IR (Machine Learning)
    = Unified Semantic Data Language
```

**Key realization:**
- Property graphs ARE ontologies
- SDSL IS our semantic layer (not Malloy)
- OpenCypher IR CAN model OWL/SHACL/SPIN
- Together they form a complete language

## The Documents (Reading Order)

### 1. Start Here
**`language-design-vision.md`** - The complete vision
- Language design principles
- IR-first approach
- What makes it interesting
- What we can actually do

### 2. Execution Stack
**`execution-stack.md`** - Our execution layer
- SDSL (our semantic layer)
- Polars (DataFrame operations)
- DuckDB (SQL analytics)
- Postgres (persistent storage)
- Rust GDS (via Absolute Logic)

### 3. Foundation
**`absolute-logic-interface.md`** - TypeScript to Rust boundary
- How TypeScript interfaces with Rust GDS
- Protocol management
- Type-safe boundary

### 4. Ontology
**`opencypher-ir-ontology.md`** - Using OpenCypher IR for ontology
- What OpenCypher really is
- How its IR models ontologies
- Natural fit for OWL/SHACL/SPIN

### 5. Unification
**`unified-ir-design.md`** - The unified IR architecture
- How three IRs work together
- Mapping strategy
- Benefits

## The Key Ideas

### 1. IR-First Design
- IR defines the language
- Syntax is UI for IR
- Multiple syntaxes possible
- Enables tooling

### 2. Property Graphs = Ontologies
- `NodePattern` → OWL classes
- `RelationshipPattern` → OWL properties
- `PathPattern` → SHACL shapes
- Natural mapping

### 3. SDSL Is Our Own
- Our semantic modeling layer
- DataModel/DataView API
- Measures, dimensions, joins
- Compiles to Polars

### 4. Modern Tooling
- TypeScript/Rust (not Java)
- VSCode/Composer (not Eclipse)
- TS-JSON schemas (not XML/RDF)
- Practical (not academic)

### 5. Multiple Backends
- Rust (GDS Kernel)
- TypeScript (Polars/Arrow)
- SQL (DuckDB)
- Cypher (Neo4j)

## Example: Complete Language

```typescript
// Unified language: Graph + Analytics + ML + Ontology
import { defineModel, sum, count } from '@model/data';

const customerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  fields: {
    id: z.string(),
    name: z.string(),
    region: z.string(),
    revenue: z.number(),
  },
  
  // Analytics (SDSL)
  measures: {
    total_revenue: sum('revenue'),
    customer_count: count(),
  },
  dimensions: {
    region: 'region',
  },
  
  // Graph structure (OpenCypher IR)
  relationships: {
    HAS_INVOICE: {
      direction: 'outgoing',
      targetClass: 'Invoice',
    },
  },
  
  // Ontology (OWL/SHACL)
  subClassOf: 'LegalEntity',
  properties: {
    name: {
      type: 'string',
      required: true,
      pattern: '^[A-Za-z ]+$',
    },
  },
  
  // ML (Our extensions)
  features: {
    revenue_normalized: { transform: 'normalize', field: 'total_revenue' },
  },
});

// Unified query
const view = customerModel.view({
  group_by: ['region'],
  aggregate: ['total_revenue', 'customer_count'],
  filter: { region: 'North' },
});
```

## What Makes This Possible

### SDSL Provides
- Our own semantic modeling
- DataModel/DataView API
- Measures, dimensions, joins
- Compiles to Polars

### We Add
- ML extensions (features, models)
- Ontology extensions (OWL/SHACL/SPIN)
- OpenCypher integration (graph IR)
- Unified semantics

### Result
- Complete semantic data language
- Modern tooling
- Real execution
- Practical applications

## The Opportunity

**Not new thinking:**
- Graph languages exist (OpenCypher)
- Analytics languages exist (SQL, Polars)
- ML languages exist (Python)
- Ontology languages exist (OWL/SHACL)

**But new opportunity:**
- **Unification** - All in one coherent language
- **Modern tooling** - TypeScript/Rust, not Java
- **IR-based** - TS-JSON, not XML/RDF
- **Practical** - Real execution, not academic
- **Our own stack** - Meets our needs, not competing with enterprise

## Language Design Perspective

### From Chip Languages to Semantic Languages

**Chip languages:**
- Low-level (registers, gates)
- Performance-focused
- Declarative (HDL)
- IR-based (synthesis)

**Semantic languages:**
- High-level (entities, relationships)
- Expressiveness-focused
- Declarative (data)
- IR-based (compilation)

**Same principles:**
- IR-first design
- Declarative specifications
- Composable primitives
- Type safety

**But more interesting:**
- Semantic meaning
- Real applications
- Modern tooling
- Practical impact

## What Needs Digesting

### 1. Three IRs
- **OpenCypher** - Graph structure, patterns
- **SDSL** - Analytics, measures, dimensions (our own)
- **ML** - Features, embeddings, models

### 2. Execution Stack
- **nodejs-polars** - DataFrame operations (no custom C++ NAPI needed)
- **DuckDB** - Analytics engine (for query planning)
- **Postgres** - Persistent storage
- **Rust GDS** - Graph/ML (via Absolute Logic interface)

### 3. Unified Language
- One syntax
- Multiple IRs
- Coherent semantics
- All compose naturally

### 4. Ontology Mapping
- Property graphs = Ontologies
- NodePattern = OWL classes
- RelationshipPattern = OWL properties
- PathPattern = SHACL shapes

### 5. Modern Approach
- TypeScript/Rust (not Java)
- TS-JSON schemas (not XML)
- VSCode tooling (not Eclipse)
- Practical focus (not academic)

### 6. Real Execution
- Multiple backends (Rust, TS, SQL)
- Real databases (DuckDB, Postgres)
- Production ready
- Not just research

## Questions to Consider

### 1. Keep OpenCypher?
**Yes:**
- Its IR is perfect for ontology
- Property graphs = ontologies
- Rich pattern language
- Already in our codebase

### 2. How to unify IRs?
**Design strategy:**
- Map OpenCypher → Ontology
- Keep SDSL → Analytics
- Add ML → Extensions
- Unified semantics layer

### 3. What about tooling?
**Build:**
- Language server (LSP)
- Visual composer
- Profiler
- Multiple backends

### 4. Real applications?
**Build:**
- Customer examples
- ML pipelines
- Knowledge graphs
- Business apps

## Next Steps (When Ready)

1. **Digest** - Take time to understand
2. **Design unified IR** - Map the three IRs
3. **Extend syntax** - Add ML, ontology, patterns
4. **Build tooling** - Language server, composer
5. **Real applications** - Production use

## The Vision

**A complete semantic data language:**
- Graph modeling (OpenCypher)
- Analytics (SDSL - our own)
- Machine learning (Our extensions)
- Ontology (OWL/SHACL/SPIN/SPARQL)
- Modern tooling (TypeScript/Rust)
- Real execution (Polars, DuckDB, Postgres, Rust GDS)
- Practical applications (Production ready)

**Our stack meets our needs.**

---

**Take your time** - this is a significant design.
