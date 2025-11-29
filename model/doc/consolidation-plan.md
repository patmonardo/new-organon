# Examples & Tests Consolidation Plan

## Current State

### Examples
- `examples/customer/` - **Semantic MVC** (ReactController, SemanticHydrator, Polars) - **PRIMARY**
- `examples/archive/customer-traditional/` - Traditional MVC (FormController, SimpleFormModel, FormView) - **ARCHIVED**

### Tests
- `test/customer-mvc.test.ts` - Tests traditional MVC
- `test/customer-radix.integration.test.tsx` - Tests semantic MVC
- `test/polars-engine.test.ts` - Tests semantic data layer
- `test/semantic-hydrator.test.ts` - Tests hydration bridge

## Rust Conventions

Following Rust project structure:
- ✅ Source code in `src/`
- ✅ Examples in `examples/`
- ✅ Tests in `test/` (not in modules)
- ✅ Documentation in `doc/`

## Consolidation Options

### Option 1: Keep Both, Organize by API Type
```
examples/
  traditional-mvc/
    customer/
      schema.ts
      model.ts
      form-model.ts
      form-view.ts
      controller.ts
      index.ts
  semantic-mvc/
    customer/
      customer.ts (schema)
      customer-model.ts (DataModel)
      customer-data.service.ts
      customer-controller.ts
      customer-view.tsx
      invoice-model.ts
      index.ts
```

**Pros:**
- Clear separation of APIs
- Easy to compare
- Both available for reference

**Cons:**
- Duplication
- Two different patterns to maintain

### Option 2: Single Customer Example, Two Approaches
```
examples/
  customer/
    traditional/
      ... (traditional MVC files)
    semantic/
      ... (semantic MVC files)
    README.md (explains both approaches)
```

**Pros:**
- Single domain (customer)
- Easy to compare side-by-side
- Clear documentation

**Cons:**
- Still duplication
- Nested structure

### Option 3: Choose One, Archive Other
```
examples/
  customer/ (chosen API)
    ... (files)
archive/
  customer-traditional/ (if not chosen)
    ... (files)
```

**Pros:**
- Single source of truth
- No confusion
- Cleaner codebase

**Cons:**
- Lose reference implementation
- Harder to compare

## Recommendation: Option 1 with Clear Naming

Keep both but make the distinction clear:

```
examples/
  traditional-mvc/
    customer/
      README.md (explains traditional MVC pattern)
      ...
  semantic-mvc/
    customer/
      README.md (explains semantic MVC pattern)
      ...
```

Then update tests to match:
```
test/
  customer-traditional.test.ts (rename from customer-mvc.test.ts)
  customer-semantic.test.ts (rename from customer-radix.integration.test.tsx)
  semantic-hydrator.test.ts
  polars-engine.test.ts
  ...
```

## Test Organization

All tests stay in `test/` following Rust conventions:
- ✅ No unit tests in source modules
- ✅ Integration tests in `test/`
- ✅ Clear naming: `{feature}-{approach}.test.ts`

## Next Steps

1. ✅ Create comparison document (`doc/api-comparison.md`)
2. ⏳ Decide which API to use as primary
3. ⏳ Reorganize examples directory
4. ⏳ Rename/update test files
5. ⏳ Update imports in tests
6. ⏳ Create README files in each example directory

