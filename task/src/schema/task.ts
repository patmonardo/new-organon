/**
 * Task Schema: Goal–Method (Active Being in Motion)
 * =================================================
 *
 * In the TAW dialectical system, Task represents the transformation of
 * Being into active, goal-directed motion:
 *
 *   - Goal: The telos or intended end-state (Active Being as destination)
 *   - Method: The means or approach for achieving the goal (Active Being as process)
 *
 * Task = Goal : Method
 *
 * Philosophical Note:
 * Task is Being in motion - not static existence but purposeful becoming.
 * It represents the first movement of Being toward agency and transformation.
 * The Goal is Being as telos, the Method is Being as kinesis.
 * This enables the dialectical bridge between static Logic and dynamic Agency.
 */

import { z } from 'zod';

// Goal: The telos or intended end-state (Active Being as destination)
export const GoalSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string().optional(),
  target: z.record(z.string(), z.any()), // The desired end-state
  form: z.string(), // The ontological type or form of the goal
  criteria: z.array(z.string()).optional(), // Success criteria
  priority: z
    .enum(['low', 'medium', 'high', 'absolute'])
    .optional()
    .default('medium'),
  deadline: z.date().optional(),
  // Dialectical metadata
  dialecticalStage: z
    .enum(['immediate', 'mediated', 'absolute'])
    .default('immediate'),
});

// Method: The means or approach for achieving the goal (Active Being as process)
export const MethodSchema = z.object({
  id: z.string(),
  name: z.string(),
  approach: z.enum(['sequential', 'parallel', 'dialectical', 'organic']),
  steps: z
    .array(
      z.object({
        id: z.string(),
        action: z.string(),
        dependencies: z.array(z.string()).optional(),
        resources: z.record(z.string(), z.any()).optional(),
      }),
    )
    .optional(),
  constraints: z.array(z.string()).optional(),
  resources: z.record(z.string(), z.any()).optional(),
  // Ontological structure
  entityType: z.string(),
  // Dialectical movement
  movement: z.enum(['thesis', 'antithesis', 'synthesis']).default('thesis'),
});

// Task: The synthesis of Goal and Method
export const TaskSchema = z
  .object({
    id: z.string(),
    goal: GoalSchema,
    method: MethodSchema,
    // State management
    status: z
      .enum(['pending', 'active', 'suspended', 'completed', 'failed'])
      .default('pending'),
    progress: z.number().min(0).max(1).default(0),
    // Dialectical context
    dialecticalContext: z
      .object({
        stage: z.enum(['being', 'essence', 'concept', 'absolute-idea']),
        contradictions: z.array(z.string()).optional(),
        syntheses: z.array(z.string()).optional(),
      })
      .optional(),
    // Temporal dimension
    createdAt: z.date().default(() => new Date()),
    updatedAt: z.date().default(() => new Date()),
    // Metadata
    metadata: z.record(z.string(), z.any()).optional(),
  })
  .catchall(z.any());

export type Goal = z.infer<typeof GoalSchema>;
export type Method = z.infer<typeof MethodSchema>;
export type Task = z.infer<typeof TaskSchema>;

/**
 * Task Factory: Creates tasks with dialectical awareness
 */
export class TaskFactory {
  /**
   * Create a Being-level task (immediate, simple goals)
   */
  public static createBeingTask(goal: string, method: string): Task {
    const now = new Date();

    return {
      id: `being-task-${Date.now()}`,
      goal: {
        id: `goal-${Date.now()}`,
        name: goal,
        target: { beingState: 'immediate' },
        form: 'being-form',
        priority: 'medium',
        dialecticalStage: 'immediate',
      },
      method: {
        id: `method-${Date.now()}`,
        name: method,
        approach: 'sequential',
        entityType: 'being-entity',
        movement: 'thesis',
      },
      status: 'pending',
      progress: 0,
      dialecticalContext: {
        stage: 'being',
      },
      createdAt: now,
      updatedAt: now,
    };
  }

