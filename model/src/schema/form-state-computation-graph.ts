/**
 * FormState[] Computation Graph Schema
 * 
 * Models Semantic Form Evaluation as a Computation Graph
 * 
 * This schema defines FormState[] as a computation graph where:
 * - Each FormState is a node in the graph
 * - Dependencies between states are edges
 * - Evaluation follows topological ordering
 * - Forward/backward propagation tracks computation
 * - Caching stores computed results
 * 
 * Architecture:
 * - FormState = Node in computation graph
 * - Dependencies = Edges (parent states)
 * - Evaluation = Forward pass (topological order)
 * - Validation = Backward pass (constraint checking)
 * - Cache = Computed values stored per state
 */

import { z } from 'zod';

/**
 * FormState - A node in the Semantic Form Evaluation computation graph
 * 
 * Each FormState represents a point in the evaluation where:
 * - A form shape is being evaluated
 * - Dependencies on other states are tracked
 * - Computed values are cached
 * - Validation constraints are checked
 */
export const FormStateSchema = z.object({
  /** Unique state identifier */
  id: z.string(),
  
  /** State type - what kind of evaluation this represents */
  type: z.enum([
    'shape',      // Shape evaluation (Pure Form)
    'entity',     // Entity evaluation (Given Form)
    'context',    // Context evaluation (Reflection Compression)
    'property',   // Property evaluation (Presupposes Context)
    'morph',      // Morph evaluation (Synthesis)
    'aspect',     // Aspect evaluation (Relation)
    'composite',  // Composite evaluation (multiple states)
  ]),
  
  /** Form reference - which form is being evaluated */
  formId: z.string(),
  
  /** Form shape reference - which shape is being evaluated */
  shapeId: z.string().optional(),
  
  /** Context reference - which context is being used */
  contextId: z.string().optional(),
  
  /** Morph reference - which morph is being used */
  morphId: z.string().optional(),
  
  /** Parent states - dependencies in computation graph */
  parents: z.array(z.string()).default([]),
  
  /** Child states - states that depend on this one */
  children: z.array(z.string()).default([]),
  
  /** Evaluation status */
  status: z.enum([
    'pending',    // Not yet evaluated
    'evaluating', // Currently being evaluated
    'computed',   // Evaluation complete, value cached
    'error',      // Evaluation failed
    'invalidated', // Cached value invalidated (dependency changed)
  ]).default('pending'),
  
  /** Computed value - cached result of evaluation */
  value: z.any().optional(),
  
  /** Value type - what kind of value this is */
  valueType: z.string().optional(),
  
  /** Validation status */
  validation: z.object({
    /** Whether validation has been checked */
    checked: z.boolean().default(false),
    /** Whether value is valid */
    valid: z.boolean().optional(),
    /** Validation errors */
    errors: z.array(z.string()).default([]),
    /** Validation constraints that failed */
    failedConstraints: z.array(z.string()).default([]),
  }).optional(),
  
  /** Evaluation metadata */
  metadata: z.object({
    /** When evaluation started */
    evaluatedAt: z.number().optional(),
    /** When evaluation completed */
    completedAt: z.number().optional(),
    /** Evaluation duration (ms) */
    duration: z.number().optional(),
    /** Number of times evaluated */
    evaluationCount: z.number().default(0),
    /** Cache hit count */
    cacheHits: z.number().default(0),
    /** Cache miss count */
    cacheMisses: z.number().default(0),
  }).optional(),
  
  /** Computation graph metadata */
  graph: z.object({
    /** Topological order index */
    topologicalIndex: z.number().optional(),
    /** Depth in dependency graph */
    depth: z.number().default(0),
    /** Whether this is a root node (no parents) */
    isRoot: z.boolean().default(false),
    /** Whether this is a leaf node (no children) */
    isLeaf: z.boolean().default(false),
    /** Cycle detection - if this state is part of a cycle */
    inCycle: z.boolean().default(false),
  }).optional(),
  
  /** Engine reference - which engine evaluated this state */
  engine: z.enum([
    'shape',
    'entity',
    'context',
    'property',
    'morph',
    'aspect',
  ]).optional(),
  
  /** Dialectical state reference - if this state is from dialectical IR */
  dialecticStateId: z.string().optional(),
  
  /** Provenance - where this state came from */
  provenance: z.object({
    /** Source file/context */
    source: z.string().optional(),
    /** Line range */
    lineRange: z.object({
      start: z.number(),
      end: z.number(),
    }).optional(),
    /** Section */
    section: z.string().optional(),
  }).optional(),
});

export type FormState = z.infer<typeof FormStateSchema>;

/**
 * FormStateGraph - The complete computation graph
 * 
 * This represents the entire computation graph for Semantic Form Evaluation
 */
