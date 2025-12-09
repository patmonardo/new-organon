# Standalone Malloy BI Package

## Status: New Commit - Standalone BI Package

This package is now a **standalone Malloy BI package** with:
- ✅ **Zero dependencies** on GDS/GDSL/Logic/Task
- ✅ **BI-only** focus (Model-View-Dashboard)
- ✅ **Runs standalone** - can be used independently
- ✅ **Malloy-inspired** semantic modeling

## What Changed

### Removed
- References to Logic/Model dyad (conceptual only, no code dependency)
- Dependencies on GDS/GDSL/Logic/Task packages
- AI platform integration points

### Added
- Standalone Malloy BI structure
- Spec-based model definitions
- Model-View-Dashboard system
- Execution engines (Polars/DuckDB/Arrow)

## Architecture

```
@organon/model (Standalone)
├── malloy/        # Malloy BI Core
├── execution/     # Execution Engines
└── schema/        # Zod Schemas
```

## Usage

```typescript
import { DataModel, MalloyView, Dashboard } from '@organon/model';

// Standalone - no Logic/GDS/GDSL needed
const model = DataModel.fromSpec({...});
const view = model.view({...});
const dashboard = new Dashboard({...});
```

## Dependencies

**Only:**
- `zod` - Schema validation
- `nodejs-polars` - Execution
- `apache-arrow` - Data format
- `duckdb` - SQL execution

**Not:**
- `@organon/gds`
- `@organon/gdsl`
- `@organon/logic`
- `@organon/task`

---

**This is a standalone Malloy BI package - use it independently.**

