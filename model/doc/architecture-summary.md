# Architecture Summary: Graph DB + App Server

## The Two Main Things

Like **Apache + MySQL**, we have two complementary systems:

1. **Graph DB (GDS)** - The data/kernel layer
2. **App Server (MVC)** - The application layer

## Key Architectural Decisions

### 1. Everything is a Form

**The Core Maxim:**
- **FormShape** is fundamental
- All form processing is based on FormShape
- Form Processor moved to @logic, but MVC still uses Shape:Context:Morph Machinery
- Entity:Property:Relation is part of Form Processor API

**Maintained through:**
- FormShape references in Entity:Property:Relation
- Shape:Context:Morph machinery in MVC
- FormApp abstraction

### 2. Entity:Property:Relation in MVC DSL

**Surfaced from Logic Form Processor:**
- Entity:Property:Relation is part of Form Processor API (in @logic)
- But we expose it through MVC DSL
- FormApps can work with it directly
- Still maintains "Everything is a Form" via FormShape references

### 3. Desktop Neo4j (Application Schema)

**The Application Concept:**
- **Application** - Complete application definition
- **Dashboard** - Main interface with Malloy + D3/Recharts
- **Navigation** - Lists/Links for breadcrumbs and navbars
- **Views** - Malloy views, charts, tables, forms
- **Models** - Malloy-inspired semantic data models
- **Forms** - FormShape definitions

### 4. Malloy Alignment

**Remarkable Alignment:**
- **Malloy** focuses on Transformations and Relationships
- **Logic Processor** exports itself as Transformation and Relationships Evaluator
- **Perfect match** - Malloy's focus matches Logic Processor's capabilities

**Malloy View Technology:**
- Malloy has View technology - TS modules ready to execute MVC IR
- Views execute MVC IR directly
- Integration with Logic Processor transformations

### 5. Sankara Integration

**Components to Merge:**
- **Lists** - Breadcrumbs, navbars (from sankara/app/form/list)
- **Links** - Navigation (from sankara/app/form/list)
- **Recharts** - Charts (from sankara/app/form)
- **Observable** - Data visualization platform

**Integration:**
- Lists/Links schemas in `model/src/schema/`
- Recharts/Observable in Dashboard
- Nav Links/Bars in Application schema

## Architecture Layers

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

## Complete Flow

```
Graph DB (GDS)
    ↓
GDSL (Client Language)
    ↓
Logic Form Processor (Primary GDS Client)
    ├─→ Uses GDSL as Client to GDS
    ├─→ Processes Given Forms
    ├─→ Exports Transformations and Relationships
    └─→ Entity:Property:Relation (Form Processor API)
    ↓
MVC (Primary Client of Given Form Processor)
    ├─→ Uses FormShape:Context:Morph
    ├─→ Surfaces Entity:Property:Relation (MVC DSL)
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

## Key Files Created

### Schemas
- `model/src/schema/application.ts` - Desktop Neo4j Application schema
- `model/src/schema/entity-property-relation.ts` - Entity:Property:Relation in MVC DSL

### Documentation
- `model/doc/graph-db-app-server.md` - Graph DB + App Server architecture
- `model/doc/malloy-logic-alignment.md` - Malloy Transformations/Relationships alignment
- `model/doc/entity-property-relation-mvc.md` - Entity:Property:Relation in MVC DSL
- `model/doc/architecture-summary.md` - This summary

## Next Steps

- [ ] Merge sankara form work (Recharts, Observable, Lists/Links)
- [ ] Design Malloy View technology integration with MVC IR
- [ ] Build Dashboard with Malloy + D3/Recharts
- [ ] Create FormApp examples using Entity:Property:Relation
- [ ] Integrate Lists/Links from sankara into Application schema

## Key Insights

1. **Two main things** - Graph DB (GDS) and App Server (MVC)
2. **Everything is a Form** - FormShape is fundamental
3. **Shape:Context:Morph** - Form processing machinery
4. **Entity:Property:Relation** - Form Processor API, surfaced in MVC DSL
5. **Desktop Neo4j** - Application schema concept
6. **Malloy alignment** - Transformations/Relationships match Logic Processor
7. **Malloy Views** - TS modules execute MVC IR
8. **Sankara integration** - Lists/Links, Recharts, Observable

