# Client Hierarchy: GDS → Logic → MVC

## Architecture Overview

```
GDS (Kernel)
    ↑
GDSL (Client Language)
    ↑
Logic Form Processor (Primary GDS Client, UserLand)
    ↑
MVC (Primary Client of Given Form Processor, FormApp SDK)
```

## Layer 1: GDS (Kernel)

**The foundation:**
- ML operations
- Logical Forms
- Kernel-level operations
- Rust-based (OpenCypher)

## Layer 2: GDSL (Client Language)

**GDSL is the language** used to communicate with GDS:
- OpenCypher queries
- Graph operations
- ML operations
- Kernel access

## Layer 3: Logic Form Processor (Primary GDS Client, UserLand)

**Logic Form Processor in UserLand:**
- Uses **GDSL as a Client to GDS**
- **Primary GDS Client** - the main interface to GDS
- Operates in UserLand (not kernel)
- Processes **Given Forms** (logical forms)
- Uses GDSL to communicate with GDS kernel
- **Given Form Processor** - processes forms given to it

### Architecture

```
Logic Form Processor (UserLand)
    ↓
Uses GDSL (Client Language)
    ↓
Communicates with GDS (Kernel)
    ↓
Returns Logical Forms
```

**Logic Form Processor is the bridge** between UserLand and GDS Kernel.

## Layer 4: MVC (Primary Client of Given Form Processor, FormApp SDK)

**MVC is special:**
- **Primary Client** of the Given Form Processor (Logic)
- Runs against **special Data Models and Forms**
- Almost part of the **Logic SDK**
- **MVC SDSL is a FormApp SDK** - not just runtime client

**MVC Components:**
- **Form-model, view, controller** - Runtime Client (part of Logic SDK)
- **MVC SDSL** - FormApp SDK (special data models, forms, middleware)

### MVC as FormApp SDK

**MVC SDSL = FormApp SDK:**
- FormApp development toolkit
- Special data models
- Form processing
- MVC patterns
- Business logic

### MVC Components

**Part of Logic SDK:**
- **Form-model, view, controller** - Runtime Client
- **MVC SDSL** - FormApp SDK

**MVC is almost part of Logic SDK** - it's the primary client that uses the Given Form Processor.

## Logic InfoStore (Should Drive, Mocked for Now)

**Logic InfoStore should be driving the show:**
- Provides the data/context
- Drives the entire flow
- **Can mock for now** - but ideally drives everything
- Eventually integrates with Logic Form Processor

### Architecture with InfoStore

```
Logic InfoStore (Should Drive, Mocked for Now)
    ↓
Drives (ideally)
    ↓
Logic Form Processor (Primary GDS Client, UserLand)
    ↓
Uses GDSL → GDS (Kernel)
    ↓
MVC (Primary Client of Given Form Processor)
    ↓
FormApp SDK (MVC SDSL)
    ↓
Special Data Models and Forms
```

## Complete Hierarchy

```
┌─────────────────────────────────────────────────────────┐
│                    GDS (Kernel)                         │
│              ML, Logical Forms, Rust                     │
└───────────────────────┬─────────────────────────────────┘
                        ↑
                        │ GDSL (Client Language)
                        │ OpenCypher queries
                        │
┌───────────────────────▼─────────────────────────────────┐
│         Logic Form Processor (UserLand)                 │
│         Primary GDS Client                               │
│         Uses GDSL to communicate with GDS                 │
└───────────────────────┬─────────────────────────────────┘
                        ↑
                        │ Given Form Processor
                        │
┌───────────────────────▼─────────────────────────────────┐
│                    MVC (FormApp SDK)                     │
│         Primary Client of Given Form Processor           │
│         Almost part of Logic SDK                         │
│                                                           │
│  ┌──────────────┐              ┌──────────────┐         │
│  │ Runtime     │              │ FormApp SDK  │         │
│  │ Client      │              │ (MVC SDSL)   │         │
│  │             │              │              │         │
│  │ FormModel   │              │ Data Models  │         │
│  │ FormView    │              │ Forms        │         │
│  │ FormController│            │ Middleware   │         │
│  └──────────────┘              └──────────────┘         │
└─────────────────────────────────────────────────────────┘
```

## Key Relationships

### 1. Logic Form Processor → GDS

**Logic Form Processor uses GDSL as Client to GDS:**

