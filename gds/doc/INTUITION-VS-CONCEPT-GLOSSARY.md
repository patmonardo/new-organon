# Intuition vs Concept (and why ML “intuition” is usually informal)

This repo uses **Kant–Fichte–Hegel** as an architectural discipline, so we need to be precise about words that modern technical culture uses loosely.

## 1) Kantian distinction (the one we care about)

- **Intuition** (Anschauung): immediate givenness (the “given” side of cognition).
- **Concept** (Begriff): discursive form (the “thinkable / judgeable” side).

So:

- If someone is transmitting a **discursive statement** (“If P then Q”, “Here is a proof step”, “Here is the rule”), that transmission is **not an intuition** in the strict Kantian sense.
- It is Concept/Judgment material (discursive).

## 2) Why ML researchers say “intuitive” anyway

In ML/CS practice, “intuitive” is typically shorthand for:

- **informal**
- **by example**
- **heuristic mental model**
- **geometric picture / analogy**
- **operational explanation**

This is often pedagogically excellent, but it is not a Kantian claim about “intuition.”

## 3) Recommended wording in our docs

To avoid category errors, prefer:

- “**Informally** …”
- “**By example** …”
- “**Operationally** …”
- “**Geometrically (as a picture)** …”
- “**As a heuristic** …”

Reserve “intuition” for:

- actual Kantian discussions, or
- a clearly defined, explicit sense (e.g., “empirical intuition of measurement,” “pure intuition of space/time”).

## 4) Architectural consequence

This is not nitpicking: it affects how we store the Reason column.

- **Concept/Judgment** belongs in transmissible artifacts (schemas, traces, WorkflowRuns).
- **Kernel knowing** may be sublingual, but once it is made transmissible, it appears as **discursive reasons** (statement–reason form).

So: our “logogenesis encyclopedia” should track *discursive reasons* precisely, and treat “intuition” as a separate genus.


