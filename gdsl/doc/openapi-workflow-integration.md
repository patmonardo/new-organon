# OpenAPI → MVC Invocation (Simple, Observable)

> **Note**: Complex workflows are handled by the Task Agent. This document focuses on simple, observable invocation patterns from OpenAPI to MVC apps.

## Philosophy

**Simple, Observable Invocation** - Not Enterprise BPM

We're not building enterprise workflow systems (BPMN, Oracle BPM, Jenkins). We want:
- ✅ Simple operation chaining via OpenAPI components
- ✅ Observable invocations from OpenAPI to MVC apps
- ✅ Lightweight orchestration

**Workflows are the Task Agent's domain** - they handle complex workflow orchestration. GDSL focuses on simple invocation patterns.

## OpenAPI Components for Simple Invocation

OpenAPI 3.1's **components** section provides workflow-like patterns:

### 1. Callbacks (Async Workflows)

```yaml
components:
  callbacks:
    OrderStatusCallback:
      '{$request.body#/callbackUrl}':
        post:
          operationId: onOrderStatusChange
          requestBody:
            $ref: '#/components/schemas/OrderStatus'
```

**Use Case**: Webhook-based workflows, async operations

### 2. Links (Operation References)

```yaml
components:
  links:
    OrderDetails:
      operationId: getOrder
      parameters:
        orderId: '$response.body#/orderId'
    
    CustomerOrders:
      operationId: listOrders
      parameters:
        customerId: '$response.body#/customerId'
```

**Use Case**: Operation chaining, workflow sequences

### 3. OperationRef (Workflow Composition)

```yaml
paths:
  /orders:
    post:
      operationId: createOrder
      responses:
        '201':
          links:
            viewOrder:
              operationRef: '#/paths/~1orders~1{id}/get'
              parameters:
                id: '$response.body#/orderId'
```

**Use Case**: Workflow orchestration, operation dependencies

## Workflow AST from OpenAPI

The **Workflow AST** is the parsed structure of OpenAPI components that form workflows:

```
OpenAPI Spec
    ↓
Parse Components (callbacks, links, operationRef)
    ↓
Workflow AST
    ├─→ Operation Nodes
    ├─→ Link Edges
    ├─→ Callback Hooks
    └─→ Parameter Bindings
```

## Integration with GDSL

### Option 1: OpenAPI → Workflow AST → GDSL

```
OpenAPI Components
    ↓
Extract Workflow Patterns
    ↓
Workflow AST
    ↓
Map to GDSL Procedures
    ↓
Execute via GDS Runtime
```

**Example:**
```typescript
// OpenAPI component
const callback = {
  operationId: 'onOrderStatusChange',
  requestBody: { $ref: '#/components/schemas/OrderStatus' }
};

// Map to GDSL procedure
const gdslProcedure = {
  name: 'onOrderStatusChange',
  input: mapSchemaToGDSL(callback.requestBody),
  execute: async (input) => {
    // Execute GDSL procedure
  }
};
```

### Option 2: GDSL Queries as OpenAPI Operations

```
GDSL Query
    ↓
Expose as OpenAPI Operation
    ↓
Reference via operationRef
    ↓
Create Workflow from References
```

**Example:**
```typescript
// GDSL query
const customerQuery = CustomerModel.view({
  filter: { id: customerId },
  aggregate: ['totalRevenue']
});

// Expose as OpenAPI operation
const openApiOp = {
  operationId: 'getCustomerMetrics',
  responses: {
    '200': {
      schema: mapGDSLResultToSchema(customerQuery)
    }
  }
};

// Reference in workflow
const workflow = {
  steps: [
    { operationRef: '#/operations/getCustomerMetrics' },
    { operationRef: '#/operations/getCustomerOrders' }
  ]
};
```

### Option 3: Workflow AST Orchestrates GDSL

```
Workflow AST (from OpenAPI)
    ├─→ Step 1: GDSL Query → Neo4j
    ├─→ Step 2: GDSL Query → Polars
    └─→ Step 3: GDSL Procedure → GDS Runtime
```

**Example:**
```typescript
const workflowAST = parseOpenAPIComponents(openApiSpec);

// Execute workflow
for (const step of workflowAST.steps) {
  if (step.operationRef) {
    const gdslQuery = mapOperationToGDSL(step.operationRef);
    const result = await executeGDSL(gdslQuery);
    workflowAST.setVariable(step.output, result);
  }
}
```

## Current Workflow Schema

Our `task/src/schema/workflow.ts` defines:
- **Process** - Steps, dependencies, dialectical roles
- **Coordination** - Rules, synchronization
- **Workflow** - Synthesis

This could be **enhanced** with OpenAPI component patterns:

```typescript
interface WorkflowStep {
  id: string;
  type: 'task' | 'operation' | 'callback' | 'link';
  
  // OpenAPI integration
  operationRef?: string;  // Reference to OpenAPI operation
  callback?: string;       // Reference to OpenAPI callback
  link?: string;           // Reference to OpenAPI link
  
  // Existing
  taskId?: string;
  dependencies?: string[];
}
```

## Next Steps

- [ ] Parse OpenAPI components (callbacks, links, operationRef)
- [ ] Extract workflow patterns from OpenAPI spec
- [ ] Create Workflow AST from OpenAPI components
- [ ] Map OpenAPI operations to GDSL procedures
- [ ] Integrate with existing Workflow schema
- [ ] Execute workflows via GDSL execution engines

