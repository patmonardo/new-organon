# Projection as Concealing / Revealing Power (Factory → Eval)

This note pins down the **two-sided** meaning of “Projection” in this repo.

- **Projection/Factory** produces an **Image** (for us: a `GraphStore`).
- **Projection/Eval** executes an evaluation that **reveals** derived structure/results from that Image.

This matches the intuition that to *reveal* determinately, we must first *conceal*.

## Terms (minimal, operational)

### Concealing Power (Factory)

**Concealing** is the act of producing an internal, operable image while **withholding** (abstracting away) the source’s contingent details.

In code, this is:

- `gds/src/projection/factory/**`
- The public ingestion contract: `GraphStoreFactory` / `GraphStoreFactoryTyped`

**Input**: external/native data representation (Arrow tables today; other sources later).

**Output (Image)**: `GraphStore` (typically `DefaultGraphStore`).

**What is “concealed”** (examples):

- source-specific cursor/scan details
- physical storage choices (Vec/Huge/Arrow) as an implementation strategy
- raw ID domains (source IDs) behind an internal ID map

### Revealing Power (Eval)

**Revealing** is the act of determinately making explicit what is *implicit* in the Image.

In code, this is:

- `gds/src/projection/eval/**`
- The “Three ISA” execution systems (Procedure / ML / Form)

**Input**: an Image (`GraphStore`) plus a spec/config.

**Output**: results, mutations, models, traces (depending on mode).

## The architecture claim

Projection is not just “loading” and not just “eval”. It is a **two-stage architecture**:

1) **Factory (Conceal)**: Source → Image (`GraphStore`)
2) **Eval (Reveal)**: Image → Derived result (streams/stats/mutate/write/train)

Neo4j treats “projection” primarily as stage (1). This repo treats stage (2) as projection too, because evaluation is also a determinate “making-appear”.

## Invariants (what we build against)

- **Image invariance**: the `GraphStore` is the stable boundary between stages.
- **Source opacity**: Eval must not depend on source details (Arrow/Polars/Neo4j should be irrelevant once we have the Image).
- **Backend swappability**: storage backend choice is an internal strategy (preferably injected via Collections factories), not a semantic part of Projection.
- **Determinacy**: Revealing must be explainable as a transformation of the Image, not magic (this is where progress/trace belongs).

## RealityFabric: the five-fold “fat pipe”

In this repo, **Fabric** names the universal interconnect between the kernel
(Projection Factory/Eval) and the substrate/reality. It is the thing that
“plugs into Reality” (i.e. the non-negotiable conditions under which computation
becomes real: policy, time, accountability).

Wheel framing (brahmachakra): Reality is modeled as a **Wheel** — a **4-fold rim**
(quadrants) plus the **center conjunction**.

- **Rim (four-fold)**: Storage, Compute, Control, Time
- **Center (conjunction)**: Witness

Operationally, the RealityFabric is still **five-fold**:

1) **Storage** — persistence/materialization surfaces
2) **Compute** — CPU/GPU allocation/execution surfaces
3) **Control** — identity/tenancy/policy labels
4) **Time** — budgets/leases/deadlines
5) **Witness** — trace/audit/proof sinks

Concrete anchors in code today:

- Storage surface for Form: `FormStoreSurface` in `gds/src/substrate/surface.rs`
- Compute surface for suites: `ComputeSurface` in `gds/src/substrate/compute_surface.rs`
- Control/Time envelopes exist as the enterprise seams (`StorageMeta` / `ComputeMeta`)
- Witness exists as Form proof JSON, and will grow a sink boundary

The long-term intent is that Reality-facing proc-macros codegen bindings against
this fabric: Collections factories, config surfaces, and the Value/ValueType +
property-type plumbing that makes projections executable.

## Relation to the Logic package

The Logic package already defines Projection as **revealing/concealing** at the concept level:

- `logic/doc/concepts/projection.md`

The intent here is to align GDS’s engineering architecture (Factory → Eval) with that conceptual split.
