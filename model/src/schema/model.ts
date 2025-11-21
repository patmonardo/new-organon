/**
 * Model Schema: State–Structure (Dharmic Substrate of Being)
 * ----------------------------------------------------------
 * Model represents Being + Kriya = BEC transformed into actionable substrate
 *
 * Philosophical Foundation:
 * - Model = Being made dharmic (actionable) rather than purely prajnic (theoretical)
 * - State: Form made actionable - the dynamic configuration that can be acted upon
 * - Structure: Entity made systematic - the organized schema enabling practical operations
 *
 * Kriya Integration:
 * - Model serves as the "dharmic substrate" - the actionable ground of experience
 * - Transforms theoretical Being (Form:Entity) into practical Being (State:Structure)
 * - Enables the Logic of Experience by making pure logic actionable
 *
 * Two-fold Structure:
 * - Relative Unconditioned: Model as practical substrate (dharmic mode)
 * - Conditioned Operation: State:Structure as Kriya-integrated Being
 */

import { z } from 'zod';

// Kriya Mode for Model components
export const ModelKriyaModeSchema = z.enum([
  'dharmic_substrate',      // Model as actionable ground
  'experiential_being',     // Being made experiential
  'practical_foundation'    // Foundation for action
]);

// State: Form made actionable through Kriya
export const StateSchema = z.object({
  id: z.string(),
  form_reference: z.string(),                        // Pure BEC Form reference
  actionable_configuration: z.record(z.string(), z.any()),  // Form made actionable
  kriya_mode: ModelKriyaModeSchema.default('dharmic_substrate'),
  dharmic_status: z.enum(['active', 'potential', 'transforming']).default('potential'),
  agent_context: z.string().optional(),              // Agent that can act on this state
  // Metadata for practical operations
  last_action: z.string().optional(),
  modification_history: z.array(z.string()).optional(),
});

// Structure: Entity made systematic through Kriya
export const StructureSchema = z.object({
  entity_reference: z.string(),                      // Pure BEC Entity reference
  systematic_organization: z.object({                // Entity made systematic
    included_properties: z.array(z.string()).optional(),
    excluded_properties: z.array(z.string()).optional(),
    operational_constraints: z.record(z.string(), z.any()).optional(),
  }),
  kriya_mode: ModelKriyaModeSchema.default('experiential_being'),
  practical_schema: z.object({                       // Schema for practical operations
    validation_rules: z.array(z.string()).optional(),
    transformation_methods: z.array(z.string()).optional(),
    agency_requirements: z.array(z.string()).optional(),
  }).optional(),
  // Systematic enablement
  supports_actions: z.array(z.string()).optional(),
  constrains_operations: z.array(z.string()).optional(),
});

// Model: Dharmic synthesis of actionable State and systematic Structure
export const ModelSchema = z.object({
  state: StateSchema,
  structure: StructureSchema,

  // Kriya Integration Metadata
  kriya_synthesis: z.object({
    being_source: z.string(),                        // Original BEC Being reference
    dharmic_transformation: z.string(),              // How Being became actionable
    practical_capabilities: z.array(z.string()),     // What this Model can do
    experiential_context: z.string().optional(),     // Context of experience
  }),

  // Logic of Experience metadata
  logic_mode: z.literal('dharmic').default('dharmic'),  // Always dharmic (actionable)
  unconditioned_status: z.literal('relative').default('relative'), // Relative Unconditioned
  theoretical_to_practical: z.boolean().default(true), // Marks the transformation

  // Operational metadata
  version: z.string().optional(),
  context: z.record(z.string(), z.any()).optional(),
}).catchall(z.any());
