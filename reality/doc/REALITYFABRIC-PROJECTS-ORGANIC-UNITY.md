# RealityFabric Projects Organic Unity (Target vs Emulation)

This note records an architectural intent that is easy to lose when we only look at the current code paths.

## Claim

The `@organon/reality` substrate (RealityFabric) is the *projection medium* for the system’s Organic Unity.

- The RS–TS split is not “backend vs frontend.”
- It is the bridge where **Knowing (kernel act)** is rendered into **prints** (events/IR/traces) for **Conceiving/Thinking**.
- RealityFabric is the medium in which that bridge becomes a shared, replayable, multi-surface execution stream.

## Target architecture (what RealityFabric should manifest)

RealityFabric is the *same* projection surface for both:

1) **GDS (kernel execution)**
- Kernel runs (procedures/ML/forms) emit structured artifacts.
- Those artifacts are published onto the fabric as envelopes with correlation + provenance.

2) **Agent / TS discourse**
- Agents subscribe to the same fabric.
- They interpret, judge, narrate, and render prints (plans/acts/results/traces/UI).

The “Organic Unity” is not an extra component.
It is the closure that exists when both sides share one fabric of prints.

## Where we are today (safe emulation)

We are not yet running the full RealityFabric across GDS + Agent.
Instead, TS land **emulates the target** with an in-process fabric that preserves the key invariants:

- Envelope + correlation + provenance
- Deterministic “print” surfaces (events/traces)
- Plural subscribers (multi-agent possibility)

Concrete pieces already present:

- In-process fabric: `model/src/sdsl/reality-pipe.ts`
  - Provides an envelope (`id`, `ts`, `kind`, `payload`, optional `meta`, `correlationId`, `source`).
  - Supports subscription/publishing without committing to transport.

- Agent-side projection runtime: `model/src/sdsl/agent-runtime.ts`
  - Maintains a trace window and yields a ContextDocument (discursive appearance).

- Kernel execution boundary (adapter): `model/src/sdsl/kernel-port.ts`
  - The kernel is invoked as an execution port, not as a discursive narrator.

- Organic unity emulation (Knowing → Print): `model/src/sdsl/kernel-organic-unity.ts`
  - Runs a kernel request, then produces:
    - TAW prints (`taw.act`, `taw.result`)
    - FactTrace prints (`kernel.run.request`, `kernel.run.result`)

This is an intentional “as-if” implementation:
- we treat the TS in-memory fabric as if it were the eventual RealityFabric,
- so the surrounding logic (correlation discipline, print surfaces, multi-subscriber semantics) is already correct.

## The rule this preserves

Aphorism: Docs are prints of knowing; kernels are acts of knowing.

RealityFabric generalizes that:

- **Everything that crosses a boundary is a print** (an envelope/event/IR/trace).
- The kernel act is never shipped as an act; only its prints are.

## Migration path (no premature bindings)

- Keep “fabric” APIs structural and transport-agnostic.
- Keep kernel APIs artifact-only (no prose reasons required).
- Upgrade the emulated TS fabric to a real cross-runtime fabric when the integration surface is ready.

Until then, the emulation is not a hack: it is a deliberate way to implement the *form* of the target system early.

## Sat Karya (Absolute Print) & Interface Functor

RealityFabric's **Fat Pipe** is the *Absolute Print* (Sat Karya): a canonical, append-only, provenance-bearing envelope that mediates kernel **Knowing** and agent **Conceiving**. The RealityPipe is the interface functor that exposes the minimal typed contract for the fabric:

- `read(filters): ReadView` — an idempotent query returning typed `PrintEnvelope`s and derived state (no provenance mutation).
- `print(PrintEnvelope): Promise<void>` — append-only publish of an immutable, ordered `PrintEnvelope`.

Minimal `PrintEnvelope` obligations:
- Required fields: `id`, `kind`, `timestamp`, `role` (`kernel|user|system`), `provenance`, `payload`, `schemaVersion`.
- `derivedFrom` links lineage (e.g., conceiving prints referencing knowing prints).
- Kernel prints (`role: kernel`) represent canonical acts-of-knowing; user/agent prints (`role: user`) represent discourse/conceiving.

