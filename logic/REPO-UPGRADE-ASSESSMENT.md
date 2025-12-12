# Repository Upgrade Assessment

**Date**: Current  
**Focus**: Post-Repo Upgrade State Assessment - Form:Entity Persistence

## 🎯 Current State Summary

### Naming Evolution
- **Repository**: Renamed "shape" → "Form" 
- **Neo4j Labels**: `:FormShape` (explicit in Neo4j, benefits from rename)
- **Relationship**: `Form:Entity` (Entity references Form via `formId`)
- **Question**: Should `Shape:Entity` be `Form:Entity`? 
  - ✅ **Answer**: Yes - Repository already uses Form, Neo4j uses `:FormShape`, relationship is `Form:Entity`

### Architecture Status

#### ✅ Alpha State (Types Compile)
- **FormShapeRepository**: Core CRUD operations implemented
- **EntityShapeRepository**: Core CRUD with `formId` reference
- **Schema Separation**: Form (Rational) vs Entity (Empirical) ✅
- **Neo4j Persistence**: Basic save/load working

#### 🐛 Known Bugs (Too Complex for Early Morning)

**Bug 1: `getFormById` reads non-existent fields**
- **Location**: `logic/src/repository/form.ts:475-476`
- **Issue**: Tries to parse `state` and `meta` from Neo4j
- **Problem**: `FormShapeSchema` no longer includes `state`/`meta` (removed in Rational/Empirical split)
- **Impact**: May cause type errors or undefined values
- **Fix**: Remove `state` and `meta` from reconstruction

**Bug 2: `findForms` reads non-existent fields**
- **Location**: `logic/src/repository/form.ts:704-705`
- **Issue**: Same as Bug 1 - tries to read `state`/`meta`
- **Fix**: Remove `state` and `meta` from reconstruction

**Bug 3: Cypher queries may not select state/meta**
- **Location**: `logic/src/repository/form.ts:323+` (getFormById query)
- **Issue**: Queries don't explicitly select `state`/`meta` properties
- **Impact**: `formShapeProps.state` and `formShapeProps.meta` would be undefined anyway
- **Note**: This might mask the bug (undefined won't cause errors, but it's wrong)

### Repository Maturity

| Repository | State | Notes |
|------------|-------|-------|
| FormShapeRepository | Alpha | Core CRUD done, has bugs in read operations |
| EntityShapeRepository | Alpha | Core CRUD done, formId reference working |
| ContextRepository | Alpha | Basic CRUD, skeletal |
| MorphRepository | Alpha | Basic CRUD, skeletal |
| AspectShapeRepository | Alpha | Basic CRUD, skeletal |
| PropertyShapeRepository | Alpha | Basic CRUD, skeletal |

### Form:Entity Relationship

✅ **Working**:
- Entity stores `formId` (required)
- Entity stores `values` (actual data)
- Form stores pure structure (no state/meta)
- Separation of Rational (Form) vs Empirical (Entity) ✅

🚧 **Needs Work**:
- Form dereferencing in Entity operations (manual currently)
- Validation: Entity.values against Form.fields
- Integration tests need schema updates

## 🔍 Code Inspection Findings

### FormShapeRepository (`logic/src/repository/form.ts`)

**Save Operation** (`saveForm`):
- ✅ Correctly excludes `state`/`meta` from props (line 53)
- ✅ Only saves Rational structure
- ✅ Handles subgraph (Fields, Options, Layout, Sections, Actions, Tags)

**Read Operations** (`getFormById`, `findForms`):
- 🐛 **BUG**: Tries to read `state`/`meta` (lines 475-476, 704-705)
- ✅ Correctly reconstructs subgraph
- ✅ Handles JSON parsing for complex fields

### EntityShapeRepository (`logic/src/repository/entity.ts`)

**Save Operation** (`saveEntity`):
- ✅ Validates `formId` presence (line 36)
- ✅ Stores `formId` and `values` as JSON
- ✅ Uses `:Entity` label (not `:EntityShape`)

**Read Operations** (`getEntityById`):
- ✅ Parses `formId` and `values` from Neo4j
- ✅ Reconstructs EntityShape correctly

