/**
 * DialecticFormState: Unified Schema for A Priori and A Posteriori Synthesis
 * 
 * This schema unifies DialecticState and FormState into a single fixed schema
 * that supports both:
 * - Pure a priori synthesis (@logic) - The presupposed, compressed basis
 * - Pure a posteriori synthesis (@model) - The given, decompressed instance
 * 
 * Architecture:
 * - @logic = Fixed schema (a priori synthesis) - The ideal
 * - @model = Client to Form Server (a posteriori synthesis) - The real
 * - Triadic Progression: Ideal → Realism
 * 
 * The key insight:
 * - DialecticState = Logical structure (a priori, presupposed)
 * - FormState = Evaluation instance (a posteriori, given)
 * - They're the same thing, just different perspectives
 * - @logic provides the fixed schema (a priori synthesis)
 * - @model uses it as a client (a posteriori synthesis)
 */

import { z } from 'zod';
import { FormStateSchema, FormState } from './form-state-computation-graph';

/**
 * DialecticFormState: Unified schema combining DialecticState and FormState
 * 
 * This is the fixed schema that supports:
 * - A priori synthesis (@logic) - The ideal, presupposed structure
 * - A posteriori synthesis (@model) - The real, given instance
 */
export const DialecticFormStateSchema = z.object({
  /** Unique state identifier */
  id: z.string(),
  
  /** State title */
  title: z.string(),
  
  /** Current concept being fixed */
  concept: z.string(),
  
  /** CPU/GPU phase tag */
  phase: z.enum([
    'quality',
    'quantity',
    'reflection',
    'essence',
    'concept',
    'idea',
  ]),
  
  // ============================================================================
  // A PRIORI SYNTHESIS (@logic) - The Ideal, Presupposed Structure
  // ============================================================================
  
  /** Active moments (polarities, determinations) - A priori structure */
  moments: z.array(z.object({
    name: z.string(),
    definition: z.string().optional(),
    type: z.string().optional(),
    relation: z.string().optional(),
    relatedTo: z.string().optional(),
  })),
  
  /** Invariant constraints (what must hold) - A priori structure */
  invariants: z.array(z.object({
    id: z.string(),
    constraint: z.string(),
    predicate: z.string().optional(),
    conditions: z.array(z.string()).optional(),
  })),
  
  /** Transition forces (what drives change) - A priori structure */
  forces: z.array(z.object({
    id: z.string(),
    description: z.string().optional(),
    type: z.string().optional(),
    trigger: z.string().optional(),
    effect: z.string().optional(),
    targetState: z.string().optional(),
  })).optional(),
  
  /** Explicit transitions to next states - A priori structure */
  transitions: z.array(z.object({
    id: z.string(),
    from: z.string(),
    to: z.string(),
    mechanism: z.string().optional(),
    middleTerm: z.string().optional(),
    description: z.string().optional(),
  })).optional(),
  
  /** Next state ID(s) - dialectically implied - A priori structure */
  nextStates: z.array(z.string()).optional(),
  
  /** Previous state ID(s) - where this came from - A priori structure */
  previousStates: z.array(z.string()).optional(),
  
  /** Provenance: source tracking - A priori structure */
  provenance: z.object({
    topicMapId: z.string().optional(),
    lineRange: z.object({
      start: z.number(),
      end: z.number(),
    }).optional(),
    section: z.string().optional(),
    order: z.number().optional(),
  }).optional(),
  
  /** Description - A priori structure */
  description: z.string().optional(),
  
  /** Key points - A priori structure */
  keyPoints: z.array(z.string()).optional(),
  
  // ============================================================================
  // A POSTERIORI SYNTHESIS (@model) - The Real, Given Instance
  // ============================================================================
  
  /** Form evaluation state - A posteriori instance */
  formState: FormStateSchema.optional(),
  
  /** Evaluation status - A posteriori instance */
  evaluationStatus: z.enum([
    'pending',
    'evaluating',
    'computed',
    'error',
    'invalidated',
  ]).optional(),
  
  /** Computed value - A posteriori instance */
  computedValue: z.any().optional(),
  
  /** Value type - A posteriori instance */
  valueType: z.string().optional(),
  
  /** Validation status - A posteriori instance */
  validation: z.object({
    checked: z.boolean().default(false),
    valid: z.boolean().optional(),
    errors: z.array(z.string()).default([]),
    failedConstraints: z.array(z.string()).default([]),
  }).optional(),
  
  /** Evaluation metadata - A posteriori instance */
  evaluationMetadata: z.object({
    evaluatedAt: z.number().optional(),
    completedAt: z.number().optional(),
    duration: z.number().optional(),
    evaluationCount: z.number().default(0),
    cacheHits: z.number().default(0),
    cacheMisses: z.number().default(0),
  }).optional(),
  
  /** Computation graph metadata - A posteriori instance */
  graphMetadata: z.object({
    topologicalIndex: z.number().optional(),
    depth: z.number().default(0),
    isRoot: z.boolean().default(false),
    isLeaf: z.boolean().default(false),
    inCycle: z.boolean().default(false),
    parents: z.array(z.string()).default([]),
    children: z.array(z.string()).default([]),
  }).optional(),
  
  /** Engine reference - A posteriori instance */
  engine: z.enum([
    'shape',
    'entity',
    'context',
    'property',
    'morph',
    'aspect',
  ]).optional(),
  
  /** Form reference - A posteriori instance */
  formId: z.string().optional(),
  
  /** Shape reference - A posteriori instance */
  shapeId: z.string().optional(),
  
  /** Context reference - A posteriori instance */
  contextId: z.string().optional(),
  
  /** Morph reference - A posteriori instance */
  morphId: z.string().optional(),
  
  // ============================================================================
  // SYNTHESIS: A Priori + A Posteriori
  // ============================================================================
  
  /** Synthesis mode - How a priori and a posteriori are combined */
  synthesisMode: z.enum([
    'a_priori',      // Pure a priori (@logic only)
    'a_posteriori',  // Pure a posteriori (@model only)
    'synthetic',     // Combined (both @logic and @model)
  ]).default('synthetic'),
  
  /** Synthesis metadata */
  synthesisMetadata: z.object({
    /** Whether this is a fixed schema (a priori) */
    isFixedSchema: z.boolean().default(true),
    /** Whether this is a client instance (a posteriori) */
    isClientInstance: z.boolean().default(false),
    /** Layer: @logic or @model */
    layer: z.enum(['logic', 'model']).optional(),
    /** Triadic progression stage */
    triadicStage: z.enum(['ideal', 'realism', 'synthesis']).optional(),
  }).optional(),
});

