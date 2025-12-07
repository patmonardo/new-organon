# FORM SDK MVC Architecture

## Overview

The **FORM SDK MVC** is the generic foundation that React and Radix clients build upon. It provides:

- **form-model.ts** — Generic Model (data, validation, persistence)
- **form-view.ts** — Generic View (display transformation, formatting)
- **form-controller.ts** — Generic Controller (orchestration, action handling)

## Architecture

```
Generic FORM SDK MVC
    ├─→ React Client
    │   ├── react-controller.ts (extends form-controller)
    │   ├── react-view.tsx (extends form-view)
    │   └── react-adapter.tsx
    │
    └─→ Radix Client
        ├── radix-controller.tsx (extends form-controller)
        └── radix-adapter.tsx
```

## FormModel

**Role:** State : Structure dyad

```typescript
export abstract class FormModel<T extends FormShape = FormShape> {
  protected _shape: T;
  protected _dirty: boolean = false;

  constructor(shape: T) {
    this._shape = shape;
  }

  get shape(): T { return this._shape; }
  get fields(): FormField[] { return this._shape.fields || []; }
  get dirty(): boolean { return this._dirty; }

  abstract validate(): boolean;
  abstract create(data: unknown): Promise<OperationResult>;
  abstract read(id: string): Promise<OperationResult>;
  abstract update(id: string, data: unknown): Promise<OperationResult>;
  abstract delete(id: string): Promise<OperationResult>;
}
```

**Responsibilities:**
- Hold current state (data)
- Define structure (schema)
- Perform data operations (CRUD)
- Validate data integrity

## FormView

**Role:** Representation : Perspective dyad

```typescript
export abstract class FormView<T extends FormShape = FormShape> {
  protected _model: FormModel<T>;
  protected _mode: FormMode;

  constructor(model: FormModel<T>, mode: FormMode = 'view') {
    this._model = model;
    this._mode = mode;
  }

  abstract render(): DisplayDocument;
}
```

**Responsibilities:**
- Transform data into display representation
- Apply perspective (filtering, formatting)
- Produce framework-agnostic DisplayDocument

## FormController

**Role:** Action : Rule dyad

```typescript
export abstract class FormController<T extends FormShape = FormShape> {
  protected _model: FormModel<T>;
  protected _view: FormView<T>;
  protected _mode: FormMode;
  protected _actions: Map<string, ControllerAction> = new Map();
  protected _rules: Map<string, ControllerRule> = new Map();

  constructor(model: FormModel<T>, view: FormView<T>, mode: FormMode = 'view') {
    this._model = model;
    this._view = view;
    this.setMode(mode);
    this.registerDefaultActions();
  }

  display(): DisplayDocument {
    return this._view.render();
  }

  async executeAction(actionId: string, data?: unknown): Promise<OperationResult>;
}
```

**Responsibilities:**
- Orchestrate Model → View pipeline
- Handle user actions
- Apply business rules
- Produce output for adapters

## DisplayDocument

The **Generic Display Language** that flows to adapters:

```typescript
interface DisplayDocument {
  id: string;
  title?: string;
  layout: DisplayLayout;
  sections?: DisplaySection[];
  actions?: DisplayAction[];
  meta?: Record<string, unknown>;
}

interface DisplayElement {
  type: string;
  id: string;
  props?: Record<string, unknown>;
  children?: DisplayElement[];
}
```

## Usage

```typescript
// Create MVC stack
const model = new CustomerModel(customerShape);
const view = new CustomerView(model, 'edit');
const controller = new CustomerController(model, view, 'edit');

// Get display document
const doc = controller.display();

// Render with adapter
const jsx = reactAdapter.render(doc, { handler: controller.createHandler() });
```

## Extension Pattern

### React Client

```typescript
export class ReactController extends FormController {
  createSubmitAction(): ServerAction {
    return async (formData: FormData) => {
      // Server Action implementation
    };
  }
}

export class ReactView extends FormView {
  render(): DisplayDocument {
    // React-specific rendering logic
  }
}
```

### Radix Client

```typescript
export class RadixController extends FormController {
  // Radix-specific state management
}
```

## Benefits

1. **Separation** — Model, View, Controller have clear responsibilities
2. **Extensibility** — Clients extend generic base classes
3. **Adapter-agnostic** — DisplayDocument works with any adapter
4. **Testable** — Each component can be tested independently

