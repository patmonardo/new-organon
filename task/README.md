# Task / Agent / Workflow — Engineering Overview

A compact overview of the Task–Agent–Workflow subsystem: typed schemas, runtime roles, and implementation guidance for building, testing, and integrating workflow engines.

## Purpose
Provide a type-safe, extensible framework for defining and executing computational Tasks, assigning them to Agents, and orchestrating them inside Workflows. Designed for production use (validation, observability, scalability) and for reuse across the Organon monorepo.

Organon Task is the **Kriya Jnana (Science of Action)** layer: knowledge that is made objective through execution (traces, artifacts, results) and then re-internalized into the next determination.

## Core Concepts
- Task — a unit of work with id, config, state, and result.  
- Agent — an active executor with capabilities, health, and assignment logic.  
- Workflow — an orchestrator of Tasks and Agents (DAGs or sequences), responsible for scheduling, monitoring, and recovery.

## Working Mapping (triad)

This package uses a compact mapping that keeps “final synthesis” precise:

- **Task = Being**: determinate work to be done.
- **Agent = Essence**: the mediated inner runtime (capability + perspective).
- **Workflow = Concept**: the composed unity that actually orchestrates execution.

In practical TS terms, you can treat the process as **Active Logic**:

- **Concept Activation → Controller**: Concept becomes effective only by entering a controller surface (decision + dispatch).
- **Controller + Concept Activation → Workflow**: together they form the unit that orchestrates Tasks through Agents.

In this sense, a **Workflow is synthesized** (it composes Tasks through Agents), while an **Agent’s view is received** (it registers constraints, context, and observations).

This keeps Organon Task embeddable: if you need a server framework, embed the agent runtime into an external host-adapter package.

## Schema files
- `src/schema/task.ts` — Task Zod schema (execution, state, config)  
- `src/schema/agent.ts` — Agent Zod schema (capabilities, health, assignment)  
- `src/schema/workflow.ts` — Workflow Zod schema (steps, orchestration)  
- `src/schema/definition.ts` — conceptual foundation (reference)  
- `src/schema/index.ts` — consolidated exports

## Runtime
- `src/agent/` contains the framework-agnostic agent runtime surface (RootAgent loop + absorption).

Organon Task’s agent runtime is designed to be **embeddable**: treat it like a component you can host inside another system.

Note: Organon Task intentionally does **not** implement external agent-stack integrations (MCP/GenKit/etc). If you need a NestJS (or similar) infrastructure, the agent embeds into an external host-adapter package (e.g. a NestJS module) rather than Task depending on that framework.

## Design Principles
- Schema-first: small, precise, validated artifacts.  
- Type-safe: derive types from Zod schemas for runtime + compile-time guarantees.  
- Extensible: plugin/adaptor friendly (executors, persistence, schedulers).  
- Observable: metrics, logs, and traces for operational visibility.  
- Scalable & resilient: distributed execution and task recovery patterns.

## Implementation Outline
1. Repository layer: persistence adapters (databases, queues).  
2. Executor layer: concrete runtime implementations for Agents.  
3. Agent runtime: RootAgent loop/absorption surfaces.

## Development
- Build: `pnpm --filter @organon/task build`  
- Test: `pnpm --filter @organon/task test`  
- Generate docs (if configured): `pnpm --filter @organon/task run doc:api`

## Notes
- Prefer package-scoped imports (e.g. `@organon/task`) across the monorepo.  
- Keep schemas minimal and well-documented to support API-first development.  
- Use the `src/schema` index for barrel exports to simplify imports.

## Conceptual context
This module expresses a practical dialectic of work, actor, and orchestration: Tasks as determinate work, Agents as active executors, and Workflows as the organizing frame that actualizes possibilities
