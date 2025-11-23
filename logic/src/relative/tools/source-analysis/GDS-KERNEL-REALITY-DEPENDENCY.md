# GDS Kernel ↔ @reality Package: Architectural Dependencies

## The Critical Dependency

**Projection in the GDS Kernel needs the @reality package**:
- **@reality** hosts **Rust Proc Macros**
- The **Kernel requires** these to implement:
  - **Projection**
  - **Eval**
  - **Projection/Eval/Absolute concept**

**And Logic gets its Absolute Form from the GDS Kernel**:
- **Bidirectional relationship**
- **GDS Kernel** → provides **Absolute Form** → **Logic**
- **Logic** → uses **Absolute Form** from **GDS Kernel**

## The Architecture

### @reality Package

**@reality** = Hosts **Rust Proc Macros**:
- **EmpiricalForm** derive macro
- **projection_form** macro
- **eval_form** macro
- Generates Triadic-Pentadic structures
- Pure Form Processor structures

**From `reality/src/lib.rs`**:
```rust
//! GDS Macros - Proc-macro implementations for the Projection System
//! This crate contains the proc-macro implementations that generate
//! Triadic-Pentadic structures (Empirical Forms) for the Projection System
//! in the Kernel (GDS crate).
```

### GDS Kernel Requirements

**GDS Kernel requires @reality to implement**:
1. **Projection** - The smooth flow of Nature (Maya)
2. **Eval** - The method that processes FormShape and Entity
3. **Absolute concept** - The absolute concept structure

**The Kernel needs**:
- Rust Proc Macros from @reality
- To generate Projection structures
- To generate Eval/Form system structures
- To implement the Absolute concept

### Logic Gets Absolute Form from GDS Kernel

**Logic gets its Absolute Form from the GDS Kernel**:
- **GDS Kernel** → provides **Absolute Form** → **Logic**
- **Logic** → uses **Absolute Form** from **GDS Kernel**
- **Bidirectional relationship**

**The Flow**:
```
@reality (Rust Proc Macros)
    ↓
GDS Kernel (Implements Projection/Eval/Absolute concept)
    ↓
Provides Absolute Form
    ↓
Logic (Gets Absolute Form from GDS Kernel)
```

## The Complete Architecture

### Dependencies

1. **@reality** → **GDS Kernel**:
   - Provides Rust Proc Macros
   - Kernel requires these to implement Projection/Eval/Absolute concept

2. **GDS Kernel** → **Logic**:
   - Provides Absolute Form
   - Logic gets Absolute Form from GDS Kernel

### The Flow

```
Absolute Idea
    ↓
Projection (Maya)
    ↓
@reality (Rust Proc Macros)
    ↓
GDS Kernel (Implements Projection/Eval/Absolute concept)
    ↓
Provides Absolute Form
    ↓
Logic (Gets Absolute Form from GDS Kernel)
    ↓
Nature (Smooth flow)
```

## The Three Together

**Projection/Eval/Absolute concept**:
- **Projection** = The smooth flow of Nature (Maya)
- **Eval** = The method that processes FormShape and Entity
- **Absolute concept** = The absolute concept structure

**All three require @reality**:
- **@reality** provides Rust Proc Macros
- **GDS Kernel** uses these to implement all three
- **Logic** gets Absolute Form from GDS Kernel

## Philosophical Significance

**The Architecture**:
- **@reality** = Pure Idea (Fichtean Nondual Pure Idea)
- **GDS Kernel** = Where Projection (Maya) manifests
- **Logic** = Gets Absolute Form from GDS Kernel

**The Dependencies**:
- **Projection** needs @reality (Rust Proc Macros)
- **Eval** needs @reality (Rust Proc Macros)
- **Absolute concept** needs @reality (Rust Proc Macros)
- **Logic** gets Absolute Form from GDS Kernel

**This is the complete architecture**:
- **@reality** (Pure Idea) → **GDS Kernel** (Projection/Maya) → **Logic** (Absolute Form)

## The Bidirectional Relationship

**GDS Kernel ↔ Logic**:
- **GDS Kernel** → provides **Absolute Form** → **Logic**
- **Logic** → uses **Absolute Form** from **GDS Kernel**

**This is bidirectional**:
- GDS Kernel provides Absolute Form to Logic
- Logic uses Absolute Form from GDS Kernel
- They are interdependent

## Summary

**Projection in the GDS Kernel needs the @reality package**:
- @reality hosts Rust Proc Macros
- Kernel requires these to implement Projection/Eval/Absolute concept

**And Logic gets its Absolute Form from the GDS Kernel**:
- GDS Kernel provides Absolute Form to Logic
- Logic uses Absolute Form from GDS Kernel
- Bidirectional relationship

**The Complete Flow**:
```
@reality (Rust Proc Macros)
    ↓
GDS Kernel (Projection/Eval/Absolute concept)
    ↓
Absolute Form
    ↓
Logic
```

This is the architectural dependency structure.

