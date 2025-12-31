# Reality's Self-Projection: The Original Concept

## The Fundamental Architecture

**Reality** (`reality/`) is the source of **Being qua Being science**. It is not in GDS - it is in Reality itself.

**Reality projects itself** - this is the original Concept.

## The Three Stages of Self-Projection

### Stage 1: Fabric → Projection Machinery

**First**, the Fabric manifests as the **Projection Machinery itself**.

This as **Factory** manifests:
- **Computation fabrics** via RealityFabric
- **Storage fabrics** via RealityPipe

```
Reality (Being qua Being)
  │
  │ (projects itself)
  │
  └─→ Stage 1: Fabric → Projection Machinery
      │
      └─→ Factory manifests:
          ├─→ Computation fabrics (RealityFabric)
          └─→ Storage fabrics (RealityPipe)
```

**Key Components**:
- `reality/src/` - The Reality crate (Being qua Being science)
- `gds/src/substrate/reality_fabric.rs` - RealityFabric (five-fold pipe)
- `model/src/sdsl/reality-pipe.ts` - RealityPipe (in-process fabric)

### Stage 2: GDSL Fabric Link

**Second**, the **GDSL Fabric Link** manifests.

This is the link between the GDS kernel and the TypeScript layer.

```
Reality (Being qua Being)
  │
  │ (projects itself)
  │
  ├─→ Stage 1: Fabric → Projection Machinery
  │   └─→ Factory → Computation/Storage fabrics
  │
  └─→ Stage 2: GDSL Fabric Link
      └─→ Link between GDS kernel and TS layer
```

**Key Components**:
- `gdsl/src/sdk/gds-tsjson-kernel-port.ts` - GDSL Fabric Link
- `gds/src/applications/services/tsjson_napi.rs` - TS-JSON NAPI boundary

### Stage 3: The Agent

**Third**, **the Agent herself** manifests.

This is the final stage of Reality's self-projection.

```
Reality (Being qua Being)
  │
  │ (projects itself)
  │
  ├─→ Stage 1: Fabric → Projection Machinery
  │   └─→ Factory → Computation/Storage fabrics
  │
  ├─→ Stage 2: GDSL Fabric Link
  │
  └─→ Stage 3: The Agent
      └─→ The agent herself manifests
```

**Key Components**:
- `model/src/sdsl/agent-runtime.ts` - Agent-side projection runtime
- `model/src/sdsl/agent-adapter.ts` - Agent adapter
- `logic/src/relative/form/eval/` - Agent form evaluation

## The Complete Architecture

```
Reality (Being qua Being Science)
  │
  │ (projects itself - the original Concept)
  │
  ├─→ Stage 1: Fabric → Projection Machinery
  │   │
  │   └─→ Factory manifests:
  │       ├─→ Computation fabrics (RealityFabric)
  │       └─→ Storage fabrics (RealityPipe)
  │
  ├─→ Stage 2: GDSL Fabric Link
  │   │
  │   └─→ Link between GDS kernel and TS layer
  │
  └─→ Stage 3: The Agent
      │
      └─→ The agent herself manifests
```

## Key Principles

1. **Reality is the source** - Being qua Being science is in Reality, not GDS
2. **Reality projects itself** - this is the original Concept
3. **Mathematical computations** happen also in the GDS kernel (and in Reality)
4. **The three stages** are manifestations of Reality's self-projection
5. **The Agent** is the final stage - the agent herself manifests

## RealityFabric and RealityPipe

### RealityFabric

**RealityFabric** is the five-fold "fat pipe" connection into reality:

- **Rim (four-fold)**: Storage, Compute, Control, Time
- **Center (conjunction)**: Witness (proof/trace/audit)

```
RealityFabric (Five-fold pipe)
  ├─ Storage  - persistence/materialization surfaces
  ├─ Compute  - CPU/GPU allocation/execution surfaces
  ├─ Control  - identity/tenancy/policy labels
  ├─ Time     - budgets/leases/deadlines
  └─ Witness  - trace/audit/proof sinks
```

### RealityPipe

**RealityPipe** is the interface functor that exposes the minimal typed contract for the fabric:

- `read(filters): ReadView` - idempotent query returning typed `PrintEnvelope`s
- `print(PrintEnvelope): Promise<void>` - append-only publish

## The Organic Unity

RealityFabric is the **projection medium** for the system's Organic Unity:

- **GDS (kernel execution)** emits structured artifacts onto the fabric
- **Agent / TS discourse** subscribes to the same fabric
- **Organic Unity** = the closure when both sides share one fabric of prints

## References

- `reality/doc/REALITYFABRIC-PROJECTS-ORGANIC-UNITY.md` - RealityFabric as projection medium
- `gds/src/substrate/reality_fabric.rs` - RealityFabric implementation
- `model/src/sdsl/reality-pipe.ts` - RealityPipe implementation
- `gds/doc/ABSOLUTE-SCIENCE-PRINCIPLE.md` - Absolute Science principle

