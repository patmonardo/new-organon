# Reorganization: Data/Execution Split & SDSL Simplification

## Changes

### 1. Split Data and Execution

**Before:**
```
src/data/
  ├── polars-engine.ts
  ├── sql-engine.ts
  ├── semantic-hydrator.ts
  ├── sdsl.ts
  └── services...
```

**After:**
```
src/
  ├── data/              # Data services only
  │   ├── entity.service.ts
  │   ├── dashboard.service.ts
  │   ├── fact-store.ts
  │   └── sdsl.ts        # SDSL core (semantic modeling)
  │
  └── execution/         # Execution engines
      ├── polars-engine.ts
      ├── sql-engine.ts
      └── semantic-hydrator.ts
```

### 2. Simplified SDSL

**Before:** Complex with generic types, FieldDefinition, etc.

**After:** Clean, focused on BI:
- `DataModel` - semantic data model
- `DataView` - query definition
- `MeasureDefinition` - aggregations
- `DimensionDefinition` - dimensions
- Helper functions: `sum()`, `count()`, `avg()`, `dimension()`

### 3. Updated Exports

**src/index.ts:**
```typescript
// SDSL Core
export * from './sdsl';

// Execution engines
export * from './execution';

// Data services
export * from './data';
```

**src/sdsl/index.ts:**
- Re-exports from `data/sdsl` (SDSL core lives there)

**src/execution/index.ts:**
- Exports all execution engines

**src/data/index.ts:**
- Exports only data services (no execution)

## Benefits

1. **Clear Separation**: Data services vs execution engines
2. **Simplified SDSL**: Focused on BI, no unnecessary complexity
3. **Better Organization**: Each directory has a clear purpose
4. **Standalone**: Still zero dependencies on GDS/GDSL/Logic/Task

## Structure

```
@organon/model/
├── src/
│   ├── sdsl/           # SDSL Core (re-exports from data/sdsl)
│   ├── execution/      # Execution engines
│   ├── data/           # Data services + SDSL core
│   └── schema/         # Zod schemas
```

---

**Result**: Clean separation of concerns, simplified SDSL, better organization.

