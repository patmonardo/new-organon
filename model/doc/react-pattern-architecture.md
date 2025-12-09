# React Pattern Architecture: Model → Controller → View

## Current React Pattern

### The Perfect Fit

**Model:**
- Provides data (not a viewer)
- Offers special interfaces (fast Find methods)
- Not aware of Views
- **Model is not a Viewer**

**Controller:**
- Stands beneath Model and View
- More aware of Views than Models
- Gets perfect data from Model for View
- Orchestrates Model + View

**View:**
- Translates data to JSX via "Display" method
- Receives perfect data from Controller
- Not aware of Model directly

### Flow

```
Model (Data Provider)
    ↓ provides data
Controller (Orchestrator)
    ├─ Gets perfect data from Model
    ├─ Aware of View
    └─ Passes data to View
        ↓
View (Display Translator)
    ├─ Receives data from Controller
    ├─ Translates via "Display" method
    └─ Returns JSX
        ↓
Controller (Displays JSX)
    └─ Returns JSX to page
```

### Example

```typescript
// Model - provides data
export class CustomerModel {
  static async findById(id: string) {
    // Fast Find method
    return await db.customer.findUnique({ where: { id } });
  }
}

// Controller - orchestrates
export class CustomerController {
  static async editForm(id: string) {
    // Gets perfect data from Model
    const customer = await CustomerModel.findById(id);
    
    // Aware of View, passes data
    const view = new CustomerView();
    const result = await view.display("edit", "jsx", {
      data: customer, // Perfect data for View
    });
    
    // Controller displays JSX
    return <>{result.data}</>;
  }
}

// View - translates to JSX
export class CustomerView {
  async display(mode: string, format: string, options: any) {
    // Receives perfect data from Controller
    const { data } = options;
    
    // Translates via Display method
    return {
      status: "success",
      data: <CustomerForm data={data} />, // JSX
    };
  }
}
```

## Key Principles

### 1. Model is Not a Viewer

**Model:**
- Provides data
- Offers special interfaces
- Not aware of Views
- **Not a Viewer**

### 2. Controller is Aware of Views

**Controller:**
- More aware of Views than Models
- Gets perfect data from Model
- Passes perfect data to View
- Orchestrates the flow

### 3. View Translates to JSX

**View:**
- Receives perfect data from Controller
- Translates via "Display" method
- Returns JSX
- Not aware of Model directly

## Malloy UI: Speculative Bubble

### Initial Approach: Mirror React Pattern

**For first Malloy seed, mirror React pattern:**

```
MalloyModel (Data Provider)
    ↓ provides query results
MalloyController (Orchestrator)
    ├─ Gets perfect data from MalloyModel
    ├─ Aware of MalloyView
    └─ Passes data to MalloyView
        ↓
MalloyView (Display Translator)
    ├─ Receives data from MalloyController
    ├─ Translates via "Display" method
    └─ Returns JSX (Malloy components)
        ↓
MalloyController (Displays JSX)
    └─ Returns JSX to page
```

### Speculative Structure

```typescript
// MalloyModel - provides query results
export class MalloyModel {
  async executeView(viewSpec: ViewSpec) {
    // Execute query, return results
    const view = this.model.viewFromSpec(viewSpec);
    return await engine.execute(view);
  }
}

// MalloyController - orchestrates
export class MalloyController {
  async renderView(viewSpec: ViewSpec) {
    // Gets perfect data from MalloyModel
    const result = await this.malloyModel.executeView(viewSpec);
    
    // Aware of MalloyView, passes data
    const view = new MalloyView();
    const jsx = await view.display("view", "jsx", {
      viewSpec,
      result, // Perfect data for View
    });
    
    // Controller displays JSX
    return jsx;
  }
}

// MalloyView - translates to JSX
export class MalloyView {
  async display(mode: string, format: string, options: any) {
    // Receives perfect data from Controller
    const { viewSpec, result } = options;
    
    // Translates via Display method (Malloy components)
    return (
      <MalloyViewRenderer 
        view={viewSpec} 
        result={result} 
      />
    );
  }
}
```

## Coevolution Path

### Phase 1: Mirror React Pattern

**Malloy UI mirrors React pattern:**
- MalloyModel → MalloyController → MalloyView
- Same flow as React
- MalloyView uses Malloy components

### Phase 2: Take Inspiration

**Malloy UI evolves:**
- May discover better patterns
- May simplify flow
- **Take inspiration for React UI**

### Phase 3: Coevolution

**Both evolve:**
- React UI learns from Malloy UI
- Malloy UI learns from React UI
- Both improve together

## Implementation: Speculative Bubble

### File Structure

```
model/src/ui/malloy/
├── model/
│   └── malloy-model.ts      # Data provider (query execution)
├── controller/
│   └── malloy-controller.ts # Orchestrator
├── view/
│   └── malloy-view.ts       # Display translator
└── components/
    ├── view-renderer.tsx    # MalloyViewRenderer
    ├── dashboard-renderer.tsx
    └── chart-renderer.tsx
```

### MalloyModel (Data Provider)

```typescript
export class MalloyModel {
  constructor(
    private dataModel: DataModel,
    private engine: ExecutionEngine
  ) {}

  async executeView(viewSpec: ViewSpec): Promise<QueryResult> {
    // Execute query, return perfect data
    const view = this.dataModel.viewFromSpec(viewSpec);
    return await this.engine.execute(view);
  }
}
```

### MalloyController (Orchestrator)

```typescript
export class MalloyController {
  constructor(
    private malloyModel: MalloyModel,
    private malloyView: MalloyView
  ) {}

  async renderView(viewSpec: ViewSpec): Promise<React.ReactNode> {
    // Gets perfect data from Model
    const result = await this.malloyModel.executeView(viewSpec);
    
    // Aware of View, passes perfect data
    const jsx = await this.malloyView.display("view", "jsx", {
      viewSpec,
      result,
    });
    
    return jsx;
  }
}
```

### MalloyView (Display Translator)

```typescript
export class MalloyView {
  async display(mode: string, format: string, options: any) {
    const { viewSpec, result } = options;
    
    // Translates via Display method (Malloy components)
    return (
      <MalloyViewRenderer 
        view={viewSpec} 
        result={result} 
      />
    );
  }
}
```

## Key Points

### 1. React Pattern is Perfect

**Current pattern works:**
- Model → Controller → View
- Controller orchestrates
- View translates to JSX
- **Keep this pattern**

### 2. Malloy UI Mirrors Initially

**For first seed:**
- Mirror React pattern
- Same flow: Model → Controller → View
- MalloyView uses Malloy components
- **Not clear how it will evolve**

### 3. Speculative Bubble

**Explore freely:**
- Build Malloy UI structure
- Take inspiration for React UI
- See how they coevolve
- **Don't force - let it evolve**

## Next Steps

1. **Build Malloy UI structure** (speculative bubble)
2. **Mirror React pattern** initially
3. **See how it evolves**
4. **Take inspiration** for React UI
5. **Let coevolution happen** naturally

---

**Key Insight**: React pattern is perfect (Model → Controller → View). For first Malloy seed, mirror this pattern. Build speculative bubble, see how it evolves, take inspiration for React UI. Let coevolution happen naturally.