export const FormStateGraphSchema = z.object({
  /** Graph identifier */
  id: z.string(),
  
  /** All states in the graph */
  states: z.array(FormStateSchema),
  
  /** Root states - states with no parents */
  roots: z.array(z.string()).default([]),
  
  /** Leaf states - states with no children */
  leaves: z.array(z.string()).default([]),
  
  /** Topological order - evaluation order */
  topologicalOrder: z.array(z.string()).default([]),
  
  /** Graph metadata */
  metadata: z.object({
    /** When graph was created */
    createdAt: z.number().optional(),
    /** When graph was last updated */
    updatedAt: z.number().optional(),
    /** Total number of states */
    stateCount: z.number().default(0),
    /** Number of edges (dependencies) */
    edgeCount: z.number().default(0),
    /** Whether graph has cycles */
    hasCycles: z.boolean().default(false),
    /** Graph depth */
    maxDepth: z.number().default(0),
  }).optional(),
  
  /** Evaluation context */
  context: z.object({
    /** Current evaluation phase */
    phase: z.enum([
      'initialization',
      'forward',      // Forward pass (compute values)
      'validation',  // Backward pass (check constraints)
      'completed',
      'error',
    ]).optional(),
    /** Current state being evaluated */
    currentStateId: z.string().optional(),
    /** Evaluation start time */
    startTime: z.number().optional(),
    /** Evaluation end time */
    endTime: z.number().optional(),
  }).optional(),
});

export type FormStateGraph = z.infer<typeof FormStateGraphSchema>;

/**
 * FormStateEvaluation - Evaluation result for a single state
 */
export const FormStateEvaluationSchema = z.object({
  /** State ID */
  stateId: z.string(),
  
  /** Evaluation status */
  status: z.enum([
    'success',
    'error',
    'skipped',  // Skipped due to dependency failure
    'cached',   // Used cached value
  ]),
  
  /** Computed value */
  value: z.any().optional(),
  
  /** Error if evaluation failed */
  error: z.object({
    message: z.string(),
    code: z.string().optional(),
    stack: z.string().optional(),
  }).optional(),
  
  /** Validation result */
  validation: z.object({
    valid: z.boolean(),
    errors: z.array(z.string()).default([]),
  }).optional(),
  
  /** Evaluation metadata */
  metadata: z.object({
    evaluatedAt: z.number(),
    duration: z.number(),
    cacheHit: z.boolean().default(false),
  }).optional(),
});

export type FormStateEvaluation = z.infer<typeof FormStateEvaluationSchema>;

/**
 * FormStateGraphEvaluation - Complete graph evaluation result
 */
export const FormStateGraphEvaluationSchema = z.object({
  /** Graph ID */
  graphId: z.string(),
  
  /** Evaluation results for each state */
  evaluations: z.array(FormStateEvaluationSchema),
  
  /** Overall status */
  status: z.enum([
    'success',    // All states evaluated successfully
    'partial',    // Some states evaluated, some failed
    'error',      // Evaluation failed
  ]),
  
  /** Evaluation metadata */
  metadata: z.object({
    startTime: z.number(),
    endTime: z.number(),
    duration: z.number(),
    totalStates: z.number(),
    successfulStates: z.number(),
    failedStates: z.number(),
    cachedStates: z.number(),
  }),
});

export type FormStateGraphEvaluation = z.infer<typeof FormStateGraphEvaluationSchema>;

/**
 * Helper: Create a FormState
 */
export function createFormState(input: {
  id: string;
  type: FormState['type'];
  formId: string;
  shapeId?: string;
  contextId?: string;
  morphId?: string;
  parents?: string[];
  engine?: FormState['engine'];
  dialecticStateId?: string;
}): FormState {
  return FormStateSchema.parse({
    id: input.id,
    type: input.type,
    formId: input.formId,
    shapeId: input.shapeId,
    contextId: input.contextId,
    morphId: input.morphId,
    parents: input.parents ?? [],
    children: [],
    status: 'pending',
    engine: input.engine,
    dialecticStateId: input.dialecticStateId,
    graph: {
      isRoot: (input.parents ?? []).length === 0,
      depth: 0,
    },
  });
}

/**
 * Helper: Create a FormStateGraph
 */
export function createFormStateGraph(input: {
  id: string;
  states: FormState[];
}): FormStateGraph {
  const states = input.states;
  const stateIds = new Set(states.map(s => s.id));
  
  // Build dependency graph
  const parents = new Map<string, Set<string>>();
  const children = new Map<string, Set<string>>();
  
  for (const state of states) {
    parents.set(state.id, new Set(state.parents));
    children.set(state.id, new Set(state.children));
    
    // Update children references
    for (const parentId of state.parents) {
      if (!children.has(parentId)) {
        children.set(parentId, new Set());
      }
      children.get(parentId)!.add(state.id);
    }
  }
  
  // Find roots and leaves
  const roots = states.filter(s => s.parents.length === 0).map(s => s.id);
  const leaves = states.filter(s => {
    const stateChildren = children.get(s.id) ?? new Set();
    return stateChildren.size === 0;
  }).map(s => s.id);
  
  // Topological sort
  const topologicalOrder = topologicalSort(states, parents);
  
  // Update states with computed graph metadata
  const updatedStates = states.map(state => {
    const stateChildren = children.get(state.id) ?? new Set();
    const depth = computeDepth(state.id, parents);
    
    return {
      ...state,
      children: Array.from(stateChildren),
      graph: {
        ...state.graph,
        topologicalIndex: topologicalOrder.indexOf(state.id),
        depth,
        isRoot: state.parents.length === 0,
        isLeaf: stateChildren.size === 0,
      },
    };
  });
  
  return FormStateGraphSchema.parse({
    id: input.id,
    states: updatedStates,
    roots,
    leaves,
    topologicalOrder,
    metadata: {
      stateCount: states.length,
      edgeCount: states.reduce((sum, s) => sum + s.parents.length, 0),
      hasCycles: topologicalOrder.length !== states.length,
      maxDepth: Math.max(...updatedStates.map(s => s.graph?.depth ?? 0)),
    },
  });
}

