# Getting Started with @organon/model

## Prerequisites

- Node.js 18+
- pnpm
- TypeScript

## Installation

```bash
pnpm add @organon/model
```

## Quick Start

### 1. Define a Form Shape

```typescript
import { z } from 'zod';

const customerShape = {
  id: 'customer-form',
  name: 'Customer',
  fields: [
    { id: 'name', type: 'text', label: 'Name', required: true },
    { id: 'email', type: 'email', label: 'Email', required: true },
    { id: 'region', type: 'select', label: 'Region', options: ['North', 'South', 'East', 'West'] },
  ],
  layout: {
    id: 'customer-layout',
    label: 'Customer Form',
    columns: 2,
  },
};
```

### 2. Create MVC Stack

```typescript
import { createFormMVC } from '@organon/model';

const mvc = createFormMVC(customerShape, 'edit');
```

### 3. Get Display Document

```typescript
const doc = mvc.display();
// Returns framework-agnostic DisplayDocument
```

### 4. Render with Adapter

```typescript
import { reactAdapter } from '@organon/model';

// In a React component
const CustomerForm = () => {
  const jsx = reactAdapter.render(doc, {
    handler: mvc.createHandler(),
  });
  return <>{jsx}</>;
};
```

## Server Actions (Next.js)

### Define Server Action

```typescript
// app/actions/customer.ts
'use server';

import { ReactController } from '@organon/model';

export async function createCustomer(formData: FormData) {
  const controller = ReactController.create(customerShape, 'create', {
    redirectOnSuccess: '/customers',
    revalidatePaths: ['/customers'],
  });
  return await controller.createSubmitAction()(formData);
}
```

### Use in Page

```typescript
// app/customers/create/page.tsx
import { createCustomer } from '@/app/actions/customer';

export default async function CreateCustomerPage() {
  return (
    <form action={createCustomer}>
      {/* Form fields */}
      <button type="submit">Save</button>
    </form>
  );
}
```

## Semantic Data Model

### Define Model with Measures

```typescript
import { defineModel, sum, count, avg } from '@organon/model';
import { z } from 'zod';

const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  fields: {
    id: z.string(),
    name: z.string(),
    email: z.string(),
    region: z.string(),
    revenue: z.number(),
  },
  measures: {
    totalRevenue: sum('revenue'),
    customerCount: count(),
    avgRevenue: avg('revenue'),
  },
  dimensions: {
    region: 'region',
    signupMonth: dimension('createdAt', 'month'),
  },
});
```

### Create a View (Query)

```typescript
const view = CustomerModel.view({
  group_by: ['region'],
  aggregate: ['totalRevenue', 'customerCount'],
  filter: { region: 'North' },
  order_by: 'totalRevenue desc',
  limit: 100,
});
```

### Execute with Polars Engine

```typescript
import { PolarsExecutionEngine } from '@organon/model';

const engine = new PolarsExecutionEngine(dataset);
const result = await engine.execute(view);

console.log(result.rows);
// [{ region: 'North', totalRevenue: 50000, customerCount: 25 }]
```

## Semantic Hydration

### Bridge Data to Forms

```typescript
import { SemanticHydrator } from '@organon/model';

const hydrator = new SemanticHydrator(dataService);

const snapshot = await hydrator.hydrate(formModel, {
  id: 'customer-profile',
  view: (ctx) => CustomerModel.view({
    filter: { id: ctx.params.customerId },
  }),
  fields: [
    { fieldId: 'name', source: 'name' },
    { fieldId: 'email', source: 'email' },
  ],
  metrics: [
    { name: 'totalRevenue', source: 'metrics.totalRevenue', fieldId: 'revenue' },
  ],
});
```

## Radix Adapter

### For Interactive UI

```typescript
import { radixAdapter } from '@organon/model';

// In a client component
'use client';

const CustomerDashboard = ({ shape, snapshot }) => {
  const mvc = createFormMVC(shape, 'view');
  const doc = mvc.display();
  
  return radixAdapter.render(doc, {
    snapshot,
    handler: mvc.createHandler(),
  });
};
```

## Next Steps

1. **Explore examples** — See `examples/dashboard/` for the current MVC demo
2. **Read architecture docs** — See `doc/INDEX.md` for full documentation
3. **Run tests** — `pnpm test` to see the test suite
4. **Build** — `pnpm build` to compile TypeScript