export type DialecticFormState = z.infer<typeof DialecticFormStateSchema>;

/**
 * Helper: Create a DialecticFormState from DialecticState (a priori)
 */
export function createDialecticFormStateFromDialectic(input: {
  id: string;
  title: string;
  concept: string;
  phase: DialecticFormState['phase'];
  moments: Array<{
    name: string;
    definition?: string;
    type?: string;
    relation?: string;
    relatedTo?: string;
  }>;
  invariants: Array<{
    id: string;
    constraint: string;
    predicate?: string;
    conditions?: string[];
  }>;
  forces?: Array<{
    id: string;
    description?: string;
    type?: string;
    trigger?: string;
    effect?: string;
    targetState?: string;
  }>;
  transitions?: Array<{
    id: string;
    from: string;
    to: string;
    mechanism?: string;
    middleTerm?: string;
    description?: string;
  }>;
  nextStates?: string[];
  previousStates?: string[];
  provenance?: {
    topicMapId?: string;
    lineRange?: { start: number; end: number };
    section?: string;
    order?: number;
  };
  description?: string;
  keyPoints?: string[];
}): DialecticFormState {
  return DialecticFormStateSchema.parse({
    id: input.id,
    title: input.title,
    concept: input.concept,
    phase: input.phase,
    moments: input.moments,
    invariants: input.invariants,
    forces: input.forces,
    transitions: input.transitions,
    nextStates: input.nextStates,
    previousStates: input.previousStates,
    provenance: input.provenance,
    description: input.description,
    keyPoints: input.keyPoints,
    synthesisMode: 'a_priori',
    synthesisMetadata: {
      isFixedSchema: true,
      isClientInstance: false,
      layer: 'logic',
      triadicStage: 'ideal',
    },
  });
}

/**
 * Helper: Create a DialecticFormState from FormState (a posteriori)
 */