  /**
   * Create an Essence-level task (mediated, reflective goals)
   */
  public static createEssenceTask(
    goal: string,
    method: string,
    mediation: Record<string, any>,
  ): Task {
    const now = new Date();

    return {
      id: `essence-task-${Date.now()}`,
      goal: {
        id: `goal-${Date.now()}`,
        name: goal,
        target: { essenceState: 'mediated', mediation },
        form: 'essence-form',
        priority: 'high',
        dialecticalStage: 'mediated',
      },
      method: {
        id: `method-${Date.now()}`,
        name: method,
        approach: 'dialectical',
        entityType: 'essence-entity',
        movement: 'antithesis',
        constraints: ['requires-mediation'],
      },
      status: 'pending',
      progress: 0,
      dialecticalContext: {
        stage: 'essence',
        contradictions: ['immediate-vs-mediated'],
      },
      createdAt: now,
      updatedAt: now,
    };
  }

  /**
   * Create a Concept-level task (self-determining, active goals)
   */
  public static createConceptTask(
    goal: string,
    method: string,
    concept: Record<string, any>,
  ): Task {
    const now = new Date();

    return {
      id: `concept-task-${Date.now()}`,
      goal: {
        id: `goal-${Date.now()}`,
        name: goal,
        target: { conceptState: 'self-determining', concept },
        form: 'concept-form',
        criteria: ['self-determination', 'universality', 'particularity'],
        priority: 'high',
        dialecticalStage: 'absolute',
      },
      method: {
        id: `method-${Date.now()}`,
        name: method,
        approach: 'organic',
        entityType: 'concept-entity',
        movement: 'synthesis',
        steps: [
          {
            id: 'universalize',
            action: 'abstract-to-universal',
          },
          {
            id: 'particularize',
            action: 'determine-particular',
            dependencies: ['universalize'],
          },
          {
            id: 'individualize',
            action: 'concrete-individual',
            dependencies: ['universalize', 'particularize'],
          },
        ],
      },
      status: 'pending',
      progress: 0,
      dialecticalContext: {
        stage: 'concept',
        syntheses: ['universal-particular-individual'],
      },
      createdAt: now,
      updatedAt: now,
    };
  }

  /**
   * Create an Absolute Idea task (complete self-aware system)
   */
  public static createAbsoluteIdeaTask(
    goal: string,
    systemContext: Record<string, any>,
  ): Task {
    const now = new Date();

    return {
      id: `absolute-idea-task-${Date.now()}`,
      goal: {
        id: `goal-${Date.now()}`,
        name: goal,
        target: {
          absoluteIdeaState: 'self-aware-system',
          systemContext,
          isAbsolute: true,
        },
        form: 'absolute-idea-form',
        criteria: [
          'complete-self-knowledge',
          'systematic-totality',
          'free-self-determination',
          'absolute-objectivity',
        ],
        priority: 'absolute',
        dialecticalStage: 'absolute',
      },
      method: {
        id: `method-${Date.now()}`,
        name: 'absolute-method',
        approach: 'organic',
        entityType: 'absolute-entity',
        movement: 'synthesis',
        steps: [
          {
            id: 'self-knowledge',
            action: 'achieve-complete-self-knowledge',
          },
          {
            id: 'systematic-totality',
            action: 'organize-systematic-totality',
            dependencies: ['self-knowledge'],
          },
          {
            id: 'free-determination',
            action: 'exercise-free-self-determination',
            dependencies: ['self-knowledge', 'systematic-totality'],
          },
        ],
      },
      status: 'pending',
      progress: 0,
      dialecticalContext: {
        stage: 'absolute-idea',
        syntheses: [
          'being-essence-concept',
          'logic-nature-spirit',
          'subjective-objective-absolute',
        ],
      },
      createdAt: now,
      updatedAt: now,
    };
  }
}

export default TaskSchema;
