/**
 * AgentState: The Unified Structure
 * 
 * AgentState = DialecticState + FormState
 * 
 * This is the unified structure that combines:
 * - DialecticState (A Priori) - The logical structure
 * - FormState (A Posteriori) - The evaluation instance
 * - Agent - The internal synthesis that sublates all
 * 
 * Architecture:
 * - Being(shape:entity) → Model
 * - Essence(context:property) → View
 * - Concept(morph:relation) → Controller
 * - Task = Logic + Model
 * - Agent sublates all
 * 
 * The evolution:
 * - Being evolves into Model
 * - Essence evolves into View
 * - Concept evolves into Controller
 * 
 * Active Forms:
 * - The moment of "particular" to logic's "universal"
 * - Universal, Particular, Singular = Compression structure
 */

import { z } from 'zod';
import { DialecticFormStateSchema, DialecticFormState } from './dialectic-form-state';
import { FormStateSchema, FormState } from './form-state-computation-graph';

/**
 * AgentState: The Unified Structure
 * 
 * Combines DialecticState and FormState with Agent-specific structure
 */
export const AgentStateSchema = z.object({
  /** Unique state identifier */
  id: z.string(),
  
  /** DialecticState (A Priori) - The logical structure */
  dialecticState: DialecticFormStateSchema,
  
  /** FormState (A Posteriori) - The evaluation instance */
  formState: FormStateSchema.optional(),
  
  /** Agent reference */
  agentId: z.string().optional(),
  
  /** Task reference (Task = Logic + Model) */
  taskId: z.string().optional(),
  
  /** Workflow reference */
  workflowId: z.string().optional(),
  
  // ============================================================================
  // MVC Evolution: Being → Model, Essence → View, Concept → Controller
  // ============================================================================
  
  /** Model (Being → Model) */
  model: z.object({
    /** Shape (Pure Form) → Model schema */
    shape: z.string().optional(),
    /** Entity (Given Form) → Model instances */
    entity: z.string().optional(),
    /** Model structure */
    structure: z.any().optional(),
  }).optional(),
  
  /** View (Essence → View) */
  view: z.object({
    /** Context (Reflection Compression) → View context */
    context: z.string().optional(),
    /** Property (Presupposes Context) → View properties */
    property: z.string().optional(),
    /** View structure */
    structure: z.any().optional(),
  }).optional(),
  
  /** Controller (Concept → Controller) */
  controller: z.object({
    /** Morph (Synthesis of Shape:Context) → Controller logic */
    morph: z.string().optional(),
    /** Relation (Synthesis of Entity:Property) → Controller actions */
    relation: z.string().optional(),
    /** Controller structure */
    structure: z.any().optional(),
  }).optional(),
  
  // ============================================================================
  // Active Forms: Particular to Logic's Universal
  // ============================================================================
  
  /** Active Forms - The moment of "particular" to logic's "universal" */
  activeForms: z.object({
    /** Model as Active Form of Being */
    model: z.any().optional(),
    /** View as Active Form of Essence */
    view: z.any().optional(),
    /** Controller as Active Form of Concept */
    controller: z.any().optional(),
  }).optional(),
  
  // ============================================================================
  // Universal, Particular, Singular: Compression Structure
  // ============================================================================
  
  /** Universal (Logic) - Fixed schema, a priori structure */
  universal: z.object({
    /** Logic structure */
    logic: z.any().optional(),
    /** Fixed schema */
    schema: z.any().optional(),
  }).optional(),
  
  /** Particular (Active Forms) - Given instances, a posteriori structure */
  particular: z.object({
    /** Active Forms */
    activeForms: z.any().optional(),
    /** Given instances */
    instances: z.any().optional(),
  }).optional(),
  
  /** Singular (Agent) - Synthesis, sublation */
  singular: z.object({
    /** Agent structure */
    agent: z.any().optional(),
    /** Synthesis */
    synthesis: z.any().optional(),
  }).optional(),
  
  // ============================================================================
  // Task = Logic + Model
  // ============================================================================
  
  /** Task structure (Task = Logic + Model) */
  task: z.object({
    /** Logic component (fixed schema) */
    logic: z.any().optional(),
    /** Model component (data layer) */
    model: z.any().optional(),
    /** Task action */
    action: z.any().optional(),
    /** Task state */
    state: z.string().optional(),
  }).optional(),
  
  // ============================================================================
  // Agent Sublates All
  // ============================================================================
  
  /** Agent capabilities - Sublates all structures */
  capabilities: z.object({
    /** Sublates Logic */
    logic: z.boolean().default(false),
    /** Sublates Model */
    model: z.boolean().default(false),
    /** Sublates View */
    view: z.boolean().default(false),
    /** Sublates Controller */
    controller: z.boolean().default(false),
    /** Sublates Task */
    task: z.boolean().default(false),
    /** Sublates Workflow */
    workflow: z.boolean().default(false),
  }).optional(),
  
  /** Agent awareness - Understanding of structures */
  awareness: z.object({
    /** Understands Logic */
    understandsLogic: z.boolean().default(false),
    /** Understands Model */
    understandsModel: z.boolean().default(false),
    /** Understands View */
    understandsView: z.boolean().default(false),
    /** Understands Controller */
    understandsController: z.boolean().default(false),
    /** Understands Task */
    understandsTask: z.boolean().default(false),
    /** Understands Workflow */
    understandsWorkflow: z.boolean().default(false),
  }).optional(),
  
  /** Metadata */
  metadata: z.object({
    /** When created */
    createdAt: z.number().optional(),
    /** When updated */
    updatedAt: z.number().optional(),
    /** Layer: logic, model, or task */
    layer: z.enum(['logic', 'model', 'task']).optional(),
    /** Triadic stage */
    triadicStage: z.enum(['ideal', 'realism', 'synthesis']).optional(),
  }).optional(),
});

export type AgentState = z.infer<typeof AgentStateSchema>;

/**
 * Helper: Create AgentState from DialecticState and FormState
 */
export function createAgentState(input: {
  id: string;
  dialecticState: DialecticFormState;
  formState?: FormState;
  agentId?: string;
  taskId?: string;
  workflowId?: string;
}): AgentState {
  return AgentStateSchema.parse({
    id: input.id,
    dialecticState: input.dialecticState,
    formState: input.formState,
    agentId: input.agentId,
    taskId: input.taskId,
    workflowId: input.workflowId,
    metadata: {
      createdAt: Date.now(),
      layer: input.dialecticState.synthesisMetadata?.layer,
      triadicStage: input.dialecticState.synthesisMetadata?.triadicStage,
    },
  });
}

/**
 * Helper: Extract MVC structure from AgentState
 */
export function extractMVC(state: AgentState): {
  model?: any;
  view?: any;
  controller?: any;
} {
  return {
    model: state.model,
    view: state.view,
    controller: state.controller,
  };
}

/**
 * Helper: Extract Universal, Particular, Singular from AgentState
 */
export function extractCompression(state: AgentState): {
  universal?: any;
  particular?: any;
  singular?: any;
} {
  return {
    universal: state.universal,
    particular: state.particular,
    singular: state.singular,
  };
}

/**
 * Helper: Extract Task structure (Task = Logic + Model)
 */
export function extractTask(state: AgentState): {
  logic?: any;
  model?: any;
  action?: any;
} {
  return {
    logic: state.task?.logic,
    model: state.task?.model,
    action: state.task?.action,
  };
}

