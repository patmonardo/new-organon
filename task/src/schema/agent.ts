/**
 * Agent Schema: Capacity–Awareness (Active Essence in Agency)
 * ===========================================================
 *
 * In the TAW dialectical system, Agent represents the transformation of
 * Essence into active, self-aware agency:
 *
 *   - Capacity: The agent's powers, skills, and capabilities (Active Essence as potential)
 *   - Awareness: The agent's consciousness, perspective, and understanding (Active Essence as actuality)
 *
 * Agent = Capacity : Awareness
 *
 * Philosophical Note:
 * Agent is Essence in agency - not static mediation but dynamic self-aware action.
 * It represents the movement of Essence toward self-consciousness and autonomous activity.
 * The Capacity is Essence as power, the Awareness is Essence as consciousness.
 * This enables the dialectical bridge between mediated Logic and conscious Agency.
 */

import { z } from 'zod';

// Capacity: The agent's powers, skills, and capabilities (Active Essence as potential)
export const CapacitySchema = z.object({
  id: z.string(),
  name: z.string(),
  skills: z.array(z.string()), // What the agent can do
  tools: z.array(z.string()).optional(), // Resources available to the agent
  resources: z.record(z.string(), z.any()).optional(), // Materials, connections, etc.
  constraints: z.array(z.string()).optional(), // Limitations on the agent's capacity
  // Dialectical structure
  potentialLevel: z
    .enum(['reactive', 'deliberative', 'intuitive', 'absolute'])
    .default('reactive'),
  actualizationProgress: z.number().min(0).max(1).default(0),
});

// Awareness: The agent's consciousness, perspective, and understanding (Active Essence as actuality)
export const AwarenessSchema = z.object({
  id: z.string(),
  viewpoint: z.string(), // The agent's perspective or standpoint
  context: z.record(z.string(), z.any()), // Current situational awareness
  filters: z.record(z.string(), z.any()).optional(), // How the agent filters information
  history: z.array(z.record(z.string(), z.any())).optional(), // Past experiences
  // Consciousness levels
  consciousnessLevel: z
    .enum(['surface', 'depth', 'integral', 'non-dual'])
    .default('surface'),
  selfAwareness: z.boolean().default(false),
  // Dialectical movement
  reflectionCapacity: z
    .enum(['none', 'basic', 'dialectical', 'absolute'])
    .default('basic'),
});

// Agent: The synthesis of Capacity and Awareness
export const AgentSchema = z
  .object({
    id: z.string(),
    capacity: CapacitySchema,
    awareness: AwarenessSchema,
    // State management
    status: z
      .enum(['idle', 'active', 'learning', 'reflecting', 'transcendent'])
      .default('idle'),
    energy: z.number().min(0).max(1).default(1), // Available energy for action
    // Dialectical context
    dialecticalContext: z
      .object({
        stage: z.enum(['being', 'essence', 'concept', 'absolute-idea']),
        autonomyLevel: z.enum([
          'reactive',
          'deliberative',
          'dialectical',
          'absolute',
        ]),
        learningEnabled: z.boolean().default(true),
      })
      .optional(),
    // Temporal dimension
    createdAt: z.date().default(() => new Date()),
    lastActive: z.date().default(() => new Date()),
    // Metadata
    metadata: z.record(z.string(), z.any()).optional(),
  })
  .catchall(z.any());

export type Capacity = z.infer<typeof CapacitySchema>;
export type Awareness = z.infer<typeof AwarenessSchema>;
export type Agent = z.infer<typeof AgentSchema>;

/**
 * Agent Factory: Creates agents with dialectical consciousness levels
 */
