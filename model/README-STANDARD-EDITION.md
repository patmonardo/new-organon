# @model Platform - Form Application Server "Standard Edition"

## Quick Start

The **@model platform** is a complete **Form Application Server "Standard Edition"** for building form-based applications with a **unified semantic data language** that combines Graph (OpenCypher), Analytics (Malloy), ML, and Ontology (OWL/SHACL/SPIN/SPARQL).

## Core Concepts

### Everything is a Form
- **FormShape** is fundamental
- All components reference FormShape
- DisplayShape-based rendering

### Form API (Complete)
- **Shape:Context:Morph** - Pure form processing
- **Entity:Property:Relation** - Concrete existence

### Malloy Middleware Iron
- Strong foundational layer
- Transformations & Relationships
- Server Actions support

### Rich Morphology
- Complete transformation system
- Malloy Model-View (Dyadic)
- Relationship definitions

## Architecture

```
React/Next (UI) → Server Actions → Malloy Middleware → Data Layer
```

## Key Schemas

```typescript
import {
  // Form API
  createShape, createContext, createMorph,
  createEntity, createProperty, createRelation,
  
  // Malloy
  createMalloyModel, createMalloyView,
  createMalloyMiddlewareConfig,
  
  // Forms
  createFormDisplay,
  createDashboardDisplayController,
  
  // Application
  createApplication,
} from '@model/schema';
```

## Documentation

### Start Here
- [README-DIGEST.md](./doc/README-DIGEST.md) - Quick overview for digestion
- [Language Design Vision](./doc/language-design-vision.md) - Complete vision

### Core Architecture
- [Form Application Server Standard Edition](./doc/form-app-server-standard.md)
- [Unified IR Design](./doc/unified-ir-design.md) - Three IRs, one language
- [Architecture Overview](./doc/graph-db-app-server.md)

### Extensions
- [Malloy ML Extension](./doc/malloy-ml-extension.md) - Features, embeddings, models
- [Malloy Ontological Extension](./doc/malloy-ontological-extension.md) - OWL/SHACL/SPIN/SPARQL
- [OpenCypher IR for Ontology](./doc/opencypher-ir-ontology.md)

### Integration
- [Malloy Middleware Iron](./doc/malloy-middleware-iron.md)
- [Rich Morphology](./doc/morphology-malloy-integration.md)
- [Server Actions Patterns](./doc/server-actions-malloy-pattern.md)

### Full Index
- [Documentation Index](./doc/index.md) - Complete documentation map

## Next Steps

- Fork Malloy repositories for integration
- Enhance Model-View (Dyadic) structure
- Build Malloy View technology
- Optimize performance and caching

