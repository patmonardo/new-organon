# Task / Agent / Workflow — Engineering Overview

A compact overview of the Task–Agent–Workflow subsystem: typed schemas, runtime roles, and implementation guidance for building, testing, and integrating workflow engines.

## Purpose
Provide a type-safe, extensible framework for defining and executing computational Tasks, assigning them to Agents, and orchestrating them inside Workflows. Designed for production use (validation, observability, scalability) and for reuse across the Organon monorepo.

## Core Concepts
- Task — a unit of work with id, config, state, and result.  
- Agent — an active executor with capabilities, health, and assignment logic.  
- Workflow — an orchestrator of Tasks and Agents (DAGs or sequences), responsible for scheduling, monitoring, and recovery.

## Schema files
- `src/schema/task.ts` — Task Zod schema (execution, state, config)  
- `src/schema/agent.ts` — Agent Zod schema (capabilities, health, assignment)  
- `src/schema/workflow.ts` — Workflow Zod schema (steps, orchestration)  
- `src/schema/definition.ts` — conceptual foundation (reference)  
- `src/schema/index.ts` — consolidated exports

## Relative runtime platform (disabled)
- `src/relative/` contains an early “relative” runtime platform (historical / Nest-flavored).
- It is currently **disabled** (excluded from the active SDK build) so we can rebuild TAW step-by-step without committing to a framework runtime.

## Design Principles
- API-first: clear schemas suitable for REST/GraphQL endpoints.  
- Type-safe: derive types from Zod schemas for runtime + compile-time guarantees.  
- Extensible: plugin/adaptor friendly (executors, persistence, schedulers).  
- Observable: metrics, logs, and traces for operational visibility.  
- Scalable & resilient: distributed execution and task recovery patterns.

## Implementation Outline
1. Service layer: business logic for creating, scheduling, and reconciling Tasks.  
2. Controller layer: REST/GraphQL endpoints for clients and operators.  
3. Repository layer: persistence adapters (databases, queues).  
4. Executor layer: concrete runtime implementations for Agents.  
5. Integration adapters: connectors for external tooling and infra.

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
