# FPU and RealityPipe Architecture

## The Core Insight

The `@logic:@model` dyad is exactly what **Reflective Agents** need—they create Logical Models. The Agent hooks into the same interface as React clients.

## Naming

| Concept | Name | Analogy |
|---------|------|---------|
| **FPU** | Form Processor Unit | CPU:GPU (sublated) |
| **RealityPipe** | Reality Pipe | in-process pipe |

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

## RealityPipe: Adapter Pipe

The **RealityPipe** is the in-process pipe + adapter interface:

```
                           │
                           ▼
┌────────────────────────────────────────────────────────────────────────┐
│                         RealityPipe (in-process)                         │
│                                                                         │
│  RealityPipe + form-{model,view,controller} = Universal Adapter Interface│
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

## The RealityPipe Components

The **RealityPipe** consists of:

1. **RealityPipe** — The communication channel
2. **form-{model,view,controller}** — The adapter interface
3. **Adapters** — Components that plug into the pipe

### What the Pipe Provides

```typescript
// RealityPipe = RealityPipe (event envelope) + form-{model,view,controller}
// The adapter interface that handles access to the FPU

FormModel      → State : Structure           (holds data, validates)
FormView       → Representation : Perspective (transforms to display)
FormController → Action : Rule               (orchestrates, handles actions)
```

### Adapter Economics

Just like people pay $$$ for PCI adapter chips, this is what's happening:

| PCI World | RealityPipe World |
|-----------|-----------|
| PCI Bus | RealityPipe |
| PCI Interface Spec | RealityPipe (Adapter Pipe) |
| Adapter Chip | form-{model,view,controller} |
| Graphics Card | React Adapter |
| Network Card | Radix Adapter |
| AI Accelerator | Agent Adapter |
| Device Driver | Runtime-specific code |

## Chain of Adapters

An App can deploy a **chain of Adapters** on the RealityPipe:

```
FPU (Form Processor Unit)
      │
      │ FormShape
      ▼
  RealityPipe (in-process)
      │
      ├─→ React Adapter → React UI (User sees dashboard)
      │
      ├─→ Agent Adapter A → Reasoning Task → 
      │         │
      │         └─→ Produces new FormShape → Back to RealityPipe
      │
      └─→ Agent Adapter B → Analysis Task →
                │
                └─→ Produces insights → Feeds React Adapter
```

**Results from the FPU can feed into:**
1. React Clients (immediate display)
2. Chain of Agents (reasoning, analysis, transformation)
3. Back to RealityPipe (agents produce new FormShapes)

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
@model (RealityPipe)
    │
    │ provides RealityPipe, adapters, DisplayDocument
    ▼
Agent creates Logical Model
    │
    │ FormShape flows back to RealityPipe
    ▼
Other adapters consume the result
```

## Implementation

### RealityPipe Interface

```typescript
// The RealityPipe (in-process)
export interface FormComponentInterface {
  // RealityPipe for communication
  pipe: RealityPipe;
  
  // Adapter registration
  registerAdapter(adapter: Adapter): void;
  
  // FormShape distribution
  publish(shape: FormShape): void;
  
  // Chain execution
  executeChain(shapes: FormShape[], adapters: string[]): Promise<ChainResult>;
}
```

### Adapter Interface

```typescript
export interface Adapter {
  name: string;
  
  // Receive FormShape from the pipe
  receive(shape: FormShape, context: AdapterContext): Promise<AdapterResult>;
  
  // Publish result back to the pipe (for chaining)
  publish?(result: AdapterResult): FormShape;
}
```

### Agent as Adapter

```typescript
// Agent is just another adapter on the pipe
export class AgentAdapter implements Adapter {
  name = 'agent';
  
  async receive(shape: FormShape, context: AdapterContext): Promise<AdapterResult> {
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
// App deploys chain of adapters on the pipe
const app = createFormApp({
  pipe: new FormComponentInterface(),
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
| **RealityPipe** | Reality Pipe | Pipe + adapter interface |

The **RealityPipe** unifies:

1. **UI Adapters** (React, Radix) — Display FormShapes to users
2. **Agent Adapters** — Reason about FormShapes, create Logical Models
3. **Chain Execution** — Results from one adapter feed the next

Both UI components and Agents are just different adapters on the same RealityPipe. The dyad (`@logic:@model` = `FPU:RealityPipe`) serves them equally—agents create Logical Models the same way UI creates displays.

---

> "An App can deploy a chain of Adapters on the Bus. Results from the FPU can feed into React Clients and to a chain of Agents."

This is the **FPU:RealityPipe Architecture**.

