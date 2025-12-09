# Malloy-Lite: Evolutionary IR Extension

## What is Malloy-Lite?

A **subset of Malloy** that extends our SDSL IR (Intermediate Representation) incrementally, without breaking existing code.

## Current State

### React Dashboard (examples/dashboard)
- ✅ Uses semantic modeling (DashboardModel, RevenueModel, etc.)
- ✅ Works with direct model methods
- ✅ **No changes needed** - continues working

### SDSL Core
- ✅ Basic ViewQuery (group_by, aggregate, filter, limit, order_by)
- ✅ DataModel and DataView classes
- ✅ Helper functions (sum, count, avg, dimension)

## Malloy-Lite Extensions

### 1. Extended ViewQuery (Non-Breaking)

**Added:**
- `where?: FilterDefinition[]` - Structured filters
- `parameters?: ViewParameter[]` - View parameters

**Backward Compatible:**
- All existing code continues to work
- New features are optional

### 2. ViewSpec (Reusable Query Definitions)

**New Type:**
```typescript
interface ViewSpec {
  id: string;
  name?: string;
  model: string;
  group_by?: string[];
  aggregate?: string[];
  where?: FilterDefinition[];
  limit?: number;
  order_by?: OrderDefinition[];
  parameters?: ViewParameter[];
}
```

**Usage:**
```typescript
// Define reusable view
const revenueByRegionView: ViewSpec = {
  id: 'revenue-by-region',
  model: 'Customer',
  group_by: ['region'],
  aggregate: ['totalRevenue'],
};

// Use with model
const view = customerModel.viewFromSpec(revenueByRegionView);
```

## Evolutionary Path

### Phase 1: Current (React Dashboard)
- Direct model methods
- Manual aggregations
- ✅ **No changes needed**

### Phase 2: Malloy-Lite Available
- ViewSpec for reusable queries
- Extended ViewQuery with Malloy features
- ✅ **Backward compatible** - React Dashboard still works

### Phase 3: Future (Malloy App)
- Full Malloy Dashboard
- View collection
- Automatic execution

## What Malloy/Looker Does with Views

**Key Concepts:**
- Views = Reusable query definitions
- Views can be nested or refined
- Views enable modular, maintainable code
- Views build upon existing queries

**Our Approach:**
- Start with ViewSpec (reusable queries)
- Add nesting when needed (turtle)
- Add refinement when needed
- Let usage patterns guide features

## Implementation

### Files Updated
- `src/data/sdsl.ts` - Extended ViewQuery, added ViewSpec

### Non-Breaking Changes
- All existing code continues to work
- New features are optional
- React Dashboard unchanged

### Migration Path
- React Dashboard: Continue using current models
- New code: Can use ViewSpec
- Gradual migration when ready

## Next Steps

1. ✅ Extended ViewQuery IR
2. ✅ Added ViewSpec type
3. ✅ Added viewFromSpec() method
4. 🔄 Create Malloy-Lite examples
5. 🔄 Build Malloy App when ready

---

**Key Insight**: Malloy-Lite extends IR naturally. React Dashboard continues working. Views become reusable queries. Evolution, not revolution.