```typescript
// Logic Form Processor
class LogicFormProcessor {
  async processForm(form: LogicalForm): Promise<FormResult> {
    // Use GDSL to communicate with GDS
    const gdslQuery = this.translateToGDSL(form);
    const result = await gds.execute(gdslQuery);
    return this.translateFromGDS(result);
  }
}
```

**Logic Form Processor is the Primary GDS Client** - it's the main interface between UserLand and GDS Kernel.

### 2. MVC → Logic Form Processor

**MVC is Primary Client of Given Form Processor:**

```typescript
// MVC uses Given Form Processor (Logic)
class MVCController {
  async processForm(form: FormShape): Promise<DisplayDocument> {
    // Use Given Form Processor (Logic)
    // MVC runs against special Data Models and Forms
    const logicalForm = this.translateToLogicalForm(form);
    const result = await logicFormProcessor.process(logicalForm);
    return this.translateToDisplay(result);
  }
}
```

**MVC is almost part of Logic SDK** - it's the primary client that uses the Given Form Processor.

**MVC runs against special Data Models and Forms** - not generic, but specialized for FormApp development.

### 3. Logic InfoStore

**Should be driving the show (but mocked for now):**

```typescript
// Logic InfoStore (should drive, but mocked for now)
class LogicInfoStore {
  async getContext(entityId: string): Promise<Context> {
    // Provides data/context
    // Should drive the entire flow
    // Can mock for now, but ideally drives everything
    return context;
  }
  
  async driveFormProcessor(form: FormShape): Promise<void> {
    // Ideally drives Logic Form Processor
    // Provides context and data
    // Mocked for now
  }
}
```

## MVC as FormApp SDK

**MVC SDSL is a FormApp SDK:**

```typescript
// FormApp SDK - MVC SDSL
import { defineModel, ReactController, ReactView } from '@model/sdsl';

// Define data model
export const CustomerModel = defineModel({
  measures: { totalRevenue: sum('invoices.amount') },
  dimensions: { region: 'region' },
  joins: { invoices: { model: InvoiceModel } }
});

// Create FormApp
export class CustomerFormApp extends ReactController {
  // FormApp logic
}
```

**MVC provides:**
- FormApp development toolkit
- Special data models
- Form processing
- MVC patterns
- Business logic

## Runtime Client vs FormApp SDK

### Runtime Client (Form-model, view, controller)

**Part of Logic SDK:**
- `FormModel` - Form state management
- `FormView` - Form rendering
- `FormController` - Form logic
- Runtime execution

### FormApp SDK (MVC SDSL)

**FormApp development:**
- `defineModel` - Data modeling
- `ReactController` - React integration
- `ReactView` - React rendering
- `SemanticHydrator` - Data hydration
- FormApp patterns

## Complete Flow

```
Logic InfoStore (Should Drive, Mocked for Now)
    ↓
Drives (ideally)
    ↓
Logic Form Processor (Primary GDS Client, UserLand)
    ├─→ Uses GDSL as Client to GDS
    ├─→ Processes Given Forms
    └─→ Returns Logical Forms
    ↓
MVC (Primary Client of Given Form Processor)
    ├─→ Almost part of Logic SDK
    ├─→ Uses Given Form Processor
    └─→ Runs against Special Data Models and Forms
    ↓
FormApp SDK (MVC SDSL)
    ├─→ Form-model, view, controller (Runtime Client)
    └─→ MVC SDSL (FormApp SDK)
    ↓
FormApp
```

## Key Insights

1. **Logic Form Processor** - Primary GDS Client (UserLand), uses GDSL as Client to GDS
2. **MVC** - Primary Client of Given Form Processor, almost part of Logic SDK
3. **MVC SDSL** - FormApp SDK (special data models, forms, middleware)
4. **Form-model, view, controller** - Runtime Client (part of Logic SDK)
5. **Logic InfoStore** - Should drive the show, but mocked for now
6. **Clear hierarchy** - GDS → GDSL → Logic Form Processor → MVC
7. **MVC runs against special Data Models and Forms** - not generic, but specialized

## Next Steps

- [ ] Document Logic Form Processor architecture
- [ ] Clarify Given Form Processor interface
- [ ] Design Logic InfoStore integration
- [ ] Document MVC as FormApp SDK
- [ ] Map Runtime Client vs FormApp SDK boundaries

