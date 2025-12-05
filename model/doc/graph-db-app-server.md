# Graph DB + App Server Architecture

## The Two Main Things

Like **Apache + MySQL**, we have two complementary systems:

1. **Graph DB (GDS)** - The data/kernel layer
2. **App Server (MVC)** - The application layer

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Graph DB (GDS)                        │
│              Kernel, ML, Logical Forms                    │
│              Rust, OpenCypher (GDSL)                      │
└───────────────────────┬─────────────────────────────────┘
                        │
                        │ GDSL (Client Language)
                        │
┌───────────────────────▼─────────────────────────────────┐
│              App Server (MVC)                            │
│         FormApp SDK, Data Models, Forms                  │
│         TypeScript, Malloy-inspired (SDSL)              │
│                                                           │
│  ┌──────────────────────────────────────────────┐     │
│  │  Everything is a Form                          │     │
│  │  FormShape:Context:Morph Machinery           │     │
│  │  Entity:Property:Relation                    │     │
│  └──────────────────────────────────────────────┘     │
│                                                           │
│  ┌──────────────────────────────────────────────┐     │
│  │  Desktop Neo4j                                │     │
│  │  Application Schema                           │     │
│  │  Dashboard with Nav Links/Bars               │     │
│  │  Malloy + D3/Recharts                        │     │
│  └──────────────────────────────────────────────┘     │
└─────────────────────────────────────────────────────────┘
```

## Graph DB (GDS)

**The Kernel:**
- **ML operations** - Machine learning, model training
- **Logical Forms** - Advanced logical processing
- **Rust-based** - Performance-critical
- **OpenCypher (GDSL)** - Graph query language
- **Kernel operations** - Low-level algorithms
- **Enterprise-scale** - Big Data processing

**GDSL is the language** used to communicate with GDS:
- OpenCypher queries
- Graph operations
- ML operations
- Kernel access

## App Server (MVC)

**The Application Layer:**
- **FormApp SDK** - Development toolkit
- **Data Models** - Malloy-inspired semantic models
- **Forms** - Everything is a Form
- **TypeScript** - Business logic
- **Polars/Arrow** - Columnar execution
- **Personal Data Science** - Individual/team scale

**MVC Components:**
- **Form-model, view, controller** - Runtime Client
- **MVC SDSL** - FormApp SDK
- **Shape:Context:Morph** - Form processing machinery
- **Entity:Property:Relation** - Form Processor API

## Everything is a Form

**The Core Maxim:**
- **FormShape** is fundamental
- All form processing is based on FormShape
- Form Processor moved to @logic, but MVC still uses Shape:Context Morph Machinery
- Entity:Property:Relation is part of Form Processor API

**Form Processing:**
```
FormShape (Pure Form)
    ↓
Context (Transactional Environment)
    ↓
Morph (Organic Unity of Shape + Context)
    ↓
Entity:Property:Relation (Concrete Existence)
```

## Entity:Property:Relation

**Part of Form Processor API:**
- **Entity** - Instances of Shapes
- **Property** - Contextualized predicates/measures
- **Relation** - Essential/Absolute ties

**Surfaced in MVC DSL:**
- Entity:Property:Relation is part of FormApp abstraction
- Available through MVC DSL
- Integrated with FormShape:Context:Morph

## Desktop Neo4j

**The Application Schema Concept:**
- **Application** - Complete application definition
- **Dashboard** - Main interface
- **Nav Links/Bars** - Navigation components
- **Charts** - D3/Recharts visualizations
- **Malloy Views** - Semantic data views

**Application Structure:**
```typescript
Application {
  id: string;
  name: string;
  dashboard: Dashboard;
  navigation: Navigation;
  views: View[];
  models: DataModel[];
  forms: FormShape[];
}
```

## Malloy + D3/Recharts

**Transformations and Relationships:**
- **Malloy** focuses on Transformations and Relationships
- **Logic Processor** exports itself as Transformation and Relationships Evaluator
- **Perfect alignment** - Malloy's focus matches Logic Processor's capabilities

**View Technology:**
- **Malloy has View technology** - TS modules ready to execute MVC IR
- **D3/Recharts** - Visualization components
- **Observable** - Data visualization platform
- **Dashboard** - Combined Malloy views + D3/Recharts

## Integration Points

### 1. Form Processor → MVC

**Form Processor (in @logic):**
- Uses Shape:Context:Morph
- Processes Entity:Property:Relation
- Exports Transformations and Relationships

**MVC (in @model):**
- Uses FormShape:Context:Morph Machinery
- Surfaces Entity:Property:Relation via MVC DSL
- Integrates with FormApp abstraction

### 2. Malloy → MVC IR

**Malloy IR:**
- Transformations
- Relationships
- Views (TS modules)

**MVC IR:**
- DataModel (Malloy-inspired)
- DataView (Queries)
- DisplayDocument (UI)

**Integration:**
- Malloy Views execute MVC IR
- Malloy Transformations/Relationships align with Logic Processor

### 3. Sankara Form Work → MVC

**Sankara Components:**
- Lists (breadcrumbs, navbars)
- Links (navigation)
- Recharts (charts)
- Observable (visualizations)

**MVC Integration:**
- Lists/Links schemas in `model/src/schema/`
- Recharts/Observable in Dashboard
- Nav Links/Bars in Application schema

## Complete Flow

```
Graph DB (GDS)
    ↓
GDSL (Client Language)
    ↓
Logic Form Processor (Primary GDS Client)
    ├─→ Uses GDSL as Client to GDS
    ├─→ Processes Given Forms
    └─→ Exports Transformations and Relationships
    ↓
MVC (Primary Client of Given Form Processor)
    ├─→ Uses FormShape:Context:Morph
    ├─→ Surfaces Entity:Property:Relation
    ├─→ Malloy Views execute MVC IR
    ├─→ D3/Recharts in Dashboard
    └─→ Lists/Links for Navigation
    ↓
FormApp (Desktop Neo4j)
    ├─→ Application Schema
    ├─→ Dashboard
    ├─→ Nav Links/Bars
    └─→ Malloy + D3/Recharts
```

## Key Insights

1. **Two main things** - Graph DB (GDS) and App Server (MVC)
2. **Everything is a Form** - FormShape is fundamental
3. **Shape:Context:Morph** - Form processing machinery
4. **Entity:Property:Relation** - Form Processor API, surfaced in MVC DSL
5. **Desktop Neo4j** - Application schema concept
6. **Malloy alignment** - Transformations/Relationships match Logic Processor
7. **Malloy Views** - TS modules execute MVC IR
8. **Sankara integration** - Lists/Links, Recharts, Observable

## Next Steps

- [ ] Create application.ts schema (Desktop Neo4j)
- [ ] Integrate Entity:Property:Relation into MVC DSL
- [ ] Merge sankara form work (Recharts, Observable, Lists/Links)
- [ ] Document Malloy View technology integration
- [ ] Build Dashboard with Malloy + D3/Recharts

