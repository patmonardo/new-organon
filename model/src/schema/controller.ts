/**
 * Controller Schema: Action–Rule (Practical Synthesis of Concept)
 * ---------------------------------------------------------------
 * Controller represents Concept + Kriya = Active Concept in the Logic of Experience
 *
 * Philosophical Foundation:
 * - Controller = Concept made practical through Kriya synthesis
 * - Action: Morphism made executable - transformation that can be enacted
 * - Rule: Relation made normative - connection that governs practical operations
 *
 * Kriya Integration:
 * - Controller serves as "practical synthesis" - the active mediation of concepts
 * - Transforms theoretical Concept (Morphism:Relation) into practical Concept (Action:Rule)
 * - Enables the Logic of Experience by making pure concepts actionable
 *
 * Practical Reason:
 * - Controller embodies Kant's Practical Pure Reason - making absolute into relative unconditioned
 * - Action:Rule enables theoretical principles to become actionable in experience
 * - This is the locus where Logic becomes dharmic (actionable) rather than prajnic (pure)
 *
 * Two-fold Structure:
 * - Relative Unconditioned: Controller as practical synthesis (dharmic mode)
 * - Conditioned Operation: Action:Rule as Kriya-integrated Concept
 */

import { z } from 'zod';

// Kriya Mode for Controller components
export const ControllerKriyaModeSchema = z.enum([
  'practical_synthesis',     // Controller as active mediation
  'executable_concept',      // Concept made actionable
  'normative_governance'     // Governance through practical rules
]);

// Action: Morphism made executable through Kriya (Active Transformation)
export const ActionSchema = z.object({
  type: z.string(),                                  // Action type (create, update, transform, etc.)
  concept_source: z.string(),                        // Original BEC Concept.Morphism reference
  executable_transformation: z.any(),                // Morphism made actionable
  kriya_mode: ControllerKriyaModeSchema.default('practical_synthesis'),

  // Practical execution details
  execution_context: z.object({
    agent_requirements: z.array(z.string()).optional(), // What agents can execute this
    capability_dependencies: z.array(z.string()).optional(), // Required capabilities
    operational_constraints: z.record(z.string(), z.any()).optional(),
  }),

  // Active Transformation metadata
  transformation_details: z.object({
    source_state: z.string().optional(),             // What state this acts on
    target_state: z.string().optional(),             // What state this produces
    morphism_logic: z.string().optional(),           // The underlying morphism
    practical_method: z.string().optional(),         // How it's actually executed
  }),

  // Execution tracking
  execution_history: z.array(z.string()).optional(),
  last_executed_by: z.string().optional(),
  execution_results: z.array(z.any()).optional(),
});

// Rule: Relation made normative through Kriya (Active Governance)
export const RuleSchema = z.object({
  name: z.string(),                                  // Rule identifier
  concept_source: z.string(),                        // Original BEC Concept.Relation reference
  normative_governance: z.object({                   // Relation made normative
    condition: z.string().optional(),               // When this rule applies
    effect: z.string().optional(),                  // What this rule does
    governance_logic: z.string().optional(),        // How it governs
  }),
  kriya_mode: ControllerKriyaModeSchema.default('normative_governance'),

  // Practical governance details
  governance_context: z.object({
    applicable_actions: z.array(z.string()).optional(), // Which actions this governs
    constraint_type: z.enum(['permissive', 'restrictive', 'transformative']).optional(),
    enforcement_mechanism: z.string().optional(),    // How the rule is enforced
  }),

  // Active Governance metadata
  relational_foundation: z.object({
    source_relation: z.string().optional(),          // The underlying BEC relation
    practical_operationalization: z.string().optional(), // How relation became rule
    normative_authority: z.string().optional(),      // What gives this rule authority
  }),

  // Rule management
  priority: z.number().optional(),
  rule_owner: z.string().optional(),
  modification_history: z.array(z.string()).optional(),
});

// Controller: Practical synthesis of executable Action and normative Rule
export const ControllerSchema = z.object({
  action: ActionSchema,
  rule: RuleSchema,

  // Kriya Integration Metadata
  kriya_synthesis: z.object({
    concept_source: z.string(),                      // Original BEC Concept reference
    practical_transformation: z.string(),            // How Concept became actionable
    synthesis_capabilities: z.array(z.string()),     // What this Controller can synthesize
    agential_mediation: z.string().optional(),       // How this mediates between agents
  }),

  // Logic of Experience metadata
  logic_mode: z.literal('dharmic').default('dharmic'),  // Always dharmic (practical)
  unconditioned_status: z.literal('relative').default('relative'), // Relative Unconditioned
  theoretical_to_practical: z.boolean().default(true), // Marks the transformation

  // Practical Reason metadata
  practical_reason_mode: z.literal('active').default('active'), // Active vs passive reason
  synthesis_context: z.string().optional(),
  mediating_agent: z.string().optional(),

  // Active Concept metadata
  concept_mode: z.literal('active').default('active'),  // Active vs static concept
  transformation_capabilities: z.array(z.string()).optional(),
  governance_scope: z.array(z.string()).optional(),

  // Operational metadata
  version: z.string().optional(),
  context: z.record(z.string(), z.any()).optional(),
}).catchall(z.any());