Implementation notes (short): add a typed `PrintEnvelope` schema (TS Zod; optional Rust Serde), extend `model/src/sdsl/reality-pipe.ts` to export typed `read` and `print` primitives, and add unit/integration tests that validate kernel vs user prints and `read` semantics.

### Aggregated ReadViews (Essence snapshots)

The fabric exposes an `aggregate` option on `read()` to produce an aggregated snapshot (a ReadView.aggregated) from a stream of prints. Aggregation uses a `groupBy` key and a `reducer`. The recommended default reducer is **`conclusive-latest`**: prefer `epistemicLevel === 'conclusive'` that meets configured thresholds, otherwise fall back to the latest print. This is how the fabric can surface *Essence* — stable, canonical summaries — from noisy, temporal prints.

Default thresholds:
- `minConfidence`: 0.8
- `minEvidence`: 1
- `epistemicFloor`: `'inferred'`

Use `aggregate` to request snapshots (e.g., latest conclusive fact per `subject`) or other rollups (count, numeric rollup, custom reducer functions).

## Principle / Action / Concept / Essence: Glossary & Flow

A concise mapping of the metaphysical frame into the Reality fabric:

- **Principle** — the root of knowing; kernel-level tacit signals and summaries expressed by prints and their lifecycle.
- **Action** — enactment (TAW prints, task runs), the operational side of the system.
- **Concept** — discursive understanding / conceiving; structured proofs produced by `logic/` or agents (`kind: 'conceiving'`, `epistemicLevel: 'conclusive'` when adjudicated).
- **Essence** — the appearing norm / canonical fact; represented by aggregated ReadView snapshots (Essence snapshots) produced via `conclusive-latest` aggregation.

### Monadic vs Triadic discriminator

The fabric records a lightweight ontology discriminator on prints to surface metaphysical shape at the surface level:

- `ontology: 'monadic'` — kernel-level / **Being** prints (GDS). Use for Prajna/Kriya signals and existence-level facts.
- `ontology: 'triadic'` — logic/agent-level / **Understanding** prints (TS). Use for Jnana/Dharma adjudication and conceptualization.

This discriminator is advisory and primarily used by tooling/tests to maintain the intended architectural separation: GDS remains monadic and the TS concept layer remains triadic. Tests in the repository assert and encode this separation; teams can opt into different ownership models if they document and enable explicit configuration.
Example flow (Principle → Action → Concept → Essence):

1. Kernel emits a **Principle** print (e.g., `kind: 'knowing'` with a summary and trace).
2. The system enacts an **Action** (TAW action) possibly producing further prints.
3. `logic/` consumes the Principle/Action prints and emits a **Concept** print (`kind: 'conceiving'`, `derivedFrom: [principle]`, `epistemicLevel: 'conclusive'`).
4. A `read({ aggregate: { reducer: 'conclusive-latest', groupBy: 'subject' } })` call surfaces the canonical **Essence** snapshot for that subject.

This mapping makes the metaphysical framing practical and testable inside the codebase: Prajna and Jnana are first-class prints and Dharma is the fabric's aggregated appearance of stable facts.

## Governance: Ownership of Epistemic Finality

By default, the **`logic/`** package is the authority to adjudicate and mark prints as **`epistemicLevel: 'conclusive'`** (i.e., produce `jnana`-level conceivings). Kernel (GDS) prints should generally be `prajna` (tacit signals) or `kriya` (actions) and supply evidence; the canonical adjudication and narrative construction happens in the TS `logic/` layer. This convention is not absolute — projects may opt for kernel-first adjudication by mirroring Serde types in `reality/` and implementing kernel-side adjudicators — but the repository's default tests and CI will assume `logic/` as the primary owner of finality.

Implementation notes (short): add policy tests that warn when a kernel-sourced print declares `epistemicLevel: 'conclusive'` without explicit opt-in, and document the configuration flag (`reality.policy.kernelConclusiveAllowed`) for teams that prefer kernel-first authority.
