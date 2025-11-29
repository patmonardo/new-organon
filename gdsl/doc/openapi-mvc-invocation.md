# OpenAPI → MVC Invocation Pattern

## Philosophy

**Simple, Observable Invocation** - Not Enterprise BPM

We're not building enterprise workflow systems (BPMN, Oracle BPM, Jenkins). Instead, we want:
- ✅ **Simple operation chaining** via OpenAPI components
- ✅ **Observable invocations** from OpenAPI to MVC apps
- ✅ **Lightweight orchestration** - just enough to coordinate operations

**Workflows are the Task Agent's domain** - they handle complex workflow orchestration. GDSL focuses on **simple invocation patterns**.

## OpenAPI Component Workflow (Simple)

### Pattern: Operation References

```yaml
paths:
  /customers/{id}:
    get:
      operationId: getCustomer
      responses:
        '200':
          links:
            getOrders:
              operationRef: '#/paths/~1customers~1{id}~1orders/get'
              parameters:
                id: '$response.body#/id'
            
            getInvoices:
              operationRef: '#/paths/~1customers~1{id}~1invoices/get'
              parameters:
                id: '$response.body#/id'
```

**This is NOT BPMN** - it's just operation chaining with observable invocations.

## MVC App Integration

### OpenAPI Operation → MVC Controller

```typescript
// OpenAPI operation
{
  operationId: 'getCustomer',
  path: '/customers/{id}',
  method: 'get'
}

// Maps to MVC Controller
const controller = new CustomerController('view');
await controller.loadCustomerProfile(id);
const { element } = controller.renderRadixDashboard();
```

### Observable Invocation

```typescript
interface ObservableInvocation {
  operationId: string;
  path: string;
  method: string;
  parameters: Record<string, unknown>;
  startTime: number;
  endTime?: number;
  result?: unknown;
  error?: Error;
  metadata?: Record<string, unknown>;
}

// Track invocations
const invocation: ObservableInvocation = {
  operationId: 'getCustomer',
  path: '/customers/123',
  method: 'get',
  parameters: { id: '123' },
  startTime: Date.now()
};

try {
  const result = await executeOperation(invocation);
  invocation.result = result;
  invocation.endTime = Date.now();
} catch (error) {
  invocation.error = error;
  invocation.endTime = Date.now();
}

// Observable - log, metrics, tracing
observeInvocation(invocation);
```

## Simple Workflow Pattern

### Not Enterprise BPM

**We DON'T need:**
- ❌ Complex state machines
- ❌ BPMN diagrams
- ❌ Enterprise workflow engines
- ❌ Heavy orchestration

**We DO need:**
- ✅ Operation chaining (via OpenAPI links)
- ✅ Observable invocations
- ✅ Simple error handling
- ✅ Parameter passing between operations

### Example: Customer Dashboard Flow

```typescript
// Simple workflow - just operation chaining
const workflow = {
  steps: [
    {
      operationId: 'getCustomer',
      parameters: { id: customerId },
      output: 'customer'
    },
    {
      operationId: 'getCustomerOrders',
      parameters: { 
        customerId: '$customer.id'  // Reference previous output
      },
      output: 'orders'
    },
    {
      operationId: 'getCustomerInvoices',
      parameters: { 
        customerId: '$customer.id' 
      },
      output: 'invoices'
    }
  ]
};

// Execute with observability
for (const step of workflow.steps) {
  const invocation = await invokeOperation(step);
  observeInvocation(invocation);
}
```

## Task Agent Handles Complex Workflows

**Task Agent's Domain:**
- Complex workflow orchestration
- Dialectical workflows (Process + Coordination)
- Multi-agent coordination
- Long-running processes

**GDSL's Domain:**
- Simple operation invocation
- Observable API calls
- OpenAPI → MVC mapping
- Lightweight orchestration

## Integration Architecture

```
OpenAPI Spec
    ↓
Parse Components (links, operationRef)
    ↓
Simple Workflow (operation chaining)
    ↓
Invoke MVC Controllers
    ↓
Observable Results
    ↓
Task Agent (for complex workflows)
```

## Implementation

### 1. Parse OpenAPI Components

```typescript
function parseOpenAPIWorkflow(openApiSpec: OpenAPISpec): SimpleWorkflow {
  const workflow: SimpleWorkflow = {
    steps: []
  };
  
  // Extract operation references from links
  for (const [path, pathItem] of Object.entries(openApiSpec.paths)) {
    for (const [method, operation] of Object.entries(pathItem)) {
      if (operation.responses) {
        for (const [status, response] of Object.entries(operation.responses)) {
          if (response.links) {
            for (const [linkName, link] of Object.entries(response.links)) {
              workflow.steps.push({
                operationId: link.operationRef || link.operationId,
                parameters: link.parameters,
                output: linkName
              });
            }
          }
        }
      }
    }
  }
  
  return workflow;
}
```

### 2. Map to MVC Controllers

```typescript
function mapOperationToMVC(operationId: string): MVCController {
  // Map OpenAPI operation to MVC controller
  const mapping = {
    'getCustomer': CustomerController,
    'getCustomerOrders': OrderController,
    'getCustomerInvoices': InvoiceController
  };
  
  return mapping[operationId];
}
```

### 3. Observable Invocation

```typescript
async function invokeOperation(
  step: WorkflowStep,
  context: ExecutionContext
): Promise<ObservableInvocation> {
  const invocation: ObservableInvocation = {
    operationId: step.operationId,
    startTime: Date.now(),
    parameters: resolveParameters(step.parameters, context)
  };
  
  try {
    const Controller = mapOperationToMVC(step.operationId);
    const controller = new Controller();
    const result = await controller.execute();
    
    invocation.result = result;
    invocation.endTime = Date.now();
    
    // Observable - emit event
    emit('invocation.completed', invocation);
    
    return invocation;
  } catch (error) {
    invocation.error = error;
    invocation.endTime = Date.now();
    
    // Observable - emit error
    emit('invocation.failed', invocation);
    
    throw error;
  }
}
```

## Key Principles

1. **Simple, not Enterprise** - No BPMN, no heavy orchestration
2. **Observable** - Every invocation is tracked and observable
3. **OpenAPI → MVC** - Direct mapping from OpenAPI operations to MVC controllers
4. **Task Agent for Complex** - Complex workflows go to Task Agent
5. **Lightweight** - Just enough to coordinate operations

## Next Steps

- [ ] Implement OpenAPI component parsing
- [ ] Create simple workflow execution
- [ ] Add observable invocation tracking
- [ ] Map OpenAPI operations to MVC controllers
- [ ] Integrate with Task Agent for complex workflows

