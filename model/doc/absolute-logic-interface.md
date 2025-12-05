# Absolute Logic Interface: TypeScript to Rust GDS

## The Architecture

**Don't target Rust GDS directly** - Use the **Absolute Logic** interface layer.

```
TypeScript World (Malloy/MVC)
    ↓
Absolute Logic Interface (TypeScript)
    ↓
Rust GDS Kernel
```

## Why Absolute Logic?

### 1. Clean Separation
- **TypeScript world** - Malloy, MVC, high-level semantics
- **Absolute Logic** - TypeScript interface to Rust
- **Rust GDS** - Graph algorithms, ML, low-level execution

### 2. Type-Safe Boundary
- **Absolute Logic** provides TypeScript types
- **FFI boundary** managed in one place
- **No direct Rust exposure** to Malloy/MVC

### 3. Unified Interface
- **Single entry point** to Rust GDS
- **Consistent API** for all TypeScript clients
- **Versioned protocol** between worlds

## Absolute Logic as Interface Layer

### What It Provides

**TypeScript API:**
```typescript
// Absolute Logic Interface
interface AbsoluteLogicInterface {
  // Graph operations
  graph: {
    query(cypher: CypherQuery): Promise<GraphResult>;
    createNode(node: NodePattern): Promise<NodeId>;
    createRelationship(rel: RelationshipPattern): Promise<RelId>;
  };
  
  // ML operations
  ml: {
    extractFeatures(spec: FeatureSpec): Promise<FeatureVector>;
    generateEmbedding(input: EmbeddingInput): Promise<Embedding>;
    predict(model: ModelId, features: Features): Promise<Prediction>;
  };
  
  // Ontology operations
  ontology: {
    validateShape(shape: SHACLShape, data: Data): Promise<ValidationResult>;
    inferRules(rules: SPINRule[], data: Data): Promise<InferredTriples>;
  };
  
  // GDS operations
  gds: {
    runAlgorithm(algo: AlgorithmSpec): Promise<AlgorithmResult>;
    computeMetrics(spec: MetricsSpec): Promise<Metrics>;
  };
}
```

**Rust GDS exposes:**
```rust
// Rust GDS Kernel (internal)
// - Not directly called from TypeScript
// - Only through Absolute Logic FFI
```

## Layered Architecture

### Layer 1: Malloy/MVC (TypeScript)
**High-level semantic language:**
- Malloy models and views
- MVC forms and controllers
- Ontology definitions (OWL/SHACL/SPIN)
- ML features and models

**Does NOT:**
- Call Rust directly
- Manage FFI
- Handle low-level execution

### Layer 2: Absolute Logic (TypeScript)
**TypeScript interface to Rust GDS:**
- Translates high-level semantics to Rust calls
- Manages FFI boundary
- Provides type-safe API
- Handles serialization/deserialization

**Responsibilities:**
- API design
- Protocol versioning
- Error handling
- Type safety

### Layer 3: Rust GDS Kernel (Rust)
**Low-level execution:**
- Graph algorithms
- GNN operations
- ML pipelines
- OpenCypher execution

**Exposes:**
- FFI interface (C ABI)
- Called by Absolute Logic only
- Internal implementation details hidden

## Interface Design

### From Malloy to Rust (via Absolute Logic)

```typescript
// 1. Malloy/MVC (High-level)
const customerModel = defineModel({
  name: 'Customer',
  measures: {
    totalRevenue: { aggregate: 'sum', field: 'revenue' },
  },
  features: {
    revenueNormalized: { transform: 'normalize', field: 'totalRevenue' },
  },
  embedding: {
    customerVec: { fields: ['name', 'region'] },
  },
});

// 2. Absolute Logic translates to Rust GDS calls
// (Internal to Absolute Logic - user doesn't see this)
const result = await absoluteLogic.ml.extractFeatures({
  source: customerModel,
  features: ['revenueNormalized'],
});

const embedding = await absoluteLogic.ml.generateEmbedding({
  fields: customerModel.embedding.customerVec.fields,
  data: customerData,
});

// 3. Rust GDS executes (FFI boundary)
// - Feature extraction
// - Embedding generation
// - Returns results
```

### Unified IR Translation

**Malloy IR → Absolute Logic → Rust GDS:**

