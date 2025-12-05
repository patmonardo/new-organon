# MVC as Part of Logic SDK

## Overview

**MVC is almost part of the Logic SDK** - it's the primary client that uses the Given Form Processor.

**MVC has two parts:**
1. **Runtime Client** - Form-model, view, controller (part of Logic SDK)
2. **FormApp SDK** - MVC SDSL (special data models, forms, middleware)

## MVC Components

### Runtime Client (Part of Logic SDK)

**Form-model, view, controller:**
- `FormModel` - Form state management
- `FormView` - Form rendering
- `FormController` - Form logic
- Runtime execution
- **Part of Logic SDK** - core runtime components

### FormApp SDK (MVC SDSL)

**MVC SDSL is a FormApp SDK:**
- `defineModel` - Data modeling (Malloy-inspired)
- `ReactController` - React integration
- `ReactView` - React rendering
- `SemanticHydrator` - Data hydration
- Special Data Models and Forms
- Middleware
- Domain Calculators

## MVC as Primary Client

**MVC is Primary Client of Given Form Processor:**

```
Logic Form Processor (Given Form Processor)
    ↑
Primary Client
    ↑
MVC (FormApp SDK)
    ├─→ Runtime Client (FormModel, FormView, FormController)
    └─→ FormApp SDK (MVC SDSL)
```

**MVC runs against special Data Models and Forms** - not generic, but specialized for FormApp development.

## Special Data Models and Forms

**MVC works with specialized models:**

```typescript
// Special Data Model (MVC SDSL)
export const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  measures: {
    totalRevenue: sum('invoices.amount'),
    averageInvoice: avg('invoices.amount'),
  },
  dimensions: {
    region: 'region',
    signupMonth: dimension('createdAt', 'month'),
  },
  joins: {
    invoices: {
      model: InvoiceModel,
      on: 'customers.id = invoices.customerId',
      type: 'left',
    },
  },
});

// Special Form (MVC SDSL)
export const CustomerShape: FormShape = {
  id: 'customer-form',
  name: 'Customer',
  fields: [
    { id: 'name', type: 'text', label: 'Name', required: true },
    { id: 'email', type: 'email', label: 'Email', required: true },
    // ...
  ],
  layout: {
    sections: [
      { id: 'basic-info', title: 'Basic Information', fields: ['name', 'email'] },
    ],
  },
};
```

**These are special** - not generic forms, but specialized for FormApp development.

## Logic SDK Structure

```
Logic SDK
├── Runtime Client
│   ├── FormModel
│   ├── FormView
│   └── FormController
│
├── Given Form Processor
│   └── Processes Given Forms
│
└── MVC (Almost Part of Logic SDK)
    ├── Runtime Client (FormModel, FormView, FormController)
    └── FormApp SDK (MVC SDSL)
        ├── Special Data Models
        ├── Special Forms
        ├── Middleware
        └── Domain Calculators
```

## Logic InfoStore Integration

**Logic InfoStore should be driving the show:**

```
Logic InfoStore (Should Drive, Mocked for Now)
    ↓
Drives (ideally)
    ↓
Logic Form Processor (Given Form Processor)
    ↓
MVC (Primary Client)
    ├─→ Runtime Client
    └─→ FormApp SDK
```

**Logic InfoStore provides:**
- Data/context
- Drives the entire flow
- Can mock for now
- Eventually integrates with Logic Form Processor

## Key Principles

1. **MVC is almost part of Logic SDK** - Primary client of Given Form Processor
2. **Runtime Client** - Form-model, view, controller (part of Logic SDK)
3. **FormApp SDK** - MVC SDSL (special data models, forms, middleware)
4. **Special Data Models and Forms** - Not generic, but specialized
5. **Logic InfoStore** - Should drive, but mocked for now

## Next Steps

- [ ] Clarify Runtime Client vs FormApp SDK boundaries
- [ ] Document special Data Models and Forms
- [ ] Design Logic InfoStore integration
- [ ] Map MVC → Logic Form Processor interface
- [ ] Document MVC as part of Logic SDK

