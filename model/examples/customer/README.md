# Customer Example - Semantic MVC with Hydration

This is the primary customer example demonstrating the **Semantic MVC** pattern with **SemanticHydrator**.

## Architecture

```
DataModel (SDSL) → SemanticHydrator → FormModel → ReactView → ReactController
```

## Key Components

### 1. Semantic Data Model (`customer-model.ts`)
Defines the semantic layer using SDSL (Semantic Data Science Language):
- **Measures**: `totalRevenue`, `averageInvoice`, `count`
- **Dimensions**: `region`, `signupMonth`
- **Joins**: `invoices` (left join to InvoiceModel)

### 2. Data Service (`customer-data.service.ts`)
Implements `SemanticDataService`:
- Builds datasets from mock data
- Executes via `PolarsExecutionEngine`
- Returns `SemanticResult` with rows, plan, and metadata

### 3. Controller (`customer-controller.ts`)
Extends `ReactController`:
- Uses `SemanticHydrator` to load customer profiles
- Handles business logic (submit, delete, getInvoices)
- Renders via Radix adapter with `HydratorSnapshot`

### 4. View (`customer-view.tsx`)
Extends `ReactView`:
- Renders `DisplayDocument` with metrics, collections, and form fields
- Customizes layout (KPI cards, invoice tables, profile sections)

### 5. Domain Types (`customer.ts`)
- Zod schemas for validation
- FormShape definitions
- TypeScript types

## Usage

```typescript
import { CustomerController } from './customer-controller';

// Create controller
const controller = new CustomerController('view');

// Load customer profile with semantic hydration
const snapshot = await controller.loadCustomerProfile('cust_100');

// Render Radix dashboard
const { document, element } = controller.renderRadixDashboard();
```

## Features

- ✅ **Semantic Queries**: Query data using measures and dimensions
- ✅ **Metrics**: Automatic KPI calculation (totalRevenue, averageInvoice, etc.)
- ✅ **Collections**: Related data arrays (invoices)
- ✅ **Hydration**: Bridge semantic results to form state
- ✅ **Radix UI**: Modern component rendering
- ✅ **Execution Plans**: Observability for debugging

## Running the Example

```bash
# Run the runtime demo
npm run example:customer

# Or import and use in your code
import { runDemo } from './examples/customer';
await runDemo();
```

## Related Documentation

- [Architecture Diagram](../../doc/architecture-diagram.md)
- [Semantic Hydrator Design](../../doc/semantic-hydrator.md)
- [API Comparison](../../doc/api-comparison.md)

