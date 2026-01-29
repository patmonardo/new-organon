# ADR 0004: Master Control Protocol (MCP) Architecture

- Status: Draft
- Date: 2026-01-28
- Owner: GDS Core
- Related: ADR 0001 (MCP for IO), ADR 0002 (Storage Model), ADR 0003 (CoreGraphStore on MCP)

## Context

We need a **dedicated MCP architecture** that goes beyond Polars IO. MCP should act as a data-processing control plane (an “operating system”) that schedules and monitors long-running work: ETL, graph builds, projections, and pre-training pipelines. ClawdBot-style interfaces (e.g., Mission Control, Morning Brief) highlight the need for orchestration, progress reporting, and system-level coordination.

## Decision

Define MCP as a **general control plane** that manages:

- **Collections layer** (Polars DataFrame/LazyFrame control).
- **GraphFrame layer** (graph semantics on top of Polars).
- **Jobs and scheduling** (ETL, projection builds, training data pipelines).
- **Progress + observability** (metrics, status, logs, summaries).

MCP remains the stable, top-level control boundary; lower layers (Collections, GraphFrame, GraphStore) are built **under** MCP and do not replace it.

## Goals

- Provide a single **system-level control plane** for data + graph operations.
- Support long-running jobs with **scheduling, checkpoints, and progress reports**.
- Expose user-facing control surfaces (Mission Control / Morning Brief).
- Keep GraphStore integration **optional and layered** above GraphFrame.

## Non-Goals

- Replace algorithm runtime APIs in `gds/src/types`.
- Bind to external NAPI/JS loaders.
- Over-specify UI/UX; define interface contracts only.
- Compete with GUI-first platforms (e.g., Obsidian/ClawdBot-style products).

## Architecture Overview

### Control Surfaces

- **Mission Control**: active operations, running jobs, resources, errors.
- **Morning Brief**: daily/periodic summary of system status and backlogs.

> These are **system summaries**, not full UI products. The MCP remains a core operating
> layer that can be surfaced by different front-ends without becoming a GUI platform.

### Core Subsystems

1. **MCP Scheduler**
   - Job queue, priorities, retries, rate limits.
   - Supports scheduled tasks and long-running pipelines.

2. **Job Executor**
   - Executes ETL, projection builds, training data prep.
   - Provides checkpoints and resumable state.

3. **Dataset/Graph Registry**
   - Tracks datasets, GraphFrames, and derived projections.
   - Connects storage metadata to operational status.

4. **Observability**
   - Progress metrics, health checks, logs, error reports.
   - Emits summary snapshots for Morning Brief.

### Layered Responsibilities

- **Collections**: Polars table/column policy, IO adapters, memory policy.
- **GraphFrame**: graph semantics (CSR, nodes/edges tables).
- **GraphStore**: optional facade for compatibility with `gds/src/types`.
- **MCP**: orchestrates all layers and runs jobs.

## Interfaces (Conceptual)

- `McpScheduler`: enqueue, schedule, cancel, retry.
- `McpJob`: lifecycle hooks, checkpoints, progress events.
- `McpRegistry`: dataset/graph/projection catalog.
- `McpMonitor`: metrics + reporting (Mission Control / Morning Brief).

## Job Types (Initial)

- **ETL**: ingest, validate, normalize, partition.
- **Projection Build**: materialize GraphFrame or adjacency indexes.
- **Training Prep**: feature extraction, splits, exports.

## Risks

- Scope creep into UI and full OS design.
- Complex scheduling semantics without a minimal kernel first.

## Next Steps

- Define minimal MCP interfaces in Rust (scheduler, job, registry, monitor).
- Map current Polars/Collections work into MCP responsibilities.
- Add a Mission Control summary schema for progress reporting.

## References

- Polars control thread: gds/doc/POLARS-MASTER-CONTROL-THREAD.md
- Polars ↔ Collections API: gds/doc/POLARS-COLLECTIONS-API.md
- Polars API research: gds/doc/POLARS-API-RESEARCH.md
