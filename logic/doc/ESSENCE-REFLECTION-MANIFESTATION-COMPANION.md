# Companion: ESSENCE → REFLECTION → MANIFESTATION (Operational Guide)

This companion document is meant to sit beside `logic/ESSENCE-REFLECTION-MANIFESTATION.md`.

It does **not** try to “improve” the artifact. Instead, it makes the artifact **actionable**:
- as a reading guide,
- as an implementation guide (how to map ideas to folders/types/results), and
- as a prompt-craft guide for using Copilot as a *generative partner* in this codebase.

## 0) About Copilot instructions (repo behavior)

This repo includes `.github/copilot-instructions.md`. In Copilot Chat/Agent contexts, those instructions are typically loaded automatically by the host environment.

Practical rule:
- Write prompts as if the agent *already knows* the monorepo conventions (ESM, schema-first, workspace imports, etc.).
- If you open a new chat outside the workspace context, paste the key conventions manually.

## 1) The artifact in one page

The artifact asserts:

- The **Form Engine** is “Active Essence”: it processes **Being → Essence → Concept**.
- There are two parallel pipelines:

**Container Pipeline (Principle)**

```
Shape (Reflection) → Context (Determination of Reflection) → Morph (Ground)
```

**Contained Pipeline (Principled-Effect)**

```
Entity (Appearance) → Property (Laws of Appearance) → Aspect (Essential Relation)
```

And a key operational claim:

- **Identity → Difference → Contradiction** runs *inside* the Context engine.
- A **contradiction-free Context** gives birth to a **Law of Appearance** (Property), i.e. “science.”
- **Context is sublated within Morph**, not within Aspect.

## 2) Glossary: philosophical term → engine output

Use this as a translation table when writing code, schemas, or tests.

- **Reflection** → `Shape` (FormShape): “result-sets of pure reflection”
- **Appearance** → `Entity`: appearance of a shape, with `formId` + facets/signature
- **Determination of Reflection** → `Context`: determinations, constraints, scope, presuppositions
- **Laws of Appearance** → `Property`: discursive laws, automata/invariants; property engine aggregates result sets
- **Ground:Condition:Facticity** → `Morph`: container unity of shape+context, mechanism of change
- **Essential Relation** → `Aspect`: relations that express essences-in-existence (spectral/relational view)

## 2.5) Thinghood × Action → Entity (factorization for codegen)

Your framing:

- **Thinghood** (Hegel: “The Thing and its Properties”) gives the logical progression of *concrete existence becoming thingness*.
- **Action** (activation) turns thingness into a **process**.
- The crossing yields **Entity** as the activation of concrete existence into Thingness:
  - **Noumenal Process**: “thingness as inward process” (what-it-is in its ownness).
  - Adding **Action** produces a **Phenomenal Process**: “thingness as outwardly expressible operation” (what-it-does / how-it-appears).

This repo already has anchors for that split:

- Thinghood IR as dialectical progression (Appearance): [logic/src/relative/essence/appearance/thing/thing-ir.ts](logic/src/relative/essence/appearance/thing/thing-ir.ts)
- Thing principle schema/doc: [logic/src/schema/thing.ts](logic/src/schema/thing.ts)
- Entity schema (existence/facticity envelope + facets/signature hooks): [logic/src/schema/entity.ts](logic/src/schema/entity.ts)
- Entity engine (activation boundary + events/commands + evaluation): [logic/src/relative/form/entity/entity-engine.ts](logic/src/relative/form/entity/entity-engine.ts)
- Entity repository (FormDB persistence boundary): [logic/src/repository/entity.ts](logic/src/repository/entity.ts)

Companion rule for codegen:
- Treat **Thinghood** as the dialectical source (IR + schema) and treat **Kriya** as the activation interface (commands/evaluate/events) that yields Entities.

If you find yourself arguing about a name: defer to what the *engine produces as a result set*.

## 3) Result-sets as the contract

The artifact repeatedly anchors itself in **ResultSets**.

Treat ResultSets as the stable “contract surface” between:
- **philosophical narrative** (why)
- **schema** (what)
- **implementation** (how)
- **tests/validation** (is it coherent?)

A concrete list appears in the artifact for Context:
- `presuppositions`
- `scope`
- `conditions`
- `entities`
- `relations`

Companion discipline:
- Whenever you add a new engine behavior, name the **ResultSet** first.
- Only then introduce types, IR transforms, or folder placement.

## 3.5) Reflection Protocol (Pre-Concept scaffold) — the key insight

This repo is intentionally operating in a **Pre-Concept scaffold**.

