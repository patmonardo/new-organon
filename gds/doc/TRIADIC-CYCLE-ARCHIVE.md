# Triadic Cycle (Archive)

This note preserves earlier, more speculative framing that used “triadic” language to describe the evaluator stack.

It is intentionally **not** embedded in runtime code/docs anymore; the current codebase keeps this kind of framing out of the hot path and keeps only minimal semantic mappings in `projection/eval/form/triadic_cycle.rs`.

## Historical framing (verbatim-ish)

- Procedure (Assertion / Universal)
- ML Pipeline (Problematic / Particular)
- Form Processor (Apodictic / Singular)

The idea was: Form consumes artifacts from earlier stages and returns a single derived graph (ResultStore).

## What remains in code

Only a small, explicit mapping remains:

- `ModalMoment::{Assertion, Problematic, Apodictic}`
- `ConceptMoment::{Universal, Particular, Singular}`

This is used as vocabulary (and is unit-tested), not as a claim about system architecture.
