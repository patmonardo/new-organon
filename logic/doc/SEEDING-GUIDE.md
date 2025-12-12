# Database Seeding & Debugging Guide

This guide explains how to seed, reset, and debug the Neo4j database used by the FormDB system.

## Quick Start

```bash
# Build the project first
pnpm build

# Check current database state
pnpm db:stats

# Reset database (wipes everything)
pnpm db:reset

# Seed with basic forms (Invoice + Customer)
pnpm seed

# Seed with enhanced dataset (Forms + Entities + Relationships)
pnpm seed:enhanced

# Run integration tests
pnpm test:integration
```

## Architecture Overview

### Layers

```
┌─────────────────────────────────────┐
│ Engines (Business Logic)            │
│ - ShapeEngine (Form operations)     │
│ - EntityEngine (Entity operations)  │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│ Repository Adapters                  │
│ - FormShapeRepositoryAdapter        │
│ - EntityShapeRepositoryAdapter      │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│ Neo4j Repositories                  │
│ - FormShapeRepository               │
│ - EntityShapeRepository             │
└─────────────────────────────────────┘
              ↓
┌─────────────────────────────────────┐
│ Neo4j Database                       │
└─────────────────────────────────────┘
```

### Key Concepts

- **Forms (FormShape)**: Rational structure - the "blueprint" or schema
- **Entities (EntityShape)**: Empirical instances - actual data referencing a Form
- **Engines**: Business logic layer that handles commands and emits events
- **Repositories**: Persistence layer that stores/retrieves from Neo4j

## Seeding Scripts

### Basic Seed (`seed-forms.ts`)

Creates minimal test data:
- Invoice Form
- Customer Form

**Usage:**
```bash
pnpm seed
```

### Enhanced Seed (`seed-enhanced.ts`)

Creates a rich dataset using the full Engine stack:
- **Forms**: Invoice, Customer, Product
- **Entities**: 3 Customers, 4 Products, 4 Invoices
- **Relationships**: Invoices linked to Customers

**Usage:**
```bash
pnpm seed:enhanced
```

**Why use enhanced seed?**
- Tests the full Engine → Repository → Neo4j stack
- Creates realistic data patterns
- Demonstrates relationships between entities
- Better for debugging and development

## Database Utilities

### Check Database Stats

```bash
pnpm db:stats
```

Shows:
- Total nodes and relationships
- Breakdown by label/type
- Sample forms and entities

### Reset Database

```bash
pnpm db:reset
```

**⚠️ Warning**: This deletes ALL nodes and relationships!

## Integration Tests

The integration tests verify that Engines work correctly with Neo4j:

```bash
pnpm test:integration
```

Tests:
- `shape-engine.neo4j.test.ts` - ShapeEngine CRUD operations
- `entity-engine.neo4j.test.ts` - EntityEngine CRUD operations

### Test Helper: `createFormDb`

The `createFormDb` helper sets up the full stack for testing:

```typescript
const { shapeEngine, entityEngine, shapeRepo, entityRepo } = createFormDb();
```

This creates:
- Neo4j repositories
- Repository adapters
- Engines wired to the adapters
- Event bus

## Debugging Workflow

### 1. Check Current State

```bash
pnpm db:stats
```

### 2. Reset if Needed

```bash
pnpm db:reset
```

### 3. Seed Fresh Data

```bash
pnpm seed:enhanced
```

### 4. Run Tests

```bash
pnpm test:integration
```

### 5. Inspect in Neo4j Browser

Connect to `neo4j://localhost:7687` and run:

```cypher
// See all forms
MATCH (f:FormShape) RETURN f

// See all entities
MATCH (e:Entity) RETURN e

// See relationships
MATCH (a)-[r]->(b) RETURN a, r, b LIMIT 50

// Count everything
MATCH (n) RETURN labels(n)[0] as label, count(n) as count
```

## Common Issues

### "Failed to connect to Neo4j"

- Check Neo4j is running: `docker ps` or check your Neo4j service
- Verify connection settings in `src/repository/neo4j-client.ts`
- Default: `neo4j://localhost:7687`, user: `neo4j`, password: `pjm070FF`

### "Database is empty after seed"

- Check seed script output for errors
- Verify Neo4j connection
- Check Neo4j logs for errors
- Try running `pnpm db:stats` to verify

### "Integration tests fail"

- Ensure database is seeded: `pnpm seed:enhanced`
- Check Neo4j connection
- Review test output for specific errors
- Tests use `createFormDb` which creates its own test data

## File Structure

```
logic/
├── src/
│   ├── repository/
│   │   ├── initial-data/
│   │   │   ├── seed-forms.ts          # Basic seed
│   │   │   └── seed-enhanced.ts      # Enhanced seed (uses Engines)
│   │   ├── form.ts                   # FormShapeRepository
│   │   └── entity.ts                 # EntityShapeRepository
│   ├── relative/form/
│   │   ├── shape/
│   │   │   └── shape-engine.ts       # ShapeEngine
│   │   └── entity/
│   │       └── entity-engine.ts      # EntityEngine
│   └── tools/
│       ├── run-seed.ts
│       ├── run-seed-enhanced.ts
│       ├── db-reset.ts
│       └── db-stats.ts
└── test/integration/
    ├── createFormDb.ts               # Test helper
    ├── shape-engine.neo4j.test.ts
    └── entity-engine.neo4j.test.ts
```

## Next Steps

1. **Explore the Engines**: Read `shape-engine.ts` and `entity-engine.ts` to understand business logic
2. **Explore Repositories**: Read `form.ts` and `entity.ts` to understand persistence
3. **Add More Seed Data**: Extend `seed-enhanced.ts` with your own forms/entities
4. **Write Tests**: Add integration tests for your specific use cases

## Philosophy Notes

- **Form = Rational**: Structure, schema, blueprint (stored in Neo4j)
- **Entity = Empirical**: Instance, data, actualization (references Form, values stored in Model/Postgres)
- **Separation**: FormDB (Neo4j) stores transcendental linkage, Model (Postgres) stores empirical values

This separation ensures:
- Forms are reusable blueprints
- Entities reference Forms explicitly (no "fake immediacy")
- Clear Thinking-work required to connect Form → Entity → Data