```typescript
// Unified IR (TypeScript)
interface UnifiedIR {
  graph: OpenCypherIR;
  analytics: MalloyIR;
  ml: MLIR;
  ontology: OntologyIR;
}

// Absolute Logic Interface
class AbsoluteLogicInterface {
  // Translates unified IR to Rust GDS operations
  async executeUnifiedIR(ir: UnifiedIR): Promise<ExecutionResult> {
    // 1. Extract graph operations → call Rust GDS graph API
    const graphResult = await this.executeGraph(ir.graph);
    
    // 2. Extract analytics → call Rust GDS analytics API
    const analyticsResult = await this.executeAnalytics(ir.analytics);
    
    // 3. Extract ML → call Rust GDS ML API
    const mlResult = await this.executeML(ir.ml);
    
    // 4. Extract ontology → call Rust GDS ontology API
    const ontologyResult = await this.executeOntology(ir.ontology);
    
    // 5. Combine results
    return {
      graph: graphResult,
      analytics: analyticsResult,
      ml: mlResult,
      ontology: ontologyResult,
    };
  }
  
  private async executeGraph(ir: OpenCypherIR): Promise<GraphResult> {
    // FFI call to Rust GDS
    return await rustGDS.graph.execute(ir);
  }
  
  // ... similar for analytics, ml, ontology
}
```

## Benefits of Absolute Logic Interface

### 1. Clean Separation
**TypeScript stays in TypeScript:**
- Malloy/MVC code is pure TypeScript
- No Rust dependencies
- No FFI concerns

**Rust stays in Rust:**
- GDS Kernel is pure Rust
- No TypeScript dependencies
- Optimized execution

### 2. Type Safety
**Absolute Logic provides:**
- TypeScript types for all operations
- Compile-time type checking
- IDE support (autocomplete, refactoring)

### 3. Protocol Versioning
**Absolute Logic manages:**
- API versions
- Backward compatibility
- Migration paths

### 4. Error Handling
**Absolute Logic handles:**
- FFI errors
- Rust panics
- Serialization errors
- Network errors (if distributed)

### 5. Testing
**Absolute Logic enables:**
- Mock Rust GDS for testing
- Unit tests without Rust dependency
- Integration tests with real Rust GDS

## Implementation Strategy

### Phase 1: Define Interface
```typescript
// @logic/src/absolute/gds-interface.ts
export interface GDSInterface {
  graph: GraphOperations;
  ml: MLOperations;
  ontology: OntologyOperations;
  analytics: AnalyticsOperations;
}
```

### Phase 2: Implement TypeScript Side
```typescript
// @logic/src/absolute/gds-client.ts
export class GDSClient implements GDSInterface {
  // FFI calls to Rust GDS
  // Serialization/deserialization
  // Error handling
}
```

### Phase 3: Implement Rust Side
```rust
// @gds/src/ffi/interface.rs
#[no_mangle]
pub extern "C" fn gds_graph_execute(ir: *const u8, len: usize) -> *mut u8 {
  // Deserialize IR
  // Execute in GDS Kernel
  // Serialize result
}
```

### Phase 4: Malloy Integration
```typescript
// @model/src/data/gds-executor.ts
import { absoluteLogic } from '@logic/absolute';

export class GDSExecutor {
  async executeMalloyIR(ir: MalloyIR): Promise<Result> {
    // Translate Malloy IR to GDS operations
    // Call through Absolute Logic
    return await absoluteLogic.executeUnifiedIR(ir);
  }
}
```

## Absolute Logic Protocol

### Message Format
```typescript
interface GDSMessage {
  version: string;
  operation: string;
  payload: unknown;
}

interface GDSResponse {
  version: string;
  status: 'success' | 'error';
  result?: unknown;
  error?: string;
}
```

### Serialization
- **JSON** for simple data
- **MessagePack** for binary data
- **Arrow IPC** for DataFrame exchange
- **Protobuf** for complex structures

## The Complete Stack

```
┌─────────────────────────────────────────┐
│ Malloy/MVC (TypeScript)                 │
│ - Unified semantic language             │
│ - Models, views, queries                │
│ - High-level API                        │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ Absolute Logic (TypeScript)             │
│ - TypeScript interface to Rust GDS      │
│ - Type-safe API                         │
│ - FFI boundary management               │
│ - Protocol versioning                   │
└─────────────────────────────────────────┘
                  ↓ FFI
┌─────────────────────────────────────────┐
│ Rust GDS Kernel (Rust)                  │
│ - Graph algorithms                      │
│ - ML pipelines                          │
│ - GNN operations                        │
│ - OpenCypher execution                  │
└─────────────────────────────────────────┘
```

## Key Insights

1. **Don't target Rust directly** - Always go through Absolute Logic
2. **Absolute Logic is the boundary** - TypeScript ↔ Rust
3. **Type-safe interface** - TypeScript types all the way
4. **Protocol versioning** - Clean evolution
5. **Unified IR translation** - Absolute Logic handles complexity

## Next Steps

- [ ] Design Absolute Logic interface API
- [ ] Implement TypeScript client
- [ ] Implement Rust FFI server
- [ ] Define serialization protocol
- [ ] Integrate with Malloy executor
- [ ] Add error handling
- [ ] Write integration tests

**The Absolute Logic layer is the key** - clean TypeScript interface to Rust GDS power.

