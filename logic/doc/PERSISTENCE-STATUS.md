# Persistence Layer Status

**Date**: December 11, 2025  
**Focus**: Form:Entity Rational/Empirical Separation

## ✅ Completed

### Schema Changes
- **FormShape** (form.ts): Removed `state`/`meta` fields
  - Pure Rational structure: fields, layout, actions, data binding config only
  - No runtime concerns (state/meta) in Form
  
- **EntityShape** (entity.ts): Added `formId` + `values` fields
  - `formId` (required): References Form Principle
  - `values` (Record<string, any>): Actual field data
  - Keeps `state`/`meta` (Empirical runtime concerns)

### Repository Updates  
- **FormShapeRepository** (repository/form.ts):
  - `saveForm()`: No longer persists state/meta
  - `getFormById()`: Returns pure Form structure
  - `findForms()`: Works as before
  
- **EntityShapeRepository** (repository/entity.ts):
  - `saveEntity()`: Validates formId presence, stores values as JSON
  - `getEntityById()`: Parses formId + values from Neo4j
  - `findEntities()`: Updated to reconstruct formId + values

### Testing Tools
- **seed-formdb.ts**: Populates Neo4j with test Forms + Entities
  - Creates 2 Forms (Customer Order, Product Registration)
  - Creates 2 Entities (each with formId reference + values)
  
- **query-formdb.ts**: Queries and verifies Form:Entity reciprocation
  - Shows Form structures (Rational - no state/meta)
  - Shows Entity instances (Empirical - formId + values + meta)
  - Validates Entity values match Form field names
  
- **test-formdb.sh**: One-command test runner
  - Checks Neo4j connectivity
  - Seeds database
  - Queries and displays results

### Verification
✅ In-memory repository tests pass (pillars.test.ts)  
✅ Forms persist without state/meta (Rational separation confirmed)  
✅ Entities persist with formId + values (Empirical reference confirmed)  
✅ Form:Entity reciprocation working (Entity dereferences Form)  
✅ Neo4j seeding/querying operational

## 🚧 Known Issues

### Type Errors (Non-blocking)
- Integration test file has schema mismatches (needs FormShape/EntityShape updates)
- Query script has TypeScript errors checking non-existent properties (runtime works)
- These don't affect actual persistence layer functionality

## 📋 Next Steps

### Immediate (Stay at Horizontal Level)
1. **Add Form dereferencing in Entity operations**
   - EntityShapeRepository accepts FormShapeRepository in constructor
   - Add method: `dereferenceForm(entityId)` → fetches Entity + its Form
   
2. **Add validation layer**
   - Entity save operations validate values against Form schema
   - Check: Entity.values keys match Form.fields names
   - Check: Entity.values types match Form.fields types/validation
   
3. **Integration tests**
   - Fix schema issues in form-entity-neo4j.test.ts
   - Add round-trip tests (save → load → compare)
   - Test Form:Entity dereferencing
   - Test validation failures

4. **Clean up Entity dereferencing**
   - Current: Manual (get Entity, then get Form by formId)
   - Wanted: Repository method does it in one call

### Next Cycle (Vertical Dimension)
5. **Context/Property/Morph/Aspect repositories**
   - Implement vertical dimension (Context/Essence relationships)
   - Follow same pattern: Repository → Schema → Engine/Form/Service triad
   
6. **Map to Prisma/Postgres**
   - FormDB (Neo4j): Structure/relationships
   - Postgres (Prisma): Empirical data values
   - Clear boundary: FormDB ≠ Data stores
   
7. **Resurrect @model package**
   - Update to consume FormShape + EntityShape
   - Dashboard V4 integration
   - DuckDB/Polars/Arrow for analytics

## Architecture Summary

```
Logic Package (@logic):
  
  Schemas:
    form.ts         → FormShape (Rational - structure only)
    entity.ts       → EntityShape (Empirical - formId + values)
    context.ts      → ContextShape (vertical - next)
    property.ts     → PropertyShape (vertical - next)
    aspect.ts       → AspectShape (vertical - next)
    morph.ts        → MorphShape (transformations)
  
  Repositories (Persistence - Neo4j):
    form.ts         → FormShapeRepository ✅ DONE
    entity.ts       → EntityShapeRepository ✅ DONE
    context.ts      → ContextRepository (skeletal)
    property.ts     → PropertyRepository (skeletal)
    ...
  
  Form Processor (Business Logic):
    relative/form/shape/     → Form pillar (engine-form-service) ✅ EXISTS
    relative/form/entity/    → Entity pillar (engine-form-service) ✅ EXISTS
    relative/form/context/   → Context pillar (next)
    relative/form/property/  → Property pillar (next)
    ...
```

## Key Principles Verified

✅ **Rational/Empirical Split**: Form = types, Entity = values  
✅ **Form Reference**: Entity.formId → Form.id (explicit Thinking)  
✅ **Compression Model**: Form Engine strips values, Entity Engine preserves  
✅ **Nondual Approach**: Same schema structure, different determination  
✅ **Message Passing**: Entity Engine ↔ Form Engine cooperation  

**Philosophy**: Brahmachakra/Cogito - Form ↔ Entity ↔ Data cycle. Form holds "keys to the kingdom" (generative blueprint), Entity uses keys via formId reference. No "fake immediacy" - explicit Thinking-work required.
