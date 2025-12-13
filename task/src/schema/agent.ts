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
export default AgentSchema;
