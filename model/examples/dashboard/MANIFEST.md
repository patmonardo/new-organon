# Dashboard App Manifest

## Overview

This document analyzes the structure of the `example/dashboard` application. It serves as a reference for the "Given Form" generated from the "Pure Form".

## Top-Level Path

The application is structured into a 6-layer architecture (Top-Down):

1.  **`(controller)`**: Routing / Mediation
2.  **`controller`**: Logic / Orchestration
3.  **`view`**: Representation / Perspective
4.  **`model`**: Substance / Data Logic
5.  **`graphics`**: Manifestation / UI Components
6.  **`data`**: Storage / Schema

---

## Metric Analysis: Important Files & Imports

### 1. Model Layer (`app/model`)

_Substance (Being). Extends `BaseModel`. Wraps `FormShape` and `PrismaClient`._

#### [customer.ts](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/model/customer.ts)

```typescript
import { prisma } from "@data/client";
import type { OperationResult } from "@/data/schema/base";
import type {
  Customer,
  CustomerShape,
  CreateCustomer,
  UpdateCustomer,
} from "@/data/schema/customer";
import {
  CustomerShapeSchema,
  CreateCustomerSchema,
  UpdateCustomerSchema,
} from "@/data/schema/customer";
import { BaseModel } from "./base";
```

#### [revenue.ts](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/model/revenue.ts)

```typescript
import { prisma } from "@data/client";
import {
  CreateRevenueSchema,
  UpdateRevenueSchema,
  RevenueShapeSchema,
} from "@/data/schema/revenue";
import type {
  Revenue,
  RevenueShape,
  CreateRevenue,
  UpdateRevenue,
  RevenueMetrics,
} from "@/data/schema/revenue";
import type { OperationResult } from "@/data/schema/base";
import { BaseModel } from "./base";
```

---

### 2. View Layer (`app/view`)

_Representation (Essence). Extends `FormView`. Transforms Data to Graphics._

#### [customer.ts](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/view/customer.ts)

```typescript
import { ReactNode } from "react";
import { OperationResult } from "@/data/schema/base";
import { Customer } from "@/data/schema/customer";
import { CustomerFormShape } from "@graphics/schema/customer";
import { CustomerForm } from "@graphics/form/customer";
import { CustomerTable } from "@graphics/table/customer";
import { FormView } from "./form";
```

#### [revenue.ts](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/view/revenue.ts)

```typescript
import { Revenue } from "@/data/schema/revenue";
import { RevenueForm } from "@graphics/form/revenue";
import { RevenueFormShape } from "@graphics/schema/revenue";
import { FormView } from "./form";
```

---

### 3. Controller Layer (`app/controller`)

_Mediation (Concept). Orchestrates Model + View._

#### [customer.tsx](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/controller/customer.tsx)

```typescript
import type { ReactNode } from "react";
import { notFound, redirect } from "next/navigation";
import { revalidatePath } from "next/cache";
import type { FormHandler } from "@graphics/schema/form";
import { CreateCustomer, UpdateCustomer } from "@/data/schema/customer";
import { CustomerModel } from "@model/customer";
import { CustomerView } from "@view/customer";
import createCustomer from "@/(controller)/customers/actions/create";
import updateCustomer from "@/(controller)/customers/actions/update";
import cancelCustomer from "@/(controller)/customers/actions/cancel";
```

#### [invoice.tsx](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/controller/invoice.tsx)

```typescript
import type { ReactNode } from "react";
import { notFound, redirect } from "next/navigation";
import { revalidatePath } from "next/cache";
import type { FormHandler } from "@graphics/schema/form";
import { InvoiceModel } from "@model/invoice";
import { InvoiceView } from "@view/invoice";
import { CreateInvoice, UpdateInvoice } from "@/data/schema/invoice";
import createInvoice from "@/(controller)/invoices/actions/create";
import updateInvoice from "@/(controller)/invoices/actions/update";
import cancelInvoice from "@/(controller)/invoices/actions/cancel";
```

---

### 4. Routing Layer (`app/(controller)`)

_Next.js App Router (File System Routing)._

#### [customers/page.tsx](<file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/(controller)/customers/page.tsx>)

```typescript
import { Suspense } from "react";
import { CustomerController } from "@controller/customer";
```

#### [dashboard/page.tsx](<file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/(controller)/dashboard/page.tsx>)

```typescript
import { Suspense } from "react";
import { Card } from "@/graphics/card/card";
import { InvoiceController } from "@controller/invoice";
import { RevenueController } from "@controller/revenue";
import { DashboardModel } from "@model/dashboard";
```

---

### 5. Graphics Layer (`app/graphics`)

_Manifestation (UI). Pure React Components & Display Schemas._

#### [form/customer.ts](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/graphics/form/customer.ts)

```typescript
import type { Customer } from "@/data/schema/customer";
import type { CustomerFormShape } from "@graphics/schema/customer";
import { CustomerFormShapeSchema } from "@graphics/schema/customer";
import { Form } from "@graphics/form/form";
```

---

### 6. Data Layer (`app/data`)

_Schema / Storage. Zod Definitions & Prisma._

#### [schema/customer.ts](file:///home/pat/VSCode/new-organon/model/examples/dashboard/app/data/schema/customer.ts)

```typescript
import { z } from "zod";
import { BaseSchema, BaseStateSchema } from "./base";
```

## Pattern Analysis

- **Absolute Imports**: Utilizes `@model/*`, `@view/*`, `@controller/*`, `@data/*`, `@graphics/*` aliases strictly.
- **Minimal Dependencies**: Files typically import < 10 modules.
- **Strict Separation**:
  - `Model` only imports `data` and `schema`.
  - `View` imports `model` (types), `graphics` (components), and `schema`.
  - `Controller` imports `model`, `view`, and actions.
  - `(Controller)` only imports `Controller`.
