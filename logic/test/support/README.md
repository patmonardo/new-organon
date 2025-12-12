# FormDB Persistence Layer Tests

Simple testing tools for the Form:Entity persistence layer in Neo4j.

## Overview

Tests the **Rational vs Empirical** distinction:

- **FormShape** (Rational): Pure structure - "keys to the kingdom"
  - Holds field definitions, layout, data binding config
  - NO state/meta (removed - Rational concerns only)
  - Form Engine caches these schemas

- **EntityShape** (Empirical): Actual data instances
  - References Form via `formId` (Form Principle)
  - Carries `values` (actual field data)
  - Has `state`/`meta` (runtime/Empirical concerns)
  - Entity Engine dereferences Form for schema

## Prerequisites

Neo4j running at `localhost:7687`:

```bash
docker run -p 7687:7687 -p 7474:7474 \\
  -e NEO4J_AUTH=neo4j/pjm070FF \\
  neo4j:latest
```

Or set environment variables:
```bash
export NEO4J_URI=neo4j://localhost:7687
export NEO4J_USER=neo4j
export NEO4J_PASSWORD=your-password
```

## Usage

### Seed Database

Populate Neo4j with test Forms and Entities:

```bash
pnpm tsx test/support/seed-formdb.ts
```

Creates:
- 2 Forms (Customer Order, Product Registration)
- 2 Entities (Order Instance, Product Instance)

Each Entity references its Form via `formId`.

### Query Database

Inspect persisted data and verify Form:Entity reciprocation:

```bash
pnpm tsx test/support/query-formdb.ts
```

Shows:
- Form structures (Rational - keys only)
- Entity instances (Empirical - formId + values)
- Form:Entity reciprocation (Entity values match Form fields)

### Run All Tests

```bash
./test/support/test-formdb.sh
```

Runs: seed → query → integration tests

## What We've Verified

✅ **FormShape** persists without `state`/`meta` (Rational separation)  
✅ **EntityShape** persists with `formId` + `values` (Empirical reference)  
✅ **Form:Entity reciprocation** - Entity dereferences Form for structure  
✅ **Compression model** - Form Engine ignores values, Entity Engine preserves them  
✅ **Nondual approach** - Same schema structure, different determination  

## Architecture

```
Repository Layer (persistence):
  logic/src/repository/
    form.ts         → FormShapeRepository (Neo4j CRUD for Forms)
    entity.ts       → EntityShapeRepository (Neo4j CRUD for Entities)
    
Form Processor Layer (business logic):
  logic/src/relative/form/
    shape/          → Form pillar (shape-engine/form/service triad)
    entity/         → Entity pillar (entity-engine/form/service triad)
    context/        → Context pillar (vertical dimension - next)
    property/       → Property pillar (vertical dimension - next)
    aspect/         → Aspect pillar (vertical dimension - next)
    morph/          → Morph pillar (transformations)
```

## Next Steps

After Form:Entity solid:
1. Context/Property/Morph/Aspect repositories (vertical dimension)
2. Validation layer (Entity values validated against Form schema)
3. Map to Prisma/Postgres (Data layer separate from FormDB)
4. Resurrect @model package for Dashboard V4 integration
5. Form dereferencing in Entity operations (Entity → formId → FormShapeRepository.getForm())

## Philosophy

**Brahmachakra / Cogito Cycle**: Form ↔ Entity ↔ Data

- **Form** (Rational): Types, structure, constraints - "what could be"
- **Entity** (Empirical): Values, state, instances - "what is"
- **Explicit Thinking**: Entity must reference Form (no "fake immediacy")

Form holds generative blueprint (keys), Entity uses keys via `formId` reference.  
Form Engine + Entity Engine cooperate via message passing (compression model).
