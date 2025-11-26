/**
 * Dialectic IR Schema: Executable Pseudo-Code for Hegelian Dialectic
 *
 * This schema defines the Intermediate Representation (IR) for dialectic states
 * as executable logic, combining OWL/SHACL-style constraints with Prolog-like facts.
 *
 * Purpose: Convert TopicMap + Chunks → Scientific Theory in Pseudo-Code
 *
 * Architecture:
 * - CPU (Concept Processing Unit): Quality/Being → Reflection → Subject
 * - GPU (Mathematical Coprocessor): Quantity → Appearance → Object
 *
 * Each dialectic state captures:
 * - State: Current dialectic position (concept being fixed)
 * - Moments: Active polarities/determinations
 * - Invariants: What must hold for state to be valid
 * - Forces: What drives transition (negations, contradictions)
 * - Transitions: Next state(s) dialectically implied
 * - Provenance: Source tracking (TopicMap + Chunk)
 */

import { z } from 'zod';

/**
 * CPU/GPU Phase Tags
 * Maps dialectic progression to computational architecture
 */
export const CpuGpuPhaseSchema = z.enum([
  'quality',      // Being: Quality (CPU)
  'quantity',    // Being: Quantity (GPU)
  'reflection',   // Essence: Reflection (CPU)
  'appearance',   // Essence: Appearance (GPU)
  'subject',     // Concept: Subject (CPU)
  'object',      // Concept: Object (GPU)
]);

export type CpuGpuPhase = z.infer<typeof CpuGpuPhaseSchema>;

/**
 * Dialectic Moment
 * A polarity or determination active in the current state
 * Moments are the "parts" that constitute the state
 */
export const MomentSchema = z.object({
  /** Name/identifier of the moment */
  name: z.string(),

  /** What this moment is/represents */
  definition: z.string(),

  /** Type: polarity, determination, quality, etc. */
  type: z.enum(['polarity', 'determination', 'quality', 'negation', 'sublation', 'mediation', 'moment', 'process']),

  /** Relationship to other moments (if any) */
  relation: z.enum(['opposite', 'mediates', 'contains', 'transforms', 'negates']).optional(),

  /** Related moment name (if relation specified) */
  relatedTo: z.string().optional(),
});

export type Moment = z.infer<typeof MomentSchema>;

/**
 * Invariant Constraint
 * What must hold for the state to be valid
 * Expressed as logical constraints (OWL/SHACL-style)
 */
export const InvariantSchema = z.object({
  /** Constraint identifier */
  id: z.string(),

  /** Constraint expression (readable logic) */
  constraint: z.string(),

  /** Formal predicate (Prolog-style) */
  predicate: z.string().optional(),

  /** Conditions under which this invariant holds */
  conditions: z.array(z.string()).optional(),
});

export type Invariant = z.infer<typeof InvariantSchema>;

/**
 * Transition Force
 * What drives the dialectic forward
 * The "motor" of the dialectic movement
 */
export const ForceSchema = z.object({
  /** Force identifier */
  id: z.string(),

  /** Description of the force */
  description: z.string(),

  /** Type: contradiction, negation, externality, etc. */
  type: z.enum(['contradiction', 'negation', 'externality', 'sublation', 'mediation', 'immanence', 'reflection', 'passover']),

  /** Trigger condition (when this force activates) */
  trigger: z.string(),

  /** Effect (what happens when force activates) */
  effect: z.string(),

  /** Target state ID (where this force leads) */
  targetState: z.string(),
});

export type Force = z.infer<typeof ForceSchema>;

/**
 * State Transition
 * Explicit dialectic movement from one state to another
 */
export const TransitionSchema = z.object({
  /** Transition identifier */
  id: z.string(),

  /** Source state ID */
  from: z.string(),

  /** Target state ID */
  to: z.string(),

  /** Dialectic mechanism (how the transition occurs) */
  mechanism: z.enum(['negation', 'sublation', 'mediation', 'passover', 'reflection', 'contradiction']),

  /** Middle term (if any) - the mediating element */
  middleTerm: z.string().optional(),

  /** Conditions for transition */
  conditions: z.array(z.string()).optional(),

  /** Description of the transition */
  description: z.string(),
});

export type Transition = z.infer<typeof TransitionSchema>;

/**
 * Provenance
 * Source tracking: TopicMap + Chunk references
 */
export const DialecticProvenanceSchema = z.object({
  /** TopicMap entry ID */
  topicMapId: z.string(),

  /** Chunk ID (from chunks.md) */
  chunkId: z.string().optional(),

  /** Source text line range */
  lineRange: z.object({
    start: z.number(),
    end: z.number(),
  }),

  /** Section/parent context */
  section: z.string().optional(),

  /** Order within section */
  order: z.number().optional(),
});

export type DialecticProvenance = z.infer<typeof DialecticProvenanceSchema>;

/**
 * Dialectic State
 * The core IR structure: a dialectic position as executable logic
 */
