# MVC Data SDSL - Next Steps

## What We Accomplished

✅ **MVC Runtime** - Universal Page pattern with Controller Registry
✅ **Data SDSL Foundation** - Malloy-inspired semantic modeling (`data-sdsl.ts`)
✅ **Design Vision** - Complete architecture in `mvc-sdk-design.md`
✅ **Working Example** - Customer/Invoice dashboard with demos

## Phase 1: Complete the Data SDSL Core

### 1.1 Implement Customer Model with Measures

**File**: `model/examples/mvc/customer-model.ts`

```typescript
import { defineModel, sum, avg, count } from '../../src/data/sdsl';

export const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers', // Table or file
  fields: {
    id: z.string(),
    name: z.string(),
    region: z.string(),
  },
  measures: {
    count: count(),
    totalRevenue: sum('invoices.amount'),
    avgInvoice: avg('invoices.amount'),
  },
  dimensions: {
    region: 'region',
    signupMonth: dimension('createdAt', 'month'),
  },
  joins: {
    invoices: {
      model: InvoiceModel,
      on: 'id = customerId',
      type: 'left',
    },
  },
});
```

### 1.2 Add Polars Execution Engine

**File**: `model/src/data/polars-engine.ts`

- Install `nodejs-polars`
- Implement `DataModel.execute()` that compiles to Polars expressions
- Return Arrow Tables

### 1.3 Add SQL Generation (for Postgres)

**File**: `model/src/data/sql-engine.ts`

- Implement `DataModel.toSQL()` for basic CRUD
- Support for `INSERT`, `UPDATE`, `DELETE`, `SELECT`
- No need for migrations—just generate SQL on the fly

## Phase 2: Integration with @logic

### 2.1 Form Processor Bridge

**File**: `model/src/sdsl/form-processor-adapter.ts`

- Connect `DataModel` to `@logic` Form Processor
- Translate `FormShape` ↔ `DataModel`
- Enable IR-based form generation from models

### 2.2 Zod Schema Generation

**File**: `model/src/data/zod-generator.ts`

- Auto-generate Zod schemas from `DataModel.fields`
- Validation at the model level

## Phase 3: Advanced Features (Malloy-Inspired)

### 3.1 Nested Queries

```typescript
const TopCustomers = CustomerModel.view({
  aggregate: ['totalRevenue'],
  limit: 10,
});

const TopCustomersByRegion = TopCustomers.view({
  group_by: ['region'],
});
```

### 3.2 Time Intelligence

```typescript
measures: {
  revenueYTD: sumIf('amount', dateRange('YTD')),
  revenueMTD: sumIf('amount', dateRange('MTD')),
}
```

### 3.3 Percent of Total

```typescript
measures: {
  revenueShare: percentOfTotal('totalRevenue', 'region');
}
```

## Phase 4: Remove Prisma ✅

### 4.1 Migrate Existing Models

- ✅ Converted Prisma schema references to `DataModel` definitions
- ✅ Updated controllers to use the Data SDSL + FactStore bridge instead of Prisma Client

### 4.2 Simplify Build

- ✅ Removed `prisma generate` (and related) scripts from package builds
- ✅ Removed Prisma dependencies
- ✅ Updated documentation to reflect the semantic-only stack

## Phase 5: Production Readiness

### 5.1 Testing

- Unit tests for `data-sdsl.ts` types
- Integration tests with Polars
- SQL generation tests

### 5.2 Documentation

- API reference for Data SDSL
- Migration guide from Prisma
- Example gallery

### 5.3 Performance

- Benchmark Polars vs SQL for common queries
- Optimize Arrow Table serialization
- Add query caching layer

## Quick Wins (Do First)

1. **Implement `customer-model.ts`** - Prove the concept works
2. **Add Polars execution** - Show real analytics
3. **Create demo dashboard** - Visualize the results
4. **Document the API** - Make it easy to adopt

## Long-Term Vision

Your Data SDSL becomes the **universal data layer** for:

- Analytics (Polars/Arrow)
- Persistence (Postgres/DuckDB)
- Forms (@logic Form Processor)
- APIs (tRPC/GraphQL)

All from **one model definition**. No Prisma, no codegen, no complexity.

---

**Status**: Foundation complete. Ready for Phase 1 implementation.
