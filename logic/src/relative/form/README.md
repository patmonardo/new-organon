# Relative Form (Reflection Engine)

`relative/form` is the **Reflection Engine**.

## Canonical mapping

- **Reflection** is the container-side triad of the form engine: **Shape / Context / Morph**.
- **Shape** is the moment of **Active Consciousness**.
- **Context** is the moment of **Active Determination-of-Reflection** (a determination _within_ Reflection; not “Reflection itself”).
- **Morph** is the moment of **Active Ground**.
  - “Morph” here means _grounding transformation / grounding movement_.
  - It is **not** intended to imply a mathematical morphism.

## Kernel boundary (Pure Form presupposed)

Our system **presupposes Absolute Form**.
We do not implement it here; its Principle as **Pure Form** lives in the GDS kernel.

Working boundary vocabulary:

- **Pure Form** is the **Application Form** the GDS kernel receives.
- **Absolute Form** manifests for us as **ProjectionFactory:EvalForm** (i.e. a projection/evaluation form that can be submitted).
- The kernel manipulates **FormShape** into a **Truth step** (a validated/grounded transformation).

`Eval` shows up twice because it is the **true middle** in two directions:

- **Read → Eval → Form** (kernel determination / TruthStep)
- **Form → Eval → Print** (return rendering / discursive transmission)

On the Relative side:

- RelativeForm “magically receives” its container **Shape as Morph**.
- **Morph** is exactly **Reflection** as Principle / Pure FormShape of the **Reflected**.

This keeps TS `relative/form` honest: it is Reflection’s active surface, while Pure/Absolute Form is kernel-owned and presupposed.

## Scientific return (RDF / graph form)

Kernel-side, **Form returns what is Scientific** in our **RDF dataset**.

“RDF” here is best read as the **nearest interoperable principle** (a public graph substrate).
Our target is **beyond RDF** in the sense that we treat graph form as a vehicle for scientific determination, not merely a web-standard serialization.

- Everything is a **FormShape**.
- We represent our **triadic datasets** as **graphs** (a tripartite dataset articulated as a graph structure).
- **Absolute Form** (Procedure/ML/Form in GDS) therefore operates by representing and transforming these triadic datasets **as graphs**.

RelativeForm does not decide what counts as “Scientific”; it produces determinate reflective acts (judgments, traces, projections) that the kernel can accept/reject/transform into scientific form.

## Naming note: Repo ≡ Representation Record

- `*Repo` means the **representation record**: the schema-validated live shape that engines, nexus classes, and repos exchange. It is intentionally named “Repo” (not “Type”) to emphasize it is the persistence- and transport-ready record, not a runtime behavior container.
- “Record” and “Repo” are synonymous here; “Repo” avoids conflating with runtime state in the engine.
- Nexus classes (`FormShape`, `FormEntity`, etc.) wrap the `*Repo` record, add behavior/mutators, and expose `fromRecord`/`toRecord` to bridge schema ↔ runtime ↔ persistence.

## Method: nearest principle

Methodologically, we locate a Principle **next to the nearest Principle**.
Fichte’s move is canonical: he places the **Science of Knowing** next to **Kant** as a further species of **Transcendental Philosophy**.
We use the same discipline here: treat existing substrates (e.g. RDF) as the nearest principle for articulation, while specifying the stricter Principle our kernel actually enacts.

## Appearance (contained side)

“Appearance” is the **contained** side of the Form Processor.

- **Entity / Property / Aspect** correspond (for us) to **Thing / Property / Relation**.
- **Aspect** presupposes a thing-in-the-world (relation-as-aspect, not a free-standing abstraction).
- **Mathematics / Quantity** lives here as **contained** (it is not the container-side law).
  - In Kantian terms: an **entity in space and time** presupposes **mathematical Space and Time** (forms of intuition).
  - This is where quantitative limiting/measure shows up.

## Reflection/Appearance dyad

The **Reflection ↔ Appearance** dyad is also our **Reflecting ↔ Conceiving** dyad.
Its telos is **Knowing** (`meta.dyadTelos = 'knowing'`).

## Restriction (no Absolute Logic here)

We treat **Concept → Judgment → Syllogism** as the (qualitative) ladder toward **Absolute Logic**.
This engine is **restricted** to the **Reflection ↔ Appearance** domain:

- It can express **Concept → Judgment** (qualitative determination).
- It does **not** traffic in **Syllogism / Absolute Logic**.

“Science” (in the strong sense) is therefore **kernel-side**.
Still, it is also **Logic**: the kernel’s Absolute/Scientific inference is the continuation (not a separate subject-matter).

From a Kantian perspective, you can view kernel **Inference (AbsoluteForm)** as producing a **Judgment from a Judgment** (judgment-to-judgment necessity).
Kant’s principles are simpler than Hegel’s, but are inadequate for our task; we keep the boundary explicit so Hegelian requirements can be satisfied without pretending the Relative engine already contains Absolute Logic.

## Pre-Science (graph + ML moments)

**Reflection is Pre-Science**.
That makes the relationship between RelativeForm and graph/ML algorithms immediate:

- Graph algorithms and ML algorithms are **moments of Pre-Science**: they produce measures, rankings, embeddings, clusters, anomaly signals, etc.
- These results are not “Scientific” by themselves; they are **candidate determinations** that still require grounding/validation.

Operationally, they sit on the Relative side as **Procedure → Pipeline**:

- **Procedure**: runnable step(s) (graph/ML computations) that generate candidate structure over FormShape/graph.
- **Pipeline**: sequencing/conditioning of procedures into an experimental determination.
- **Form (kernel)**: the act that turns those candidates into a **TruthStep** (scientific relation), or rejects them.

You can read this as an **emergence** problem:

- Pre-Science yields artifacts (scores, embeddings, clusters, traces).
- The kernel promotes (or refuses to promote) these into **TruthSteps** (scientific relations).

In code terms, this is the intended boundary vocabulary in `@organon/logic`:

- `PreScienceArtifact` (Relative)
- `PromotionRequest` → `PromotionResult` (boundary)
- `TruthStep` (kernel)

RelativeForm’s role is therefore to:

- dispatch procedures/pipelines,
- collect traces/artifacts,
- render outcomes discursively as **Aspects** (what can be narrated/compared),
- and submit projections/eval-forms to the kernel for scientific closure.

## Form theory (Active Form as graph)

Even though Reflection is Pre-Science, the **Science axis is present in Form**.
That is the point of the terminology:

- A **Graph** represents a **Form in execution** — an **Active Form**.
- RelativeForm is therefore a **Form Theory** layer: it treats forms operationally (as executable, traceable graph-shapes) before they are closed as kernel-side scientific TruthSteps.

## Knowing (non-discursive)

**Knowing is not discursive**.

- The **Syllogism** should not be treated as “Knowing proper”; it is still primarily a form of **Judging**.
- Discursive judgment (the kind born from explicit assertions/premises) is the product of **Abstract Reason**.
- **Pure Reason / Form** (in the kernel sense) is **not discursive**: it is the scientific act/result, not merely an argued conclusion.

Practically, the non-discursive site is where kernel work closes:

- **Procedure → Pipeline → Form**, where **Form** here is strictly **Scientific** (kernel-produced Truth step over FormShape/graph).

When that kernel Truth-step is transmitted back into RelativeForm, it becomes **discursive** (expressible as judgments, traces, explanations).
But discursive narrative can only ever touch **Aspects**:

- RelativeForm receives kernel Relations/TruthSteps as **aspectual renderings** (what can be said, compared, scheduled, revised).
- The kernel FormProcessor deals in **Relations**; RelativeForm tells discursive stories about **Aspects of Relations**.

We do not need RelativeForm to “reach Absolute Knowing”.
For this layer, **Reflection as the power of speech** is sufficient: the capacity to articulate, transmit, and revise aspectual determinations.

## Relative enclosure (container ↔ contained)

In Relative Logic, `Eval` stands **in the middle between the container and the contained**.
That is why the return path takes the form of a **discursive print** of kernel evaluation:

- it is evaluation **within an enclosure** (Reflection/Appearance mediation)
- it can only touch **Aspects**

Absolute Form is not enclosed in this way; it is the kernel-side act of determination itself.

## Sensible science (e.g. abstract algebra)

This architecture still has a role even in “sensible” sciences (e.g. Abstract Algebra).
The point is not to mystify mathematics, but to keep the promotion boundary explicit:

- pre-scientific procedure/pipeline work still exists (construction, computation, exploration),
- scientific closure still appears as relations/truth-steps,
- and discursive print still returns to RelativeForm as aspectual narration.

In Kant’s terms: the Universal Pure Reason branch of our knowledge (Concept) is **not empty**.

## FactStore vs KnowledgeStore

We distinguish two stores (even if they converge in the long run):

- **FactStore (Reflection-side)**: the store of _facts_ in the Kant–Hegel sense — discursive, aspectual determinations (what can be said, traced, revised). This is the natural persistence target for RelativeForm engines.
- **KnowledgeStore (Science/Universal-side)**: the store of _promoted_ determinations — relations/truth-steps that are worthy of inclusion in a Universal “encyclopedia” surface.

Narratively, “KnowledgeStore” is often the better name than “Encyclopedia” (less grandiose, more operational), but they point at the same destination: a universal body of promoted knowledge.

## Runtime trace

Dialectic→Form materialization tags emitted commands with:

- `meta.container = 'reflection'`
- `meta.moment = 'shape' | 'context' | 'morph'`
- `meta.semanticRole = 'active-consciousness' | 'determination-of-reflection' | 'active-ground'`

This meta is the intended handle for contextual tracing ("chain of inference") without forcing a specific compilation target.

## Aspect vs Relation (processor focus)

- Our **engine** shows up as **Aspect** (a situated slice of the Reflected).
- The kernel **FormProcessor** deals in **Relations** (essential connections), and sees Aspects as moments **of** Relations.

This is the practical consequence of “Essential Relation is the Truth of the Reflected”: kernel outputs are relational/essential, while RelativeForm runtime operates aspectually (reflective, situated, trace-first).
