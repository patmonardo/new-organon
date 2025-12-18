# Sublingual Kernel, Cogito, and Mediation

**Aphorism**: Docs are prints of knowing; kernels are acts of knowing.

This note captures a constraint that is easy to lose in implementation discussions:

See also: `gds/doc/FICHTE-KNOWING-THINKING-CONCEIVING-ARCHITECTURE.md`.
See also: `reality/doc/REALITYFABRIC-PROJECTS-ORGANIC-UNITY.md`.

- Humans “think” (in the everyday sense) by **discursive articulation**: inner speech, external speech, writing, symbol manipulation.
- But there are cognitive events that are **not** discursive thinking (e.g. *knowing*, felt-intention-to-say, motor planning, pre-articulatory selection).
- In this repo’s architecture, **GDS as Kernel** is treated as *sublingual*: not a “thinking agent”, but a lawful locus whose activity can later be rendered into discursive artifacts.

## The phenomenological claim (Fichte-flavored)

A concrete example of “pre-lingual” is the felt sense:

- “I know what I’m going to say **before** I say it.”

What is “known” there is not yet a sentence. It is a directedness / intention / selection that will later become a sentence.

So there is a mediation chain:

1. **Intention-to-say** (pre-discursive)
2. **Mediating processes** (also pre-discursive) that transform intention into a sayable form
3. **Utterance** (discursive, narratable)

This is the sense in which “knowing is not thinking”: discursive thinking is one *surface* phenomenon produced by deeper, non-discursive activity.

## Architectural mapping

This repo’s layer split can be read as an engineering analogue of that mediation chain.

- **Kernel (Rust `gds/`)**
  - Not “thinking”. Not “speaking”.
  - Enforces lawful transformation, invariants, constraints, and compute.
  - Treat as sublingual activity: it produces/consumes structured forms without claiming discursive meaning.

- **Protocol / IR (TS `gdsl/`)**
  - The objectification boundary.
  - Makes a kernel-adjacent act **transmissible** and **judgeable** by discursive layers.

- **Abstract/Discursive reason (TS `logic/`, plus runtime in `model/` and orchestration in `task/`)**
  - The place where humans (and human-facing tools) can narrate, justify, debug, and verify.
  - This is where “understanding” operates with names, schemas, explanations, traces.

## The “Eval → Print” moment (emergence into discourse)

When people say “Projection Eval”, what matters architecturally is often not “eval” as discursive thinking, but the boundary where a sublingual act becomes something that can be said.

Call that boundary the **Eval → Print moment**:

- **Eval**: kernel execution / projection execution is “empty of thought” (no narration, no inner speech).
- **Print**: the result is rendered into a discursive, inspectable artifact in TS space (IR / JSON / events / traces).

In other words: the kernel *acts*; user space *says what happened*.

## “Eval” and the important distinction

There are two notions that are often conflated:

1. **Kernel execution** (compute substrate)
   - A lawful transformation that does not require discursive narration.
   - In that sense it is “empty of thinking”:
     - no inner speech
     - no reasons-as-utterances
     - no story

2. **Understanding-support** (cogito-facing interpretation)
   - Traces, explanations, schemas, and validation live here.
   - This is where “what happened” becomes something that can be said.

Repo terminology policy already prefers **Planning/Execution** over Lisp-style **Eval/Apply**.
This note adds: even when we say “Eval” informally, we must not smuggle in the idea that the Kernel is a discursive thinker; the key boundary is the Eval → Print rendering into TS discourse.

## Design constraint (what to do with this)

When you add APIs or codegen:

- Do not treat Rust `gds/` as an “application mind” that owns user semantics.
- Do not push “explanation-first” requirements into the kernel; keep explanation as a user-space layer that *interprets* kernel artifacts.
- Do keep the kernel’s contracts stable, boring, and lawful: it should be easier to *recognize* correctness than to “tell a story”.

This keeps the boundary crisp:

- **Kernel** = lawful activity / constraint / compute (sublingual)
- **User space** = narration / judgment / justification (discursive)
