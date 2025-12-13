/**
 * Workflow Schema: Process–Coordination (Active Concept in Organization)
 * ======================================================================
 *
 * In the TAW dialectical system, Workflow represents the transformation of
 * Concept into active, organized coordination:
 *
 *   - Process: The active sequence of transformations (Active Concept as temporal unfolding)
 *   - Coordination: The organizational principle that governs the process (Active Concept as systematic unity)
 *
 * Workflow = Process : Coordination
 *
 * Philosophical Note:
 * Workflow is Concept in organization - not static logical structure but dynamic systematic coordination.
 * It represents the movement of Concept toward complete systematic organization and temporal actualization.
 * The Process is Concept as temporal unfolding, the Coordination is Concept as systematic unity.
 * This enables the dialectical bridge between conceptual Logic and organized Agency.
 */

import { z } from 'zod';

// Process: The active sequence of transformations (Active Concept as temporal unfolding)
export const ProcessSchema = z.object({
  id: z.string(),
  name: z.string(),
  steps: z.array(
    z.object({
      id: z.string(),
      name: z.string(),
      type: z.enum(['task', 'decision', 'transformation', 'synthesis']),
      dependencies: z.array(z.string()).optional(),
      duration: z.number().optional(), // Expected duration in milliseconds
      resources: z.record(z.string(), z.any()).optional(),
      // Dialectical structure
      dialecticalRole: z
        .enum(['thesis', 'antithesis', 'synthesis'])
        .default('thesis'),
    }),
  ),
  // Process characteristics
  approach: z
    .enum(['linear', 'parallel', 'dialectical', 'organic'])
    .default('linear'),
  adaptability: z
    .enum(['fixed', 'flexible', 'self-organizing', 'transcendent'])
    .default('fixed'),
});

// Coordination: The organizational principle that governs the process (Active Concept as systematic unity)
export const CoordinationSchema = z.object({
  id: z.string(),
  name: z.string(),
  rules: z.array(
    z.object({
      id: z.string(),
      condition: z.string(),
      action: z.string(),
      priority: z.number().default(1),
    }),
  ),
  // Coordination mechanisms
  synchronization: z
    .enum(['loose', 'tight', 'dialectical', 'organic'])
    .default('loose'),
  conflictResolution: z
    .enum(['hierarchy', 'consensus', 'dialectical', 'transcendent'])
    .default('hierarchy'),
  // Systematic structure
  organizationalPrinciple: z
    .enum(['centralized', 'distributed', 'dialectical', 'absolute'])
    .default('centralized'),
  emergenceLevel: z
    .enum(['none', 'basic', 'complex', 'transcendent'])
    .default('none'),
});

// Workflow: The synthesis of Process and Coordination
export const WorkflowSchema = z
  .object({
    id: z.string(),
    process: ProcessSchema,
    coordination: CoordinationSchema,
    // State management
    status: z
      .enum([
        'initialized',
        'running',
        'paused',
        'completed',
        'failed',
        'transcended',
      ])
      .default('initialized'),
    progress: z.number().min(0).max(1).default(0),
    // Dialectical context
    dialecticalContext: z
      .object({
        stage: z.enum(['being', 'essence', 'concept', 'absolute-idea']),
        systemicLevel: z.enum(['simple', 'complex', 'dialectical', 'absolute']),
        emergentProperties: z.array(z.string()).optional(),
      })
      .optional(),
    // Performance metrics
    metrics: z
      .object({
        efficiency: z.number().min(0).max(1).optional(),
        coherence: z.number().min(0).max(1).optional(),
        adaptability: z.number().min(0).max(1).optional(),
        emergence: z.number().min(0).max(1).optional(),
      })
      .optional(),
    // Temporal dimension
    startedAt: z.date().optional(),
    completedAt: z.date().optional(),
    // Metadata
    metadata: z.record(z.string(), z.any()).optional(),
  })
  .catchall(z.any());

export type Process = z.infer<typeof ProcessSchema>;
export type Coordination = z.infer<typeof CoordinationSchema>;
export type Workflow = z.infer<typeof WorkflowSchema>;

// =====================================================================
// WORKFLOW UNITY (W = C + C)
// =====================================================================
//
// The public/external interface may later add agent-facing specifications,
// but the internal absolute meaning here is:
// - Concept: the goal/constraints (what)
// - Controller: the operational driver (how)
//
// Workflow = Concept + Controller

