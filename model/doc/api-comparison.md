# API Comparison: Two Customer Examples

## Overview

There are two customer examples representing two different API approaches:

1. **Semantic MVC** (`examples/customer/`) - EDA-focused, semantic queries, metrics - **PRIMARY**
2. **Traditional MVC** (`examples/archive/customer-traditional/`) - Form-focused, transport-agnostic - **ARCHIVED**

## Architecture Comparison

### Traditional MVC (`examples/archive/customer-traditional/`) - ARCHIVED

**Base Classes:**
- `FormController` (from `src/sdsl/form-controller`)
- `SimpleFormModel` (from `src/sdsl/form-model`)
- `FormView` (from `src/sdsl/form-view`)

**Pattern:**
```
Schema → Model (CRUD) → FormModel → FormView → Controller
```

**Key Features:**
- ✅ Transport-agnostic (tRPC, REST, Server Actions)
- ✅ Simple CRUD operations
- ✅ FactStore integration
- ✅ Static methods for queries
- ✅ Form validation via Zod schemas
- ❌ No semantic layer
- ❌ No metrics/aggregations
- ❌ No EDA capabilities

**Use Case:** Traditional form applications, CRUD interfaces

### Semantic MVC (`examples/customer/`) - PRIMARY

**Base Classes:**
- `ReactController` (from `src/sdsl/react-controller`)
- `FormModel` (from `src/sdsl/form-model`)
- `ReactView` (from `src/sdsl/react-view`)

**Pattern:**
```
DataModel (SDSL) → SemanticHydrator → FormModel → ReactView → ReactController
```

**Key Features:**
- ✅ Semantic data modeling (Malloy-style)
- ✅ Metrics and aggregations
- ✅ Polars execution engine
- ✅ HydratorSnapshot for KPI cards/tables
- ✅ Radix adapter integration
- ✅ EDA toolkit capabilities
- ❌ React-specific (not transport-agnostic)
- ❌ More complex setup

**Use Case:** Data dashboards, analytics forms, EDA applications

## Code Comparison

### Controller

**Traditional MVC:**
```typescript
export class CustomerController {
  private _model: CustomerFormModel;
  private _view: CustomerFormView;
  
  // Static methods for transport layer
  static async create(data: CreateCustomer): Promise<ControllerResult<CustomerData>>
  static async getById(id: string): Promise<ControllerResult<CustomerData>>
  
  // Instance methods for form handling
  getFormDefinition(): FormDefinition
  getDisplayDocument(): DisplayDocument
}
```

**Semantic MVC:**
```typescript
export class CustomerController extends ReactController<FormShape> {
  private dataService: CustomerDataService;
  private hydrator: SemanticHydrator;
  
  // Semantic loading
  async loadCustomerProfile(customerId: string): Promise<HydratorSnapshot>
  
  // Radix rendering
  renderRadixDashboard(): RadixDashboardRender
  
  // Business logic
  async getInvoices(customerId: string)
}
```

### Model

**Traditional MVC:**
```typescript
export class CustomerModel {
  // Static CRUD methods
  static async create(data: CreateCustomer): Promise<OperationResult<CustomerData>>
  static async findById(id: string): Promise<OperationResult<CustomerData>>
  static async findAll(options): Promise<OperationResult<CustomerData[]>>
  
  // Uses FactStore
  private static factStore: FactStoreInterface
}
```

**Semantic MVC:**
```typescript
export const CustomerModel = defineModel({
  name: 'Customer',
  source: 'customers',
  fields: { id, name, email, ... },
  measures: {
    totalRevenue: sum('invoices.amount'),
    averageInvoice: avg('invoices.amount'),
  },
  joins: { invoices: { model: InvoiceModel, on: '...' } }
});

// Used via DataView queries
CustomerModel.view({ filter: { id }, aggregate: ['totalRevenue'] })
```

### View

**Traditional MVC:**
```typescript
export class CustomerFormView {
  render(): DisplayDocument {
    // Simple form fields
    return { title, layout: { type: 'stack', children: fields } }
  }
}
```

**Semantic MVC:**
```typescript
export class CustomerView extends ReactView<FormShape> {
  render(): DisplayDocument {
    // Includes metrics, collections, KPI cards
    return {
      layout: {
        children: [
          { type: 'grid', children: metricElements },
          { type: 'card', children: [invoiceTable] },
          { type: 'card', children: profileFields }
        ]
      }
    }
  }
}
```

### Data Access

**Traditional MVC:**
- Direct CRUD via `CustomerModel.create()`, `findById()`, etc.
- In-memory Map storage (mock)
- FactStore for event tracking

**Semantic MVC:**
- Semantic queries via `CustomerModel.view()`
- `SemanticDataService` executes via Polars
- `SemanticHydrator` bridges to FormModel
- Supports metrics, collections, aggregations

## Decision Matrix

| Feature | Traditional MVC | Semantic MVC |
|---------|----------------|--------------|
| **Transport Agnostic** | ✅ Yes | ❌ React-specific |
| **Simple Forms** | ✅ Excellent | ⚠️ Overkill |
| **Metrics/KPIs** | ❌ No | ✅ Yes |
| **Data Aggregations** | ❌ No | ✅ Yes |
| **EDA Capabilities** | ❌ No | ✅ Yes |
| **Semantic Queries** | ❌ No | ✅ Yes |
| **Complexity** | Low | High |
| **Learning Curve** | Low | High |
| **Use Case** | CRUD apps | Analytics/Dashboards |

## Recommendation

**Choose Semantic MVC** if:
- Building analytics dashboards
- Need metrics and KPIs
- Want EDA capabilities
- Building FormApp (the stated goal)

**Choose Traditional MVC** if:
- Simple CRUD forms
- Need transport-agnostic API
- No analytics requirements

## Hybrid Approach?

Could we combine both?
- Use Semantic MVC as the foundation
- Add transport-agnostic static methods to ReactController
- Make ReactController optional (can use without React)
- Support both simple forms and semantic queries

This would give us:
- ✅ Semantic capabilities when needed
- ✅ Simple forms when not
- ✅ Transport-agnostic API
- ✅ Backward compatibility

