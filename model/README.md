# Model Package (`@/model`): System Instruction & Architecture

## Purpose

The `@/model` package implements the Model-View-Controller (MVC) pattern as executable, empirical sciences. Each MVC is a Knowledge Graph, representing an applied science or domain of inference and action.

## Architectural Role

- **Driven by `@/task`**: All model execution (inference, action) is orchestrated by the Task/Agent/Workflow system in `@/task`, using Genkit and NestJS.
- **Constructed by `@/logic`**: Model construction, schema generation, and EPR (Entity-Property-Relation) logic are provided by the Hegelian engine in `@/logic`.
- **Integrated with `@/core`**: Agents in `@/task` may utilize local LLMs or LLM caches in `@/core` for advanced reasoning, caching, or native language model execution.

## Key Principles

- Each Model is an executable Knowledge Graph (MVC).
- Models are not static; they are dynamically constructed and orchestrated.
- Inference is treated as a type of action—model execution is always active.
- The system is designed for extensibility, composability, and orchestration by higher-level controllers.
- Models will evolve to use NestJS Decorators for seamless orchestration by `@/task`.

## Philosophical Foundation

- **Dialectical Movement**: The architecture embodies the dialectic from Pure Forms (principle, a priori analytic, defined in `@/logic`) to Given Forms (data, a posteriori, realized in `@/model`), and returns through practice (`@/task`) to renewed principle.
- **Qualitative vs. Quantitative Logic**: The system distinguishes between qualitative reasoning (essence, meaning, pure discrimination) and quantitative reasoning (measurement, external relation). True knowledge arises from the synthesis and mediation of both.
- **Triadic Structure**: While the architecture is sequential (`@logic` → `@model` → `@task`), it supports triadic reasoning—principle, mediation, and practice—essential for supersensible scientific language and dialectical systems.

## System Flow

1. **Model Construction**: `@/logic` generates EPR schemas and Knowledge Graphs.
2. **Model Execution**: `@/task` orchestrates the execution of Models (MVCs) as part of workflows.
3. **Agent Integration**: Agents may leverage `@/core` for LLM-based reasoning or caching.

## File Structure

- `model/` — Model definitions and schemas
- `view/` — View logic and presentation
- `controller/` — Controller logic for model orchestration
- `form/` — Input/output schemas and validation
- `schema/` — Model schema definitions
- `index.ts` — Main exports

## Evolution Roadmap

- Refactor models to use NestJS Decorators for better integration and control by `@/task`.
- Enhance composability and orchestration of models as Knowledge Graphs.
- Expand schema and controller logic for richer, more dynamic model execution.

---

This README serves as the system instruction and architectural guide for the `@/model` package. Update as the package evolves.

# ORGANON Model, View, Controller, Task, Agent, Workflow: Dialectical Root Schema

## Overview
This package encodes the root dialectical architecture of ORGANON, capturing the generative movement from pure logic (the Form Engine) into living, agential process. The schemas here are not mere data structures—they are the abstract, recursive mediators that energize and control the logic engine, enabling both principled structure and dynamic process.

## Dialectical Structure
The system is organized as a 3×3×3 (3³) generative cube, with each layer decomposed into triadic dyads:

- **BEC (Being–Essence–Concept)**: The logical ground (Form, Entity, Context, Property, Morphism, Relation)
- **MVC (Model–View–Controller)**: The operationalization of logic (State, Structure, Representation, Perspective, Action, Rule)
- **TAW (Task–Agent–Workflow)**: The synthesis into process (Goal, Operation, Identity, Capability, Sequence, Transition)

Each component is defined as a synthesis of two poles (a dyad), forming a living unity:

- **Model = State : Structure**
- **View = Representation : Perspective**
- **Controller = Action : Rule**
- **Task = Goal : Operation**
- **Agent = Identity : Capability**
- **Workflow = Sequence : Transition**

## Dialectical Movement: Wheel Within a Wheel
The architecture encodes a two-fold movement:

1. **Absolute Dialectic**: The recursive descent from universal principle (logic) into concrete, agential process. Each schema is a moment in this descent, mediating between abstraction and actualization.
2. **Appearance/Transaction**: The return movement, where process and agency feed back into the logic engine, generating new forms, rules, and possibilities. This is the dynamic, event-driven flow that makes the system living and generative.

## Inferences and Relations
- **Each dyad is a control/flow network**: It mediates between two poles, enabling both analytic clarity (distinction) and generative synthesis (unity).
- **The system is both principled and dynamic**: The logic engine provides the ground; the schemas here energize and direct it.
- **Recursive traversal**: Each schema supports further decomposition, enabling deep, flexible modeling and process orchestration.
- **Event-driven, OOFP design**: The architecture is not static OOP, but a functional, event-driven network—"a wheel within a wheel"—where every component can mediate, react, and synthesize.

## Purpose
This root schema is not an application framework, but the abstract control/flow network that wraps and energizes the Form Engine. It is the minimal, generative core needed to build living, dialectical systems in ORGANON.

---

> "The system must be both principled and dynamic: grounded in Being, but always returning through mediation and process to new beginnings." (Hegel, paraphrased)

---

For further details, see the philosophical notes in each schema file and the main ORGANON documentation.
