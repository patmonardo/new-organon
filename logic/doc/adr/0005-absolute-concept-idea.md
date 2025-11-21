# ADR 0005 — AbsoluteConcept: Idea stage (analytical transformer)

Status: Proposed

Date: 2025-08-19

## Context

The `AbsoluteConcept` integrator currently converts phenomenal Aspects (ActiveRelation / ActiveProperty) into Judgments, evaluates per-aspect truth, maps Samyama bhumis, classifies aspects into realms (nature/spirit/logic), computes attention weights, and provides conservative teleology planning hints. The next logical pure transformation is to synthesize Judgments into higher-order analytical units we call `Idea`.

## Decision

Introduce a pure `Idea` transformer:

- `synthesizeIdeasFromJudgments(judgments[])` groups judgments by predicate (a pragmatic proxy for the asserted idea) and aggregates warrant confidences into a normalized `score`.
- Each `Idea` contains: id, summary, judgmentIds, score, provisional `truthHint` (likely|possible|contradicted|unknown), and minimal provenance.
- The Idea transformer is intentionally conservative and deterministic. It is a pure analytical layer sitting above Judgment-level reasoning and below full Knowledge/Jnana synthesis.
- `processAbsoluteConcept` may call the Idea transformer optionally behind a feature flag (config/flag `emitIdeaStage`) to preserve backward compatibility.

## Rationale

- Separating Idea from Judgment preserves the philosophical division between propositional judgments and synthesized ideas (aggregated meaning/intent).
- Keeping the transformer pure and conservative enables easy testing and auditing.
- Opt-in emission prevents breaking existing integrations and keeps the base integrator minimal for consumers that only need judgments.

## Consequences

- New outputs may be added to `AbsoluteConceptOutputs` when enabled: `ideas: Idea[]`.
- A future ADR will propose a Knowledge (Jnana) transformer that promotes Ideas to Knowledge using stronger synthesis rules and richer provenance.