export const WorkflowConceptSchema = z.object({
  goal: z.object({
    id: z.string(),
    type: z.string(),
    description: z.string(),
  }),
  constraints: z.array(z.string()).optional(),
});

export const WorkflowControllerStepSchema = z.object({
  id: z.string(),
  description: z.string(),
  action: z.string(),
  defaultInput: z.unknown().optional(),
});

export const WorkflowControllerSchema = z.object({
  goalId: z.string(),
  steps: z.array(WorkflowControllerStepSchema),
});

export const WorkflowUnitySchema = z.object({
  id: z.string(),
  concept: WorkflowConceptSchema,
  controller: WorkflowControllerSchema,
});

export type WorkflowConcept = z.infer<typeof WorkflowConceptSchema>;
export type WorkflowControllerStep = z.infer<typeof WorkflowControllerStepSchema>;
export type WorkflowController = z.infer<typeof WorkflowControllerSchema>;
export type WorkflowUnity = z.infer<typeof WorkflowUnitySchema>;

/**
 * Workflow Factory: Creates workflows with dialectical organization principles
 */
export class WorkflowFactory {
  /**
   * Create a Being-level workflow (linear, simple coordination)
   */
  public static createBeingWorkflow(name: string, steps: string[]): Workflow {
    return {
      id: `being-workflow-${Date.now()}`,
      process: {
        id: `process-${Date.now()}`,
        name: `${name}-process`,
        steps: steps.map((step, index) => ({
          id: `step-${index}`,
          name: step,
          type: 'task',
          dialecticalRole: 'thesis',
        })),
        approach: 'linear',
        adaptability: 'fixed',
      },
      coordination: {
        id: `coordination-${Date.now()}`,
        name: `${name}-coordination`,
        rules: [
          {
            id: 'sequential-rule',
            condition: 'previous-step-completed',
            action: 'start-next-step',
            priority: 1,
          },
        ],
        synchronization: 'tight',
        conflictResolution: 'hierarchy',
        organizationalPrinciple: 'centralized',
        emergenceLevel: 'none',
      },
      status: 'initialized',
      progress: 0,
      dialecticalContext: {
        stage: 'being',
        systemicLevel: 'simple',
      },
    };
  }

  /**
   * Create an Essence-level workflow (parallel, mediated coordination)
   */
  public static createEssenceWorkflow(
    name: string,
    processes: Array<{ name: string; steps: string[] }>,
  ): Workflow {
    const allSteps = processes.flatMap((proc, procIndex) =>
      proc.steps.map((step, stepIndex) => {
        let role: 'thesis' | 'antithesis' | 'synthesis';
        if (stepIndex % 3 === 0) role = 'thesis';
        else if (stepIndex % 3 === 1) role = 'antithesis';
        else role = 'synthesis';

        return {
          id: `${procIndex}-${stepIndex}`,
          name: step,
          type: 'transformation' as const,
          dialecticalRole: role,
        };
      }),
    );

    return {
      id: `essence-workflow-${Date.now()}`,
      process: {
        id: `process-${Date.now()}`,
        name: `${name}-essence-process`,
        steps: allSteps,
        approach: 'parallel',
        adaptability: 'flexible',
      },
      coordination: {
        id: `coordination-${Date.now()}`,
        name: `${name}-essence-coordination`,
        rules: [
          {
            id: 'mediation-rule',
            condition: 'contradiction-detected',
            action: 'initiate-mediation',
            priority: 2,
          },
          {
            id: 'synthesis-rule',
            condition: 'mediation-completed',
            action: 'synthesize-results',
            priority: 1,
          },
        ],
        synchronization: 'dialectical',
        conflictResolution: 'dialectical',
        organizationalPrinciple: 'distributed',
        emergenceLevel: 'basic',
      },
      status: 'initialized',
      progress: 0,
      dialecticalContext: {
        stage: 'essence',
        systemicLevel: 'complex',
        emergentProperties: ['mediation', 'reflection'],
      },
    };
  }