### Neo4j Label Consistency

| Schema Type | Neo4j Label | Status |
|-------------|-------------|--------|
| FormShape | `:FormShape` | ✅ Explicit, clear |
| EntityShape | `:Entity` | ✅ Simple, works |
| ContextShape | `:Context` | ✅ Consistent |
| MorphShape | `:Morph` | ✅ Consistent |
| AspectShape | `:Aspect` | ✅ Consistent |

**Observation**: Form uses `:FormShape` (explicit), others use base name. This is intentional - Form benefits from explicit "Shape" in Neo4j.

## 📋 Next Steps (Priority Order)

### 1. Fix Read Operations (Critical)
- [ ] Remove `state`/`meta` from `getFormById` reconstruction (line 475-476)
- [ ] Remove `state`/`meta` from `findForms` reconstruction (line 704-705)
- [ ] Verify Cypher queries don't try to select these fields
- [ ] Test: Save Form → Load Form → Verify no state/meta

### 2. Add Form Dereferencing (High Value)
- [ ] Add `FormShapeRepository` dependency to `EntityShapeRepository`
- [ ] Add method: `getEntityWithForm(entityId)` → returns Entity + Form
- [ ] Update Entity operations to optionally dereference Form

### 3. Add Validation Layer (High Value)
- [ ] Validate Entity.values keys match Form.fields names
- [ ] Validate Entity.values types match Form field types
- [ ] Add validation errors to Entity save operations

### 4. Integration Tests (Medium)
- [ ] Fix schema mismatches in `form-entity-neo4j.test.ts`
- [ ] Add round-trip tests (save → load → compare)
- [ ] Test Form:Entity dereferencing
- [ ] Test validation failures

### 5. Documentation (Low)
- [ ] Update PERSISTENCE-STATUS.md with bug fixes
- [ ] Document Form:Entity relationship patterns
- [ ] Add examples of Form dereferencing

## 🎨 Naming Clarity

**Current State**:
- Schema: `FormShape` (TypeScript type)
- Repository: `FormShapeRepository` (class name)
- Neo4j: `:FormShape` (label)
- Relationship: `Form:Entity` (conceptual)

**Decision**: ✅ Keep `FormShape` naming - it's explicit in Neo4j and benefits from the rename. The "Shape" suffix clarifies it's the schema structure, not runtime data.

**Question Resolved**: `Shape:Entity` → `Form:Entity` ✅ (Repository already uses Form)

## 💭 Philosophy Notes

**Rational/Empirical Split**:
- Form = Rational (structure, "keys to the kingdom")
- Entity = Empirical (formId + values, actual data)
- No "fake immediacy" - explicit Thinking-work via formId reference

**Repository as Essential Embodiment**:
- Repository controls schema persistence
- FormShape explicit in Neo4j (benefits from rename)
- Entity references Form (explicit Thinking)

**Beta Goal**: "Glorious morning" - Repository needs to rear its ugly head and say "get to Beta quick because things are settling in"

## 🐛 Bug Details for Later Debugging

### Bug 1 & 2: State/Meta in Read Operations

**Files**: `logic/src/repository/form.ts`

**Lines**:
- `getFormById`: 475-476
- `findForms`: 704-705

**Current Code**:
```typescript
state: this.safeJsonParse(formShapeProps.state),
meta: this.safeJsonParse(formShapeProps.meta),
```

**Problem**:
- `FormShapeSchema` doesn't include `state`/`meta` (removed in Rational/Empirical split)
- These fields don't exist in Neo4j (not saved in `saveForm`)
- Reading them returns `undefined`, which gets parsed as `null`

**Fix**:
```typescript
// Remove these lines - FormShape doesn't have state/meta
// state: this.safeJsonParse(formShapeProps.state),
// meta: this.safeJsonParse(formShapeProps.meta),
```

**Testing**:
1. Save a Form (should not have state/meta)
2. Load the Form
3. Verify returned FormShape doesn't have state/meta properties
4. Verify FormShapeSchema.parse() succeeds

---

**Status**: Ready for debugging session (not early morning work 😊)

