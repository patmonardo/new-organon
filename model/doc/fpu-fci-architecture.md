# FPU and FCI Architecture

## The Core Insight

The `@logic:@model` dyad is exactly what **Reflective Agents** need—they create Logical Models. The Agent hooks into the same interface as React clients.

## Naming

| Concept | Name | Analogy |
|---------|------|---------|
| **FPU** | Form Processor Unit | CPU:GPU (sublated) |
| **FCI** | Form Component Interface | PCI bus |

## FPU: Form Processor Unit

The **FPU** sublates the CPU:GPU dyad:

| Component | Handles | Domain |
|-----------|---------|--------|
| **CPU** | Qualitative Properties | Logic, Relations, Structure |
| **GPU** | Quantitative Properties | Numbers, Aggregates, Analytics |
| **FPU** | Both (sublation) | The complete Form Processor |

```
┌─────────────────────────────────────────────────────────┐
│                    FPU (Form Processor Unit)            │
│                                                         │
│   ┌─────────────────┐     ┌─────────────────┐          │
│   │      CPU        │     │      GPU        │          │
│   │   Qualitative   │ ←→  │   Quantitative  │          │
│   │   Properties    │     │   Properties    │          │
│   │                 │     │                 │          │
│   │  Logic, Shape   │     │  Numbers, Agg   │          │
│   │  Relations      │     │  Analytics      │          │
│   └─────────────────┘     └─────────────────┘          │
│                                                         │
│            @logic - Reflection, 6 Pillars               │
└──────────────────────────┬──────────────────────────────┘
                           │
                           │ speaks FormShape
                           ▼
```

## FCI: Form Component Interface

The **FCI** is the bus + adapter interface:

```
                           │
                           ▼
┌────────────────────────────────────────────────────────────────────────┐
│                     FCI (Form Component Interface)                      │
│                                                                         │
│  FormBus + form-{model,view,controller} = Universal Adapter Interface   │
└────────┬─────────────────┬─────────────────┬─────────────────┬─────────┘
         │                 │                 │                 │
         ▼                 ▼                 ▼                 ▼
    ┌─────────┐      ┌─────────┐      ┌─────────┐      ┌─────────┐
    │  React  │      │  Radix  │      │  Agent  │      │  Agent  │
    │ Adapter │      │ Adapter │      │ Adapter │      │ Adapter │
    │         │      │         │      │  (A)    │      │  (B)    │
    └────┬────┘      └────┬────┘      └────┬────┘      └────┬────┘
         │                 │                 │                 │
         ▼                 ▼                 ▼                 ▼
      React UI         Radix UI         Agent Task        Agent Task
```

## The FCI Components

The **FCI** (Form Component Interface) consists of:

1. **FormBus** — The communication channel
2. **form-{model,view,controller}** — The adapter interface
3. **Adapters** — Components that plug into the FCI

### What the FCI Provides

```typescript
// FCI = FormBus + form-{model,view,controller}
// The adapter interface that handles access to the FPU

FormModel      → State : Structure           (holds data, validates)
FormView       → Representation : Perspective (transforms to display)
FormController → Action : Rule               (orchestrates, handles actions)
```

### Adapter Economics

Just like people pay $$$ for PCI adapter chips, this is what's happening:

| PCI World | FCI World |
|-----------|-----------|
| PCI Bus | FormBus |
| PCI Interface Spec | FCI (Form Component Interface) |
| Adapter Chip | form-{model,view,controller} |
| Graphics Card | React Adapter |
| Network Card | Radix Adapter |
| AI Accelerator | Agent Adapter |
| Device Driver | Runtime-specific code |

## Chain of Adapters

An App can deploy a **chain of Adapters** on the FCI:

```
FPU (Form Processor Unit)
      │
      │ FormShape
      ▼
   FCI (Form Component Interface)
      │
      ├─→ React Adapter → React UI (User sees dashboard)
      │
      ├─→ Agent Adapter A → Reasoning Task → 
      │         │
      │         └─→ Produces new FormShape → Back to FCI
      │
      └─→ Agent Adapter B → Analysis Task →
                │
                └─→ Produces insights → Feeds React Adapter
```

**Results from the FPU can feed into:**
1. React Clients (immediate display)
2. Chain of Agents (reasoning, analysis, transformation)
3. Back to FCI (agents produce new FormShapes)

## The Logic:Model Dyad for Agents

**Reflective Agents need to create Logical Models**—this is exactly what the dyad provides:

```
Agent Task
    │
    │ needs to reason about
    ▼
@logic (FPU - Form Processor Unit)
    │
    │ provides schemas, reflection, 6 Pillars
    │ CPU: Qualitative | GPU: Quantitative
    ▼
@model (FCI - Form Component Interface)
    │
    │ provides FormBus, adapters, DisplayDocument
    ▼
Agent creates Logical Model
    │
    │ FormShape flows back to FCI
    ▼
Other adapters consume the result
```

## Implementation

### FCI Interface

```typescript
// The FCI (Form Component Interface)
export interface FormComponentInterface {
  // FormBus for communication
  bus: FormBus;
  
  // Adapter registration
  registerAdapter(adapter: FCIAdapter): void;
  
  // FormShape distribution
  publish(shape: FormShape): void;
  
  // Chain execution
  executeChain(shapes: FormShape[], adapters: string[]): Promise<ChainResult>;
}
```

### Adapter Interface

```typescript
export interface FCIAdapter {
  name: string;
  
  // Receive FormShape from FCI
  receive(shape: FormShape, context: FCIContext): Promise<AdapterResult>;
  
  // Publish result back to FCI (for chaining)
  publish?(result: AdapterResult): FormShape;
}
```

### Agent as Adapter

```typescript
// Agent is just another adapter on the FCI
export class AgentAdapter implements FCIAdapter {
  name = 'agent';
  
  async receive(shape: FormShape, context: FCIContext): Promise<AdapterResult> {
    // Agent receives FormShape
    // Reasons about it using @logic schemas (FPU)
    // Creates new Logical Models
    // Returns result (potentially new FormShapes)
  }
  
  publish(result: AdapterResult): FormShape {
    // Agent produces new FormShape for next adapter in chain
  }
}
```

## The Chain Pattern

```typescript
// App deploys chain of adapters on FCI
const app = createFormApp({
  fci: new FormComponentInterface(),
  chain: [
    'react',      // Display initial state
    'agent-a',    // Reason about state
    'agent-b',    // Analyze results
    'react',      // Display final results
  ],
});

// Results flow through the chain
await app.process(initialFormShape);
```

## Summary

| Term | Full Name | Role |
|------|-----------|------|
| **FPU** | Form Processor Unit | CPU:GPU sublation, processes forms |
| **FCI** | Form Component Interface | Bus + adapter interface |
| **FormBus** | Form Bus | Communication channel within FCI |

The **FCI** unifies:

1. **UI Adapters** (React, Radix) — Display FormShapes to users
2. **Agent Adapters** — Reason about FormShapes, create Logical Models
3. **Chain Execution** — Results from one adapter feed the next

Both UI components and Agents are just different adapters on the same FCI. The dyad (`@logic:@model` = `FPU:FCI`) serves them equally—agents create Logical Models the same way UI creates displays.

---

> "An App can deploy a chain of Adapters on the Bus. Results from the FPU can feed into React Clients and to a chain of Agents."

This is the **FPU:FCI Architecture**.