export const DialecticStateSchema = z.object({
  /** Unique state identifier (from TopicMapEntry.id) */
  id: z.string(),

  /** State title (from TopicMapEntry.title) */
  title: z.string(),

  /** Current concept being fixed */
  concept: z.string(),

  /** CPU/GPU phase tag */
  phase: CpuGpuPhaseSchema,

  /** Active moments (polarities, determinations) */
  moments: z.array(MomentSchema),

  /** Invariant constraints (what must hold) */
  invariants: z.array(InvariantSchema),

  /** Transition forces (what drives change) */
  forces: z.array(ForceSchema).optional(),

  /** Explicit transitions to next states */
  transitions: z.array(TransitionSchema).optional(),

  /** Next state ID(s) - dialectically implied */
  nextStates: z.array(z.string()).optional(),

  /** Previous state ID(s) - where this came from */
  previousStates: z.array(z.string()).optional(),

  /** Provenance: source tracking */
  provenance: DialecticProvenanceSchema,

  /** Description (from TopicMapEntry.description) */
  description: z.string().optional(),

  /** Key points (from TopicMapEntry.keyPoints) */
  keyPoints: z.array(z.string()).optional(),
});

export type DialecticState = z.infer<typeof DialecticStateSchema>;

/**
 * Dialectic IR Document
 * Complete IR representation for a section (e.g., Constitution)
 */
export const DialecticIRSchema = z.object({
  /** Document identifier */
  id: z.string(),

  /** Document title */
  title: z.string(),

  /** Section context */
  section: z.string(),

  /** All dialectic states in this document */
  states: z.array(DialecticStateSchema),

  /** Metadata */
  metadata: z.object({
    sourceFile: z.string().optional(),
    totalStates: z.number(),
    cpuGpuMapping: z.record(z.string(), CpuGpuPhaseSchema).optional(),
    createdAt: z.string().optional(),
    updatedAt: z.string().optional(),
  }).optional(),
});

export type DialecticIR = z.infer<typeof DialecticIRSchema>;

/**
 * Helper: Create a dialectic state from TopicMap entry
 * This is the conversion function: TopicMap → DialecticState
 */
export function createDialecticState(
  topicMapEntry: {
    id: string;
    title: string;
    description: string;
    keyPoints: string[];
    lineRange: { start: number; end: number };
    section?: string;
    order?: number;
  },
  stateConfig: {
    concept: string;
    phase: CpuGpuPhase;
    moments: Moment[];
    invariants: Invariant[];
    forces?: Force[];
    transitions?: Transition[];
    nextStates?: string[];
    previousStates?: string[];
  }
): DialecticState {
  return {
    id: topicMapEntry.id,
    title: topicMapEntry.title,
    concept: stateConfig.concept,
    phase: stateConfig.phase,
    moments: stateConfig.moments,
    invariants: stateConfig.invariants,
    forces: stateConfig.forces,
    transitions: stateConfig.transitions,
    nextStates: stateConfig.nextStates,
    previousStates: stateConfig.previousStates,
    provenance: {
      topicMapId: topicMapEntry.id,
      lineRange: topicMapEntry.lineRange,
      section: topicMapEntry.section,
      order: topicMapEntry.order,
    },
    description: topicMapEntry.description,
    keyPoints: topicMapEntry.keyPoints,
  };
}

// --- Commands ---

export type DialecticStateTransitionCmd = {
  kind: 'dialectic.state.transition';
  payload: {
    fromStateId: string;
    toStateId: string;
    dialecticState: DialecticState;
  };
  meta?: Record<string, unknown>;
};

export type DialecticMomentActivateCmd = {
  kind: 'dialectic.moment.activate';
  payload: {
    stateId: string;
    moment: Moment;
  };
  meta?: Record<string, unknown>;
};

export type DialecticForceApplyCmd = {
  kind: 'dialectic.force.apply';
  payload: {
    stateId: string;
    force: Force;
  };
  meta?: Record<string, unknown>;
};

export type DialecticInvariantCheckCmd = {
  kind: 'dialectic.invariant.check';
  payload: {
    stateId: string;
    invariants: Invariant[];
  };
  meta?: Record<string, unknown>;
};

export type DialecticEvaluateCmd = {
  kind: 'dialectic.evaluate';
  payload: {
    dialecticState: DialecticState;
    context?: any;
  };
  meta?: Record<string, unknown>;
};

export type DialecticCommand =
  | DialecticStateTransitionCmd
  | DialecticMomentActivateCmd
  | DialecticForceApplyCmd
  | DialecticInvariantCheckCmd
  | DialecticEvaluateCmd;

// --- Events ---

export type DialecticStateTransitionedEvent = {
  kind: 'dialectic.state.transitioned';
  payload: {
    fromState: string;
    toState: string;
    mechanism?: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticMomentActivatedEvent = {
  kind: 'dialectic.moment.activated';
  payload: {
    stateId: string;
    moment: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticForceAppliedEvent = {
  kind: 'dialectic.force.applied';
  payload: {
    stateId: string;
    force: string;
    effect: string;
    targetState?: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticInvariantViolatedEvent = {
  kind: 'dialectic.invariant.violated';
  payload: {
    stateId: string;
    invariant: string;
    reason: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticInvariantSatisfiedEvent = {
  kind: 'dialectic.invariant.satisfied';
  payload: {
    stateId: string;
    count: number;
  };
  meta?: Record<string, unknown>;
};

export type DialecticEvaluatedEvent = {
  kind: 'dialectic.evaluated';
  payload: {
    stateId: string;
    concept: string;
    phase: string;
  };
  meta?: Record<string, unknown>;
};

export type DialecticEvent =
  | DialecticStateTransitionedEvent
  | DialecticMomentActivatedEvent
  | DialecticForceAppliedEvent
  | DialecticInvariantViolatedEvent
  | DialecticInvariantSatisfiedEvent
  | DialecticEvaluatedEvent;
