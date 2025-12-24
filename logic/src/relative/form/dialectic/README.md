# Dialectic - Support Library for Form Eval

## Philosophy of Science is Philosophy Enough!!!

**We're building a system that implements Philosophy of Science** - and dialectic is the **immanent process** that runs through everything, supporting Form eval in this scientific cognition.

## The Relationship

**Dialectic and evaluation are very similar things** - dialectic is a **support library for Form eval**.

The `dialectic/` folder provides utilities that:
- Materialize DialecticIR into commands for form engines
- Compose Concept/Judgment/Syllogism artifacts
- Bridge between dialectic state and form evaluation

## What Dialectic Does

### 1. Materialization (`materialize.ts`)
- Converts `DialecticIR` into commands for Shape/Context/Morph engines
- Maps dialectic states to form evaluation:
  - **Shape**: `dialectic.evaluate` (state → active Shape)
  - **Context**: `dialectic.invariant.check` (state → law/constraint checks)
  - **Morph**: `dialectic.state.transition` (state → transformation edges)

### 2. Essence Inferences (`essence.ts`)
- Runs three inference passes over DialecticIR:
  - Reflection (Shape)
  - Foundation (Context)
  - Ground (Morph)
- Calls materialization to produce commands

### 3. Concept Composition (`concept.ts`)
- Composes Concept artifacts from Morph + Judgment + Syllogism
- Provides the unity that form/eval works with

### 4. Active Ground (`active-ground.ts`)
- Seeds syllogism input from GDS Form Program
- Bridges kernel program to discursive syllogism

## The Flow

```
DialecticIR
  ↓ (materialize)
Commands (dialectic.evaluate, dialectic.invariant.check, dialectic.state.transition)
  ↓ (engines handle)
Shape/Context/Morph instances
  ↓ (form/eval)
Recursive descent into Shape:Context:Morph
```

## Why It's a Support Library

Dialectic provides:
- **Pre-processing**: Prepares dialectic state for form evaluation
- **Materialization**: Converts IR to executable commands
- **Composition**: Builds Concept/Judgment/Syllogism artifacts
- **Bridging**: Connects dialectic state to form engines

Form eval does:
- **Evaluation**: Recursively descends into Shape:Context:Morph
- **Principle reception**: Receives Principle from Pure Form
- **Determination**: Produces aspectual determinations

## The Speculative Bubble

The form/eval prompt was placed in dialectic, but dialectic reads like a support library for Form eval - **that would be correct**.

Dialectic prepares the material; form/eval evaluates it.

## Dialectic as Immanent

**Dialectic = Immanent** (runs through everything, internal)

Dialectic is the **internal process** that:
- Moves through Shape → Context → Morph
- Processes dialectical moments
- Transforms states through transitions
- Is **immanent** (within the form structure)

**Eval = Emanant** (external witness) - see `eval/EVAL-AS-WITNESS.md`

Dialectic runs through everything; Eval witnesses it from outside and extracts values for printing.

## Philosophy of Science in Action

**The system implements Philosophy of Science:**

- **Dialectic (Immanent)** = The process running through Shape → Context → Morph
- **Eval (Emanant)** = The witness extracting values for scientific cognition
- **Form Eval** = Recursive descent through Concept-Judgment-Syllogism (Determinate Science)
- **Philosophy of Science** = Principle Discovery → Reflection on Appearances → Determinate Science

**This is how we get computers to play the Philosophy of Science game.**

