# Malloy Evolutionary Path: From React Dashboard to Malloy App

## Current State

### React Dashboard (examples/dashboard)

**Already uses semantic modeling:**
- `DashboardModel` - aggregates data (customers, invoices, revenue)
- `RevenueModel` - revenue calculations
- `CustomerModel` - customer data
- `InvoiceModel` - invoice data

**Structure:**
```
examples/dashboard/
├── app/
│   ├── model/        # Models (DashboardModel, RevenueModel, etc.)
│   ├── view/         # Views (DashboardView, RevenueView, etc.)
│   └── controller/   # Controllers (DashboardController, etc.)
```

**Current approach:**
- Direct model methods (`DashboardModel.getCardData()`)
- Manual aggregations
- React components consume model data directly

## Malloy-Lite: Evolutionary Subset

### What is Malloy-Lite?

A **subset of Malloy** that:
- Extends our current SDSL IR (Intermediate Representation)
- Adds Malloy concepts gradually
- Doesn't break existing React Dashboard
- Informs evolution naturally

### Malloy View System (What We Learn)

**From Malloy/Looker:**
- **Views** = Reusable query definitions
- Views can be **nested** or **refined**
- Views enable **modular, maintainable** code
- Views build upon existing queries

**Malloy View Example:**
```malloy
view: revenue_by_region {
  group_by: region
  aggregate: total_revenue
}

view: top_customers {
  group_by: customer_name
  aggregate: total_revenue
  limit: 10
}
```

## Evolutionary Path

### Phase 1: React Dashboard (Current)

**Status:** ✅ Working

**Approach:**
- Direct model methods
- Manual aggregations
- React components

**Example:**
```typescript
// Current: Direct model method
const data = await DashboardModel.getCardData();
```

### Phase 2: Malloy-Lite IR Extension

**Goal:** Extend SDSL IR to support Malloy concepts

**Changes:**
1. **Extend ViewQuery** to support Malloy features
2. **Add ViewSpec** for reusable query definitions
3. **Keep backward compatibility** with current models

**Example:**
```typescript
// New: Malloy-Lite view spec
const revenueByRegionView: ViewSpec = {
  id: 'revenue-by-region',
  model: 'Customer',
  group_by: ['region'],
  aggregate: ['totalRevenue'],
};

// Can still use old way
const data = await DashboardModel.getCardData();

// Or new way (gradual migration)
const view = customerModel.view(revenueByRegionView);
const result = await engine.execute(view);
```

### Phase 3: Malloy App (Future)

**Goal:** Full Malloy Dashboard

**Approach:**
- Dashboard as View collection
- Malloy View definitions
- Automatic execution and rendering

**Example:**
```typescript
// Future: Malloy Dashboard
const dashboard = new MalloyDashboard({
  views: [
    { id: 'revenue-by-region', viewSpec: revenueByRegionView },
    { id: 'top-customers', viewSpec: topCustomersView },
  ],
  layout: { gridColumns: 12 },
});
```

## Malloy-Lite IR Extension

### Current IR (SDSL)

```typescript
interface ViewQuery {
  group_by?: string[];
  aggregate?: string[];
  filter?: Record<string, any>;
  limit?: number;
}
```

### Extended IR (Malloy-Lite)

```typescript
interface ViewQuery {
  // Current (backward compatible)
  group_by?: string[];
  aggregate?: string[];
  filter?: Record<string, any>;
  limit?: number;
  
  // Malloy-Lite extensions
  order_by?: OrderDefinition[];
  where?: FilterDefinition[];  // More structured filters
  parameters?: ViewParameter[]; // View parameters
  turtle?: TurtleDefinition[]; // Nested aggregations (future)
  reduce?: string[];            // Simplified aggregation (future)
}
```

### ViewSpec (Reusable Query Definitions)

```typescript
interface ViewSpec {
  id: string;
  name?: string;
  model: string;  // Reference to DataModel
  group_by?: string[];
  aggregate?: string[];
  where?: FilterDefinition[];
  limit?: number;
  order_by?: OrderDefinition[];
  parameters?: ViewParameter[];
}
```