  /**
   * Create a Concept-level workflow (organic, self-organizing coordination)
   */
  public static createConceptWorkflow(
    name: string,
    conceptStructure: Record<string, any>,
  ): Workflow {
    return {
      id: `concept-workflow-${Date.now()}`,
      process: {
        id: `process-${Date.now()}`,
        name: `${name}-concept-process`,
        steps: [
          {
            id: 'universalize',
            name: 'Abstract to Universal',
            type: 'synthesis',
            dialecticalRole: 'thesis',
          },
          {
            id: 'particularize',
            name: 'Determine Particular',
            type: 'synthesis',
            dialecticalRole: 'antithesis',
            dependencies: ['universalize'],
          },
          {
            id: 'individualize',
            name: 'Concrete Individual',
            type: 'synthesis',
            dialecticalRole: 'synthesis',
            dependencies: ['universalize', 'particularize'],
          },
        ],
        approach: 'organic',
        adaptability: 'self-organizing',
      },
      coordination: {
        id: `coordination-${Date.now()}`,
        name: `${name}-concept-coordination`,
        rules: [
          {
            id: 'self-determination-rule',
            condition: 'concept-formation-needed',
            action: 'self-organize-process',
            priority: 3,
          },
          {
            id: 'dialectical-development-rule',
            condition: 'contradiction-in-concept',
            action: 'dialectical-resolution',
            priority: 2,
          },
          {
            id: 'organic-unity-rule',
            condition: 'elements-unified',
            action: 'emerge-higher-concept',
            priority: 1,
          },
        ],
        synchronization: 'organic',
        conflictResolution: 'transcendent',
        organizationalPrinciple: 'dialectical',
        emergenceLevel: 'complex',
      },
      status: 'initialized',
      progress: 0,
      dialecticalContext: {
        stage: 'concept',
        systemicLevel: 'dialectical',
        emergentProperties: [
          'self-determination',
          'systematic-unity',
          'organic-development',
        ],
      },
      metrics: {
        efficiency: 0.9,
        coherence: 0.95,
        adaptability: 0.9,
        emergence: 0.8,
      },
      // Attach concept structure to metadata so it participates in the schema and avoids unused param warnings
      metadata: {
        conceptStructure,
      },
    };
  }

  /**
   * Create an Absolute Idea workflow (transcendent, absolute coordination)
   */
  public static createAbsoluteWorkflow(
    name: string,
    systemContext: Record<string, any>,
  ): Workflow {
    return {
      id: `absolute-workflow-${Date.now()}`,
      process: {
        id: `process-${Date.now()}`,
        name: `${name}-absolute-process`,
        steps: [
          {
            id: 'absolute-knowledge',
            name: 'Achieve Absolute Knowledge',
            type: 'synthesis',
            dialecticalRole: 'synthesis',
          },
          {
            id: 'systematic-totality',
            name: 'Organize Systematic Totality',
            type: 'synthesis',
            dialecticalRole: 'synthesis',
            dependencies: ['absolute-knowledge'],
          },
          {
            id: 'free-self-determination',
            name: 'Exercise Free Self-Determination',
            type: 'synthesis',
            dialecticalRole: 'synthesis',
            dependencies: ['absolute-knowledge', 'systematic-totality'],
          },
        ],
        approach: 'organic',
        adaptability: 'transcendent',
      },
      coordination: {
        id: `coordination-${Date.now()}`,
        name: `${name}-absolute-coordination`,
        rules: [
          {
            id: 'absolute-self-organization',
            condition: 'system-requires-evolution',
            action: 'transcend-current-limits',
            priority: 1,
          },
        ],
        synchronization: 'organic',
        conflictResolution: 'transcendent',
        organizationalPrinciple: 'absolute',
        emergenceLevel: 'transcendent',
      },
      status: 'initialized',
      progress: 0,
      dialecticalContext: {
        stage: 'absolute-idea',
        systemicLevel: 'absolute',
        emergentProperties: [
          'absolute-self-knowledge',
          'complete-freedom',
          'systematic-totality',
          'transcendent-unity',
        ],
      },
      metrics: {
        efficiency: 1.0,
        coherence: 1.0,
        adaptability: 1.0,
        emergence: 1.0,
      },
      // Preserve provided system context on the workflow for downstream processors/inspection
      metadata: {
        systemContext,
      },
    };
  }
}

export default WorkflowSchema;
