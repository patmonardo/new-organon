# PCI Adapter Pattern (RealityPipe)

## The Pattern

The `@model` package follows a **PCI Adapter pattern**—a hardware analogy applied to software:

```
Universal Substrate (Server)
    ↓
MVC (Standard Adapter Device)
    ↓
@model (Extension of MVC)
    ├─ Logic Form Processor (Source)
    ├─ React Adaptation
    ├─ Radix Adaptation
    └─ Other Adaptations
        ↓
Runtime Adapters
    ├─ React UI
    ├─ Radix UI
    └─ Other UIs
```

## Hardware Precedent

**PCI Adapters:**
- **PCI slot** = Standard interface on motherboard
- **PCI adapter** = Card that plugs into slot
- **Various devices** = Graphics, network, sound, etc.

**Key characteristics:**
- Standard interface enables compatibility
- Plug and play components
- Component reuse and modularity

## Software Application

**@model as Adapter:**
- **Adapter** = @model adapts Logic Form Processor
- **To Universal MVC** = Standard adapter device
- **Extension** = Extends MVC into various adaptations

```
Logic Form Processor (@logic)
    ↓
@model (Adapter/Extension)
    ↓
Universal MVC
    ↓
Runtime Adapters
```

## MVC as Standard Adapter

**MVC is the standard adapter device that hooks into the Server:**

| PCI Hardware | MVC Software |
|--------------|--------------|
| PCI Bus | Universal substrate |
| PCI Slot | MVC Interface |
| Adapter Card | @model Extension |
| Device Driver | Runtime Adapter |

## The Extension Architecture

**@model extends MVC into various adaptations:**

```
Universal MVC (Standard Adapter Device)
    ↓
@model (Extension of MVC)
    ├─ DisplayDocument (Standard Output)
    ├─ React Adaptation
    ├─ Radix Adaptation
    └─ Agent Adaptation
```

## Benefits

1. **Proven pattern** — Hardware has done this for decades
2. **Standard interfaces** — Enable compatibility
3. **Modularity** — Component reuse
4. **Extensibility** — Easy to add new adapters
5. **Plug and play** — Adapters work with any MVC

## Implementation

### The Adapter Interface

```typescript
export interface Adapter {
  name: string;
  translate(document: DisplayDocument, context?: RenderContext): unknown;
}
```

### Standard Adapters

```typescript
// React Adapter
export class ReactAdapter implements Adapter {
  name = 'react';
  translate(document: DisplayDocument, context?: RenderContext): React.ReactNode;
}

// Radix Adapter  
export class RadixAdapter implements Adapter {
  name = 'radix';
  translate(document: DisplayDocument, context?: RenderContext): React.ReactNode;
}

// JSON Adapter (serialization)
export class JSONAdapter implements Adapter {
  name = 'json';
  translate(document: DisplayDocument): string;
}
```

## The Complete Picture

```
User Code (SDSL/MVC)
    ↓
@logic (Form Processor)
    ↓
@model (MVC Extension/Adapter)
    ↓
DisplayDocument (Standard Output)
    ↓
Runtime Adapter (PCI-style)
    ├─ React Adapter
    ├─ Radix Adapter
    └─ Other Adapters
    ↓
Runtime UI
```

This is a hardware-inspired modularity pattern applied to the Model substrate.