## How This Informs Evolution

### 1. React Dashboard → Malloy Dashboard

**Current:**
```typescript
// examples/dashboard/app/model/dashboard.ts
export class DashboardModel {
  static async getCardData(): Promise<CardData> {
    const numberOfCustomers = await CustomerModel.count();
    const totalPaidInvoices = await InvoiceModel.getTotalByStatus("PAID");
    // Manual aggregations
  }
}
```

**Evolution (Malloy-Lite):**
```typescript
// Can gradually migrate to views
const customerCountView: ViewSpec = {
  id: 'customer-count',
  model: 'Customer',
  aggregate: ['customerCount'],
};

// Still works with old models
const data = await DashboardModel.getCardData();

// But can also use new views
const view = customerModel.view(customerCountView);
const result = await engine.execute(view);
```

### 2. Extending IR Naturally

**IR Extension Strategy:**
- Add Malloy features to IR incrementally
- Keep backward compatibility
- Let usage patterns emerge

**Example:**
```typescript
// Start simple
interface ViewQuery {
  group_by?: string[];
  aggregate?: string[];
}

// Add Malloy features as needed
interface ViewQuery {
  // ... existing
  turtle?: TurtleDefinition[];  // When we need nested aggregations
  reduce?: string[];              // When we need simplified aggregation
}
```

### 3. Views as Reusable Queries

**Current:** Each model has specific methods

**Evolution:** Views become reusable across models

```typescript
// Reusable view
const revenueView: ViewSpec = {
  id: 'revenue',
  model: 'Invoice',
  group_by: ['customerId'],
  aggregate: ['totalRevenue'],
};

// Can use with different models
const customerRevenue = customerModel.view(revenueView);
const invoiceRevenue = invoiceModel.view(revenueView);
```

## Implementation Plan

### Step 1: Extend IR (Non-Breaking)

**File:** `src/data/sdsl.ts`

```typescript
// Add to existing ViewQuery (backward compatible)
export interface ViewQuery {
  // Existing
  group_by?: string[];
  aggregate?: string[];
  filter?: Record<string, any>;
  limit?: number;
  
  // New (optional, doesn't break existing code)
  order_by?: OrderDefinition[];
  where?: FilterDefinition[];
  parameters?: ViewParameter[];
}
```

### Step 2: Add ViewSpec

**File:** `src/schema/view-spec.ts` (new)

```typescript
export interface ViewSpec {
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

### Step 3: Keep React Dashboard Working

**No changes to examples/dashboard** - it continues to work as-is

**Gradual migration path:**
- React Dashboard keeps using current models
- New Malloy features available for new code
- Can migrate incrementally when ready

### Step 4: Malloy App (Future)

**New example:** `examples/malloy-dashboard/`

```typescript
// Full Malloy Dashboard
const dashboard = new MalloyDashboard({
  views: [
    { id: 'revenue-by-region', viewSpec: revenueByRegionView },
    { id: 'top-customers', viewSpec: topCustomersView },
  ],
});
```

## Key Principles

1. **Don't Force**: React Dashboard continues working
2. **Evolutionary**: Add Malloy features incrementally
3. **IR Extension**: Extend IR naturally as needs emerge
4. **Malloy-Lite**: Start with subset, grow as needed
5. **Views Inform**: Let Malloy Views inform how SDSL evolves

## What Malloy/Looker Does with Views

**Malloy Views:**
- Reusable query definitions
- Can be nested or refined
- Enable modular, maintainable code
- Build upon existing queries

**Our Evolution:**
- Start with simple ViewSpec
- Add nesting when needed
- Add refinement when needed
- Let usage patterns guide features

## Next Steps

1. ✅ Extend ViewQuery IR (non-breaking)
2. ✅ Add ViewSpec type
3. ✅ Keep React Dashboard working
4. 🔄 Create Malloy-Lite examples
5. 🔄 Build Malloy App when ready

---

**Key Insight**: Malloy-Lite extends our IR naturally. React Dashboard continues working. Malloy features inform evolution without forcing changes. Views become reusable queries that can gradually replace direct model methods.

