# MVC as UserLand Application Layer Protocol

## Overview

**MVC for us is:**
1. **UserLand Application Layer Protocol** - The protocol for building applications
2. **Language** - The language for expressing applications
3. **Service Provider** - Provides services for application development

**MVC within @model:**
- **Form Client Runtime** - Form processing and rendering
- **SDSL Runtime** - Semantic Data Science Language runtime
- **Malloy** - Middleware provider and transformation engine

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│         MVC UserLand Application Layer Protocol           │
│                                                           │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Form Client Runtime                            │   │
│  │  - Form processing                              │   │
│  │  - Display rendering                            │   │
│  │  - Shape:Context:Morph                          │   │
│  └─────────────────────────────────────────────────┘   │
│                        │                                 │
│  ┌─────────────────────┼─────────────────────────────┐ │
│  │  SDSL Runtime        │                             │ │
│  │  - Data modeling    │                             │ │
│  │  - Semantic queries │                             │ │
│  │  - Polars/Arrow     │                             │ │
│  └─────────────────────┼─────────────────────────────┘ │
│                        │                                 │
│  ┌─────────────────────▼─────────────────────────────┐ │
│  │  Malloy (Middleware Provider)                      │ │
│  │  - Transformations                                 │ │
│  │  - Relationships                                   │ │
│  │  - Model-View (Dyadic)                            │ │
│  │  - Service orchestration                           │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## MVC as Protocol

### 1. Application Layer Protocol
- **Defines** how applications communicate
- **Standardizes** application structure
- **Provides** common patterns and conventions

### 2. Language
- **Expresses** application logic
- **Defines** data models and views
- **Describes** transformations and relationships

### 3. Service Provider
- **Provides** runtime services
- **Orchestrates** components
- **Manages** application lifecycle

## MVC Runtime Components

### Form Client Runtime
- **Form processing** - Shape:Context:Morph machinery
- **Display rendering** - DisplayShape-based rendering
- **Form state** - Form state management
- **User interactions** - Event handling

### SDSL Runtime
- **Data modeling** - Malloy-inspired semantic models
- **Query execution** - Polars/Arrow execution
- **Data transformations** - Data processing
- **Analytics** - EDA operations

### Malloy (Middleware Provider)
- **Transformations** - Data and form transformations
- **Relationships** - Joins, hierarchies, semantic relationships
- **Model-View** - Dyadic structure (State:Structure, Representation:Perspective)
- **Service orchestration** - Service coordination

## Malloy as MVC Internal Bus

**Malloy is NOT a NestJS replacement - Different roles:**
- **NestJS** - For Task Processor and Agent System (orchestration)
- **Malloy** - MVC-only Internal Bus connecting Logic and Model

**Malloy serves as an MVC-only Internal Bus** that:
- ✅ **Connects Logic (GDSL) and Model (SDSL)**
- ✅ **Provides unified interface** for Task Processor
- ✅ **Maintains clear split** at GDSL/SDSL level
- ✅ **Focused on transformations and relationships**
- ✅ **MVC-only scope** - Not for Task Processor

**NestJS features we might need:**
- Dependency injection
- Module system
- Decorators
- Guards/Interceptors
- Exception handling

**Malloy can provide:**
- Service orchestration (simpler than NestJS)
- Middleware pipeline
- Transformation system
- Relationship management

## Malloy as General Middleware Provider

### Service Architecture

```
Application
    ↓
Malloy Middleware Provider
    ├─→ Service Registry
    ├─→ Transformation Pipeline
    ├─→ Relationship Manager
    └─→ Model-View Coordinator
    ↓
Services
    ├─→ Form Service
    ├─→ Data Service
    ├─→ Query Service
    └─→ Display Service
```

### Service Definition

```typescript
// Malloy Service (simpler than NestJS)
interface MalloyService {
  id: string;
  name: string;
  // Service configuration
  config: {
    model: string; // Malloy model
    view?: string; // Malloy view
    transformations?: string[]; // Morph IDs
  };
  // Service methods
  methods: {
    [methodName: string]: {
      input: any;
      output: any;
      malloy: {
        view?: string;
        transformations?: string[];
      };
    };
  };
}
```

### Service Orchestration

```typescript
// Malloy Service Orchestrator
class MalloyServiceOrchestrator {
  private services: Map<string, MalloyService>;
  private middleware: MalloyMiddlewareConfig;

  // Register service
  register(service: MalloyService): void {
    this.services.set(service.id, service);
  }

  // Call service method
  async call(serviceId: string, method: string, params: any): Promise<any> {
    const service = this.services.get(serviceId);
    if (!service) throw new Error(`Service ${serviceId} not found`);

    const methodDef = service.methods[method];
    if (!methodDef) throw new Error(`Method ${method} not found`);

    // Use Malloy middleware
    const view = this.middleware.views[methodDef.malloy.view];
    return view.execute(params);
  }
}
```

## Comparison: Malloy vs NestJS

| Feature | NestJS | Malloy |
|---------|--------|--------|
| **Complexity** | High (full framework) | Low (middleware provider) |
| **Focus** | General-purpose | Transformations & Relationships |
| **Structure** | Modules, Controllers, Services | Model-View (Dyadic) |
| **Dependency Injection** | Built-in | Simple service registry |
| **Middleware** | Guards, Interceptors | Transformation pipeline |
| **Data Modeling** | External (TypeORM, Prisma) | Built-in (Malloy Model) |
| **Transformations** | Manual | Declarative (Morph) |
| **Relationships** | Manual joins | Built-in (Malloy joins) |

## Malloy Service Provider Architecture

```
┌─────────────────────────────────────────────────────────┐
│         Malloy Service Provider                          │
│                                                           │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Service Registry                                │   │
│  │  - Service registration                          │   │
│  │  - Service discovery                             │   │
│  └─────────────────────────────────────────────────┘   │
│                        │                                 │
│  ┌─────────────────────┼─────────────────────────────┐ │
│  │  Transformation     │                             │ │
│  │  Pipeline           │                             │ │
│  │  - Morph system     │                             │ │
│  │  - Pure/fusible     │                             │ │
│  └─────────────────────┼─────────────────────────────┘ │
│                        │                                 │
│  ┌─────────────────────┼─────────────────────────────┐ │
│  │  Relationship       │                             │ │
│  │  Manager            │                             │ │
│  │  - Joins            │                             │ │
│  │  - Hierarchies      │                             │ │
│  └─────────────────────┼─────────────────────────────┘ │
│                        │                                 │
│  ┌─────────────────────▼─────────────────────────────┐ │
│  │  Model-View Coordinator                           │ │
│  │  - State:Structure                                │ │
│  │  - Representation:Perspective                     │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## Implementation Strategy

### Phase 1: Basic Service Provider
- Service registry
- Service method calls
- Malloy integration

### Phase 2: Middleware Pipeline
- Transformation pipeline
- Relationship manager
- Model-View coordinator

### Phase 3: Advanced Features
- Dependency injection (simpler than NestJS)
- Service lifecycle management
- Error handling
- Caching

## Key Insights

1. **MVC as Protocol** - UserLand Application Layer Protocol
2. **MVC Runtime** - Form Client + SDSL + Malloy
3. **Malloy as NestJS Replacement** - Simpler, focused on transformations
4. **General Middleware Provider** - Service orchestration
5. **Dyadic Structure** - Model-View (State:Structure, Representation:Perspective)

## Next Steps

- [ ] Design Malloy Service Provider architecture
- [ ] Implement service registry
- [ ] Build transformation pipeline
- [ ] Create relationship manager
- [ ] Integrate with Form Client and SDSL runtimes

