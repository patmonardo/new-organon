# Cognition: Intuition and Concept

> "In whatever manner and by whatever means a mode of knowledge may relate to objects, **intuition** is that through which it is in immediate relation to them, and to which all thought as a means is directed. But intuition takes place only insofar as the object is given to us." — Kant, _CPR_ A19/B33

This document defines the architectural distinction between **Intuition** and **Concept**, which together form the genus of **Cognition** (Objective Representation).

## 1. The Definitions (Kantian Strictness)

In this repository, we strictly distinguish two species of **Cognition** (_Erkenntnis_):

1.  **Intuition** (_Anschauung_):

    - **Mode**: Immediate, Singular.
    - **Nature**: it is _given_. It relates directly to the object.
    - **In GDS**: The "Sublingual Kernel" (Rust). The act of knowing before speaking.

2.  **Concept** (_Begriff_):
    - **Mode**: Mediate, Universal.
    - **Nature**: it is _thought_. It relates to the object through features (marks).
    - **In GDS**: The "Discursive Logic" (TypeScript). The articulation of reasons.

> **Note**: We are discussing **Cognition** (Objective Representation), not Sensation (Subjective modification). The GDS Kernel produces _Objective_ structures, not mere feelings.

## 2. The Architectural Map: Kernel vs. Logic

We map this Dyad directly to the engineering layers:

| Layer          | Kantian Species           | Architecture Component    | Characteristic                                                               |
| :------------- | :------------------------ | :------------------------ | :--------------------------------------------------------------------------- |
| **GDS Kernel** | **Intuition** (Immediate) | Rust (`gds/src/`)         | **Sublingual**. Lawful execution. "I know _what_ to say" (before saying it). |
| **TS Logic**   | **Concept** (Mediate)     | TypeScript (`logic/src/`) | **Discursive**. Articulation. "I say _why_ it is so" (Judgment/Syllogism).   |

### The "Eval → Print" Moment

The boundary between these two is the transformation from Intuition to Concept.

- **Eval** (Kernel) is an _act_ of immediate knowing. It is "empty of thought" (no inner speech), but full of law.
- **Print** (Logic) is the rendering of that act into a transmissible, discursive artifact (Concept/Judgment).

## 3. The Real Mechanism: Concept-Judgment

Within the **Discursive (Concept)** layer, the "Real Mechanism" is the processing of Concepts and Judgments _before_ Inference.

### The Primacy of Concept-Judgment

Syllogism (Inference) is not a separate, third mechanism. It is **Categorical Judgment** augmented with a **Restored Concept**.

- _Example_: "All Humans are Musical" (Judgment) + Restored Concept (Human) + "Socrates is Human" = Syllogism.
- Therefore, the architecture must process **Shape** (Concept/Genus) and **Context** (Judgment/Predicate) _before_ deploying **Morph** (Syllogism).

### The Flow

1.  **Shape Evaluator** (Concept): Extracts Genus/Species.
2.  **Context Evaluator** (Judgment): Extracts Subject/Predicate.
3.  **Morph Evaluator** (Syllogism): Deploys the Logic of Existence/Reflection _using_ the results of 1 and 2.

## 4. Pre-Concept Connections: Feeding the Mechanism

Before we reach the Pure Concept (Hegelian Logic), we have the "Pre-Concept" material provided by the Kernel (Intuition).

### Two Pre-Concept Layers

1.  **Graph Algos / GNNs (Kernel/Intuition side)**:
    - Discover discrete patterns (PageRank, Embeddings).
    - This is _immediate_ discovery (Pre-Concept material).
2.  **Hegelian Syllogisms (Logic/Concept side)**:
    - Syllogisms of Existence and Reflection are _also_ "Pre-Concept" in the sense that they prepare the material for the **Pure Concept** (Self-moving Measure).

### The Connection Hooks

We connect the immediate findings of the Kernel to the discursive forms of Logic:

- **Graph Patterns** (e.g., Communities) → **Syllogism of Existence** (Figures S-P-U, etc.).
- **GNN Embeddings** (Latent Space) → **Syllogism of Reflection** (Allness, Induction, Analogy).

## 5. Summary: The Complete Cognitive Act

A full "Cognition" in GDS involves the movement from Intuition to Concept:

1.  **Intuition (Kernel)**: The system _acts_ / _sees_ a pattern (Sublingual, Immediate). "The Graph is thus."
2.  **Concept (Logic)**: The system _articulates_ the pattern (Discursive, Mediate). "This Node belongs to this Community _because_..."
    - First, as **Concept** (Shape).
    - Second, as **Judgment** (Context).
    - Third, as **Syllogism** (Morph).

This unity of Intuition and Concept constitutes the system's **Objective Representation** of Reality.