export function createDialecticFormStateFromForm(input: {
  id: string;
  formState: FormState;
  concept?: string;
  phase?: DialecticFormState['phase'];
}): DialecticFormState {
  return DialecticFormStateSchema.parse({
    id: input.id,
    title: input.formState.id,
    concept: input.concept ?? input.formState.type,
    phase: input.phase ?? 'quality',
    moments: [],
    invariants: [],
    formState: input.formState,
    evaluationStatus: input.formState.status,
    computedValue: input.formState.value,
    valueType: input.formState.valueType,
    validation: input.formState.validation,
    evaluationMetadata: input.formState.metadata,
    graphMetadata: input.formState.graph ? {
      topologicalIndex: input.formState.graph.topologicalIndex,
      depth: input.formState.graph.depth,
      isRoot: input.formState.graph.isRoot,
      isLeaf: input.formState.graph.isLeaf,
      inCycle: input.formState.graph.inCycle,
      parents: input.formState.parents,
      children: input.formState.children,
    } : undefined,
    engine: input.formState.engine,
    formId: input.formState.formId,
    shapeId: input.formState.shapeId,
    contextId: input.formState.contextId,
    morphId: input.formState.morphId,
    synthesisMode: 'a_posteriori',
    synthesisMetadata: {
      isFixedSchema: false,
      isClientInstance: true,
      layer: 'model',
      triadicStage: 'realism',
    },
  });
}

/**
 * Helper: Synthesize a priori and a posteriori
 */
export function synthesizeDialecticFormState(input: {
  dialecticState: DialecticFormState;  // A priori (@logic)
  formState: FormState;                 // A posteriori (@model)
}): DialecticFormState {
  return DialecticFormStateSchema.parse({
    // A priori (from dialectic state)
    id: input.dialecticState.id,
    title: input.dialecticState.title,
    concept: input.dialecticState.concept,
    phase: input.dialecticState.phase,
    moments: input.dialecticState.moments,
    invariants: input.dialecticState.invariants,
    forces: input.dialecticState.forces,
    transitions: input.dialecticState.transitions,
    nextStates: input.dialecticState.nextStates,
    previousStates: input.dialecticState.previousStates,
    provenance: input.dialecticState.provenance,
    description: input.dialecticState.description,
    keyPoints: input.dialecticState.keyPoints,
    
    // A posteriori (from form state)
    formState: input.formState,
    evaluationStatus: input.formState.status,
    computedValue: input.formState.value,
    valueType: input.formState.valueType,
    validation: input.formState.validation,
    evaluationMetadata: input.formState.metadata,
    graphMetadata: input.formState.graph ? {
      topologicalIndex: input.formState.graph.topologicalIndex,
      depth: input.formState.graph.depth,
      isRoot: input.formState.graph.isRoot,
      isLeaf: input.formState.graph.isLeaf,
      inCycle: input.formState.graph.inCycle,
      parents: input.formState.parents,
      children: input.formState.children,
    } : undefined,
    engine: input.formState.engine,
    formId: input.formState.formId,
    shapeId: input.formState.shapeId,
    contextId: input.formState.contextId,
    morphId: input.formState.morphId,
    
    // Synthesis
    synthesisMode: 'synthetic',
    synthesisMetadata: {
      isFixedSchema: true,
      isClientInstance: true,
      layer: 'model',  // @model is the client
      triadicStage: 'synthesis',
    },
  });
}

/**
 * Helper: Extract a priori part (for @logic)
 */
export function extractApriori(state: DialecticFormState): DialecticFormState {
  return DialecticFormStateSchema.parse({
    id: state.id,
    title: state.title,
    concept: state.concept,
    phase: state.phase,
    moments: state.moments,
    invariants: state.invariants,
    forces: state.forces,
    transitions: state.transitions,
    nextStates: state.nextStates,
    previousStates: state.previousStates,
    provenance: state.provenance,
    description: state.description,
    keyPoints: state.keyPoints,
    synthesisMode: 'a_priori',
    synthesisMetadata: {
      isFixedSchema: true,
      isClientInstance: false,
      layer: 'logic',
      triadicStage: 'ideal',
    },
  });
}

/**
 * Helper: Extract a posteriori part (for @model)
 */
export function extractAposteriori(state: DialecticFormState): FormState {
  if (!state.formState) {
    throw new Error('No form state in dialectic form state');
  }
  return state.formState;
}