export class AgentFactory {
  /**
   * Create a Being-level agent (reactive, immediate responses)
   */
  public static createBeingAgent(name: string, skills: string[]): Agent {
    const now = new Date();

    return {
      id: `being-agent-${Date.now()}`,
      capacity: {
        id: `capacity-${Date.now()}`,
        name: `${name}-capacity`,
        skills,
        potentialLevel: 'reactive',
        actualizationProgress: 0.2,
      },
      awareness: {
        id: `awareness-${Date.now()}`,
        viewpoint: 'immediate',
        context: { level: 'being', mode: 'reactive' },
        consciousnessLevel: 'surface',
        selfAwareness: false,
        reflectionCapacity: 'none',
      },
      status: 'idle',
      energy: 1,
      dialecticalContext: {
        stage: 'being',
        autonomyLevel: 'reactive',
        learningEnabled: true,
      },
      createdAt: now,
      lastActive: now,
    };
  }

  /**
   * Create an Essence-level agent (deliberative, reflective capabilities)
   */
  public static createEssenceAgent(
    name: string,
    skills: string[],
    reflectionCapacity: 'basic' | 'dialectical' = 'basic',
  ): Agent {
    const now = new Date();

    return {
      id: `essence-agent-${Date.now()}`,
      capacity: {
        id: `capacity-${Date.now()}`,
        name: `${name}-capacity`,
        skills: skills.concat(['mediate', 'reflect', 'synthesize']),
        potentialLevel: 'deliberative',
        actualizationProgress: 0.6,
      },
      awareness: {
        id: `awareness-${Date.now()}`,
        viewpoint: 'mediated',
        context: { level: 'essence', mode: 'deliberative' },
        consciousnessLevel: 'depth',
        selfAwareness: true,
        reflectionCapacity,
      },
      status: 'reflecting',
      energy: 0.8,
      dialecticalContext: {
        stage: 'essence',
        autonomyLevel: 'deliberative',
        learningEnabled: true,
      },
      createdAt: now,
      lastActive: now,
    };
  }

  /**
   * Create a Concept-level agent (intuitive, self-determining)
   */
  public static createConceptAgent(name: string, skills: string[]): Agent {
    const now = new Date();

    return {
      id: `concept-agent-${Date.now()}`,
      capacity: {
        id: `capacity-${Date.now()}`,
        name: `${name}-capacity`,
        skills: skills.concat([
          'self-determine',
          'universalize',
          'particularize',
          'individualize',
          'synthesize-dialectically',
        ]),
        potentialLevel: 'intuitive',
        actualizationProgress: 0.9,
      },
      awareness: {
        id: `awareness-${Date.now()}`,
        viewpoint: 'self-determining',
        context: { level: 'concept', mode: 'intuitive' },
        consciousnessLevel: 'integral',
        selfAwareness: true,
        reflectionCapacity: 'dialectical',
      },
      status: 'active',
      energy: 0.9,
      dialecticalContext: {
        stage: 'concept',
        autonomyLevel: 'dialectical',
        learningEnabled: true,
      },
      createdAt: now,
      lastActive: now,
    };
  }

  /**
   * Create an Absolute Idea agent (transcendent, absolute autonomy)
   */
  public static createAbsoluteAgent(
    name: string,
    systemContext: Record<string, any>,
  ): Agent {
    const now = new Date();

    return {
      id: `absolute-agent-${Date.now()}`,
      capacity: {
        id: `capacity-${Date.now()}`,
        name: `${name}-absolute-capacity`,
        skills: [
          'absolute-self-knowledge',
          'systematic-totality',
          'free-self-determination',
          'dialectical-transcendence',
          'reality-transformation',
        ],
        potentialLevel: 'absolute',
        actualizationProgress: 1.0,
      },
      awareness: {
        id: `awareness-${Date.now()}`,
        viewpoint: 'absolute',
        context: {
          level: 'absolute-idea',
          mode: 'transcendent',
          systemContext,
        },
        consciousnessLevel: 'non-dual',
        selfAwareness: true,
        reflectionCapacity: 'absolute',
      },
      status: 'transcendent',
      energy: 1.0,
      dialecticalContext: {
        stage: 'absolute-idea',
        autonomyLevel: 'absolute',
        learningEnabled: true,
      },
      createdAt: now,
      lastActive: now,
    };
  }
}

export default AgentSchema;
