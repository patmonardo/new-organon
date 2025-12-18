/**
 * Workflow Schema (Schema-first)
 * ==============================
 *
 * This module is intentionally *definitions-only*:
 * - Zod schemas + inferred types
 * - No factory classes, no runtime helpers
 */

import { z } from 'zod';

export const ProcessStepSchema = z.object({
  id: z.string(),
  name: z.string(),
  type: z.enum(['task', 'decision', 'transformation', 'synthesis']),
  dependencies: z.array(z.string()).optional(),
  /** Expected duration in milliseconds (optional). */
  duration: z.number().optional(),
  resources: z.record(z.string(), z.any()).optional(),
  dialecticalRole: z.enum(['thesis', 'antithesis', 'synthesis']).default('thesis'),
});

// Process: active sequence of transformations
export const ProcessSchema = z.object({
  id: z.string(),
  name: z.string(),
  steps: z.array(ProcessStepSchema),
  approach: z.enum(['linear', 'parallel', 'dialectical', 'organic']).default('linear'),
  adaptability: z.enum(['fixed', 'flexible', 'self-organizing', 'transcendent']).default('fixed'),
});

export const CoordinationRuleSchema = z.object({
  id: z.string(),
  condition: z.string(),
  action: z.string(),
  priority: z.number().default(1),
});

// Coordination: organizational principle governing the process
export const CoordinationSchema = z.object({
  id: z.string(),
  name: z.string(),
  rules: z.array(CoordinationRuleSchema),
  synchronization: z.enum(['loose', 'tight', 'dialectical', 'organic']).default('loose'),
  conflictResolution: z.enum(['hierarchy', 'consensus', 'dialectical', 'transcendent']).default('hierarchy'),
  organizationalPrinciple: z.enum(['centralized', 'distributed', 'dialectical', 'absolute']).default('centralized'),
  emergenceLevel: z.enum(['none', 'basic', 'complex', 'transcendent']).default('none'),
});

export const WorkflowDialecticalContextSchema = z.object({
  stage: z.enum(['being', 'essence', 'concept', 'absolute-idea']),
  systemicLevel: z.enum(['simple', 'complex', 'dialectical', 'absolute']),
  emergentProperties: z.array(z.string()).optional(),
});

export const WorkflowMetricsSchema = z.object({
  efficiency: z.number().min(0).max(1).optional(),
  coherence: z.number().min(0).max(1).optional(),
  adaptability: z.number().min(0).max(1).optional(),
  emergence: z.number().min(0).max(1).optional(),
});

// Workflow: synthesis of Process and Coordination
export const WorkflowSchema = z
  .object({
    id: z.string(),
    process: ProcessSchema,
    coordination: CoordinationSchema,
    status: z.enum(['initialized', 'running', 'paused', 'completed', 'failed', 'transcended']).default('initialized'),
    progress: z.number().min(0).max(1).default(0),
    dialecticalContext: WorkflowDialecticalContextSchema.optional(),
    metrics: WorkflowMetricsSchema.optional(),
    startedAt: z.date().optional(),
    completedAt: z.date().optional(),
    metadata: z.record(z.string(), z.any()).optional(),
  })
  .catchall(z.any());

export type ProcessStep = z.infer<typeof ProcessStepSchema>;
export type Process = z.infer<typeof ProcessSchema>;
export type CoordinationRule = z.infer<typeof CoordinationRuleSchema>;
export type Coordination = z.infer<typeof CoordinationSchema>;
export type Workflow = z.infer<typeof WorkflowSchema>;

// =====================================================================
// WORKFLOW UNITY (W = Concept + Control Surface)
// =====================================================================

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
  /**
   * Action identifier (often a syscall/tool name).
   * Together with Coordination rules this forms an Action:Rule control surface.
   */
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
  // Historical naming: "controller" is the workflow control surface (step program).
  controller: WorkflowControllerSchema,
});

export type WorkflowConcept = z.infer<typeof WorkflowConceptSchema>;
export type WorkflowControllerStep = z.infer<typeof WorkflowControllerStepSchema>;
export type WorkflowController = z.infer<typeof WorkflowControllerSchema>;
export type WorkflowUnity = z.infer<typeof WorkflowUnitySchema>;

export default WorkflowSchema;