Meaning:
- We are not doing “Kantian rules to avoid error” (merely regulative).
- We are building a **constructive/generative protocol** where determinacy and law emerge immanently from reflection (Essence).

### What Reflection produces (Essence)

Reflection is implemented as a protocol whose outputs are **ResultSets**.

- **Context** is the ResultSet of the reflective movement (Identity/Difference/Contradiction operating *within* the Context engine).
- **Property** is a law-of-appearance derived from contradiction-free Context (still Essence; not yet Concept).
- The unresolved remainder “falls to Ground” and is carried forward via **Morph** (Context sublated into Morph, not Aspect).

### Inputs → Outputs

- **Input**: principles / Being (what is posited as given)
- **Output A (Context ResultSets)**: `presuppositions`, `scope`, `conditions`, `entities`, `relations`
- **Output B (Property ResultSets)**: invariants/automata describing stable lawful appearance under a Context
- **Output C (Ground remainder)**: what cannot remain as coherent determination (handed to Morph)

### Invariants vs forbidden states (terminology)

- **Invariant (program correctness sense)**: a property that must *always* hold at the points it claims to hold.
- **Forbidden/Invalid state**: a state that must *never* be reachable (useful for tests and contradiction definitions).

### Non-negotiable boundary (protocol stack rule)

- No Particular-model fields belong in the Universal layer.
  - FormDB stores linkage/handles + dialectical metadata (facets/signature), not empirical records.
  - Empirical values live in Model/Prisma/Postgres.

### What we explicitly defer (Concept)

We reserve **Judgment of the Concept** (assertoric/problematic/apodictic as Concept-truth) for the future Concept layer.

In the Pre-Concept scaffold, we do:
- determination, compatibility, contradiction detection, grounding, derivation of laws.

We do *not* do:
- Concept-judgment classification as truth (universality/particularity/singularity judgments), except as a named future hook.

## 3.6) Essence Megha (Absolute) — the Essence → Concept transition hook

Insight to preserve:
- The transition we ultimately seek is the movement from **Reflection as Pre-Conceptual process** (Sphere of Essence; necessity) into the **Sphere of the Concept** (freedom).
- You name this transitional “weather system” **Essence Megha** (the Absolute in logic terms): the condition under which reflective necessity becomes the ground of free Conceptual activity.

Engineering implication (keep it honest):
- In this repo, “Essence Megha” should be treated as the *explicit boundary event/handoff* between layers, not as something we silently mix into Essence.
- Until the Concept layer exists, we only leave:
  - a named hook (documentation + types/entrypoints if needed), and
  - clear preconditions (e.g., contradiction-free Context → lawful Property; remainder → Morph).

This prevents category errors: Essence stays constructive and generative, while Concept remains a deliberate next phase rather than an accidental leak.

## 4) The “Context” focus: where Identity/Difference/Contradiction lives

The artifact is explicit: Identity/Difference/Contradiction operates **within** Context.

Operationally, this means:
- Don’t implement Identity/Difference/Contradiction as standalone “world transforms.”
- Implement them as **internal passes** that produce Context result sets.

A useful mental model:

```
Being (principles)
  ↓
Reflection passes (Identity → Difference → Contradiction)
  ↓
Context = determinations of reflection (result sets)
```

### How to use this when designing types

- If a type is about “what counts as valid/realizable,” it belongs with Context.
- If a type is about “what exists as appearance,” it belongs with Entity/Property.
- If a type is about “what changes, and how,” it belongs with Morph.

## 5) The “sublation rule” (most common implementation trap)

The artifact’s sharp constraint:

- Context is sublated within **Morph** (Ground), **not** Aspect.

If you see code where Aspect must “know” Context deeply to function:
- that’s a design smell,
- you are probably mixing Container and Contained pipelines.

Corrective heuristic:
- If Aspect needs a Context fact, Morph should have already carried it forward (as Ground).

## 6) Copilot workflows: prompts that respect the artifact

These are *prompt patterns* you can use while editing code.

### A) “ResultSet-first” prompt

Use when adding a feature.

Prompt:
> Add/modify engine X. Start by proposing the ResultSet contract (name, type shape, invariants), then show the minimal TS types and the function skeletons. Do not add extra UX or unrelated files.

### B) “Pipeline separation” prompt

Use when refactoring.

Prompt:
> Refactor to enforce the two pipelines: Container (Shape→Context→Morph) and Contained (Entity→Property→Aspect). Identify any leakage where Contained depends on Context details, and move that dependency into Morph.

### C) “Contradiction-free Context” prompt

