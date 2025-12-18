# Task / Agent / Workflow — Engineering Overview

A compact overview of the Task–Agent–Workflow subsystem: typed schemas, runtime roles, and implementation guidance for building, testing, and integrating workflow engines.

## Purpose
Provide a type-safe, extensible framework for defining and executing computational Tasks, assigning them to Agents, and orchestrating them inside Workflows. Designed for production use (validation, observability, scalability) and for reuse across the Organon monorepo.

## Core Concepts
- Task — a unit of work with id, config, state, and result.  
- Agent — an active executor with capabilities, health, and assignment logic.  
- Workflow — an orchestrator of Tasks and Agents (DAGs or sequences), responsible for scheduling, monitoring, and recovery.

## Four-Fold Reality Box (working mapping)

This package is intentionally “meaning constrained” to the final synthesis (Agent) and its execution artifacts.

- **Controller = Model**: the control surface is an Action:Rule structure (what actions exist + when/how they apply).
- **Workflow = Task**: the live synthesis that carries the goal forward through actions.
- **View = Agent**: the dharmic display/active perspective that enacts the loop.

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