/**
 * Topological sort for computation graph
 */
function topologicalSort(
  states: FormState[],
  parents: Map<string, Set<string>>
): string[] {
  const result: string[] = [];
  const visited = new Set<string>();
  const visiting = new Set<string>();
  
  function visit(stateId: string) {
    if (visiting.has(stateId)) {
      // Cycle detected
      return;
    }
    if (visited.has(stateId)) {
      return;
    }
    
    visiting.add(stateId);
    
    const stateParents = parents.get(stateId) ?? new Set();
    for (const parentId of stateParents) {
      visit(parentId);
    }
    
    visiting.delete(stateId);
    visited.add(stateId);
    result.push(stateId);
  }
  
  for (const state of states) {
    visit(state.id);
  }
  
  return result;
}

/**
 * Compute depth in dependency graph
 */
function computeDepth(
  stateId: string,
  parents: Map<string, Set<string>>
): number {
  const stateParents = parents.get(stateId) ?? new Set();
  if (stateParents.size === 0) {
    return 0;
  }
  
  return 1 + Math.max(
    ...Array.from(stateParents).map(parentId => computeDepth(parentId, parents))
  );
}

/**
 * Helper: Evaluate a FormState in the computation graph
 */
export function evaluateFormState(
  state: FormState,
  graph: FormStateGraph,
  getStateValue: (stateId: string) => any
): FormStateEvaluation {
  // Check if already computed
  if (state.status === 'computed' && state.value !== undefined) {
    return FormStateEvaluationSchema.parse({
      stateId: state.id,
      status: 'cached',
      value: state.value,
      metadata: {
        evaluatedAt: Date.now(),
        duration: 0,
        cacheHit: true,
      },
    });
  }
  
  // Check dependencies
  const parentValues = state.parents.map(parentId => {
    const parentState = graph.states.find(s => s.id === parentId);
    if (!parentState) {
      throw new Error(`Parent state not found: ${parentId}`);
    }
    if (parentState.status !== 'computed') {
      throw new Error(`Parent state not computed: ${parentId}`);
    }
    return getStateValue(parentId);
  });
  
  // Evaluate state (this would call the appropriate engine)
  const startTime = Date.now();
  try {
    const value = getStateValue(state.id);
    const duration = Date.now() - startTime;
    
    return FormStateEvaluationSchema.parse({
      stateId: state.id,
      status: 'success',
      value,
      metadata: {
        evaluatedAt: startTime,
        duration,
        cacheHit: false,
      },
    });
  } catch (error) {
    const duration = Date.now() - startTime;
    
    return FormStateEvaluationSchema.parse({
      stateId: state.id,
      status: 'error',
      error: {
        message: error instanceof Error ? error.message : String(error),
      },
      metadata: {
        evaluatedAt: startTime,
        duration,
        cacheHit: false,
      },
    });
  }
}

/**
 * Helper: Evaluate entire FormStateGraph
 */
export function evaluateFormStateGraph(
  graph: FormStateGraph,
  getStateValue: (stateId: string) => any
): FormStateGraphEvaluation {
  const startTime = Date.now();
  const evaluations: FormStateEvaluation[] = [];
  
  // Evaluate in topological order
  for (const stateId of graph.topologicalOrder) {
    const state = graph.states.find(s => s.id === stateId);
    if (!state) {
      continue;
    }
    
    try {
      const evaluation = evaluateFormState(state, graph, getStateValue);
      evaluations.push(evaluation);
    } catch (error) {
      evaluations.push(FormStateEvaluationSchema.parse({
        stateId,
        status: 'error',
        error: {
          message: error instanceof Error ? error.message : String(error),
        },
      }));
    }
  }
  
  const endTime = Date.now();
  const duration = endTime - startTime;
  
  const successfulStates = evaluations.filter(e => e.status === 'success' || e.status === 'cached').length;
  const failedStates = evaluations.filter(e => e.status === 'error').length;
  const cachedStates = evaluations.filter(e => e.status === 'cached').length;
  
  const overallStatus = failedStates === 0
    ? 'success'
    : successfulStates > 0
    ? 'partial'
    : 'error';
  
  return FormStateGraphEvaluationSchema.parse({
    graphId: graph.id,
    evaluations,
    status: overallStatus,
    metadata: {
      startTime,
      endTime,
      duration,
      totalStates: evaluations.length,
      successfulStates,
      failedStates,
      cachedStates,
    },
  });
}