Use when implementing validation.

Prompt:
> Implement a check that Context is contradiction-free. Define what contradiction means in this module (operationally), and produce failing test cases demonstrating contradiction → Ground transition.

### D) “Mapping-to-folders” prompt

Use when you’re unsure where code should live.

Prompt:
> Given this new concept/type, place it under the existing `logic/src/` folder structure consistent with the artifact (reflection vs appearance; context vs property; morph vs aspect). Propose the smallest change.

## 7) A companion “reading protocol” (how to keep it generative)

When revisiting the artifact, read it in three passes:

1) **Names pass**: list engine nouns (Shape/Context/Morph/Entity/Property/Aspect).
2) **Contract pass**: for each noun, list its ResultSets.
3) **Invariants pass**: for each ResultSet, state what must *never* be true.

Then translate pass (2) into:
- schemas,
- validation,
- tests.

## 8) Questions worth answering next (choose 1–2)

These are the highest-leverage “make it real” questions suggested by the artifact:

- What is the minimal formal definition of “contradiction” for Context in this repo?
- What is the minimal representation of a “Law of Appearance” as an automaton (Property)?
- What are the stable IDs/keys that connect `Shape → Entity` and `Property → Aspect`?
- What exactly is carried forward into Morph as Ground (shape+context), and what is left behind?

## 8.5) Pillars of Form → TypeScript module boundaries (the codegen target)

You said: “each of the Pillars of Form must be crafted in that factorization of TS modules.”

This section turns the six pillars into a concrete module-boundary map that codegen can reliably target.

Pillars (from the artifact):
- Shape (Reflection)
- Context (Determination of Reflection)
- Morph (Ground)
- Entity (Appearance)
- Property (Laws of Appearance)
- Aspect (Essential Relation)

Boundary rules (small, enforceable):
- **IR lives with the pillar** (dialectic source-of-truth): keep IR close to the philosophical section it encodes.
- **Schemas live under** `logic/src/schema/` and are the stable ResultSet contracts.
- **Engines live under** `logic/src/relative/` and implement command/evaluate/process semantics.
- **Repositories live under** `logic/src/repository/` and must honor persistence constraints (e.g., FormDB may store linkage but not empirical values).

Concrete placement map (what codegen should emit / extend):

- **Shape pillar**
  - IR: `logic/src/relative/essence/reflection/...` (keep “pure reflection” here)
  - Schema: `logic/src/schema/shape.ts` (or the existing equivalent if already present)
  - Engine: `logic/src/relative/form/shape/shape-engine.ts`

- **Context pillar**
  - IR passes: `logic/src/relative/essence/reflection/foundation/...` (Identity/Difference/Contradiction)
  - Schema: `logic/src/schema/context.ts` capturing `presuppositions/scope/conditions/entities/relations`
  - Engine: `logic/src/relative/form/context/context-engine.ts` (runs the passes; outputs Context ResultSets)

- **Morph pillar**
  - Schema: `logic/src/schema/morph.ts` capturing Ground/Condition/Facticity and the carried-forward (sublated) Context
  - Engine: `logic/src/relative/form/morph/morph-engine.ts` (the sole place Context becomes Ground)

- **Entity pillar**
  - IR: [logic/src/relative/essence/appearance/thing/thing-ir.ts](logic/src/relative/essence/appearance/thing/thing-ir.ts)
  - Schema: [logic/src/schema/entity.ts](logic/src/schema/entity.ts)
  - Engine: [logic/src/relative/form/entity/entity-engine.ts](logic/src/relative/form/entity/entity-engine.ts)
  - Repo: [logic/src/repository/entity.ts](logic/src/repository/entity.ts)

- **Property pillar**
  - Schema: `logic/src/schema/property.ts` capturing “laws of appearance” (automata/invariants + ResultSets)
  - Engine: `logic/src/relative/form/property/property-engine.ts` (aggregates ResultSets from laws)

- **Aspect pillar**
  - Schema: `logic/src/schema/aspect.ts` capturing essential relations produced from Property ResultSets
  - Engine: `logic/src/relative/form/aspect/aspect-engine.ts`

The design intent is simple: codegen can generate one pillar at a time, and each pillar has an obvious home for (1) IR, (2) schema, (3) engine, and optionally (4) persistence.

## 9) If you want, I can draft a v2

If you tell me:
- the intended audience (you-only vs team), and
- whether you want it to reference concrete files/types in `logic/src/`,

…I can produce a second version that hyperlinks into the actual engines/schemas in this repo and proposes a minimal set of “next implementation steps” aligned with existing code.
