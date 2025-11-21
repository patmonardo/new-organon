/**
 * Being Schema: Form–Entity (Pure Presence with Kriya Potential)
 * -------------------------------------------------------------
 * Being represents pure presence or existence - the ground of actuality
 * Enhanced with Kriya potential for transformation into actionable substrate
 *
 * Philosophical Foundation:
 * - Being = Pure presence, the immediate ground of all determination
 * - Form: The shape, pattern, or structure that gives Being its identity
 * - Entity: The particular, individuated instance of Being
 *
 * Kriya Integration:
 * - Being serves as the substrate for dharmic transformation
 * - Form becomes actionable configuration through Kriya
 * - Entity becomes systematic organization through Kriya
 *
 * Dialectical Structure:
 * - Form (thesis): Pure structural pattern
 * - Entity (antithesis): Individuated instance
 * - Being (synthesis): Their living unity as pure presence
 */

import { z } from 'zod';

// Form: The shape, pattern, or structure that gives Being its identity
export const FormSchema = z.object({
  id: z.string(),
  pattern_structure: z.string(),                   // The structural pattern itself
  identity_signature: z.string(),                  // What makes this form unique
  kriya_potential: z.object({
    actionable_aspects: z.array(z.string()),       // How this form can become actionable
    transformation_readiness: z.enum(['dormant', 'emerging', 'active']).default('dormant'),
    dharmic_compatibility: z.array(z.string()),    // What dharmic operations this supports
  }),

  // Pure form characteristics
  geometric_properties: z.record(z.string(), z.any()).optional(),
  logical_constraints: z.array(z.string()).optional(),
  structural_invariants: z.array(z.string()).optional(),

  // Metadata
  abstraction_level: z.enum(['concrete', 'generic', 'universal']).default('generic'),
  complexity_degree: z.number().min(0).max(10).default(5),
});

// Entity: The particular, individuated instance of Being
export const EntitySchema = z.object({
  id: z.string(),
  form_reference: z.string(),                      // Reference to the Form this instantiates
  individuation_principle: z.string(),             // What makes this entity particular
  kriya_integration: z.object({
    systematic_organization: z.record(z.string(), z.any()), // How entity organizes systematically
    operational_constraints: z.array(z.string()),  // What constraints enable operations
    practical_capabilities: z.array(z.string()),   // What this entity can practically do
  }),

  // Particular entity characteristics
  concrete_properties: z.record(z.string(), z.any()),
  contextual_relations: z.array(z.string()).optional(),
  temporal_aspects: z.object({
    creation_context: z.string().optional(),
    modification_history: z.array(z.string()).optional(),
    persistence_conditions: z.array(z.string()).optional(),
  }).optional(),

  // Instance metadata
  instantiation_context: z.string().optional(),
  uniqueness_factors: z.array(z.string()).optional(),
});

// Being: The dialectical unity of Form and Entity as pure presence
export const BeingSchema = z.object({
  form: FormSchema,
  entity: EntitySchema,

  // Dialectical unity metadata
  dialectical_synthesis: z.object({
    form_entity_unity: z.string(),                 // How Form and Entity are unified in Being
    pure_presence_character: z.string(),           // The character of pure presence
    actuality_ground: z.string(),                  // How this Being grounds actuality
  }),

  // Kriya transformation potential
  kriya_substrate: z.object({
    dharmic_readiness: z.enum(['potential', 'emerging', 'actualized']).default('potential'),
    actionable_transformation: z.string().optional(), // How Being can become actionable
    systematic_evolution: z.string().optional(),   // How Being can become systematic
  }),

  // Pure Being characteristics
  presence_quality: z.enum(['immediate', 'mediated', 'absolute']).default('immediate'),
  determination_level: z.enum(['indeterminate', 'determinate', 'self-determining']).default('indeterminate'),

  // Metadata
  logical_priority: z.number().min(1).max(10).default(1), // Being is foundational
  version: z.string().optional(),
  context: z.record(z.string(), z.any()).optional(),
});

// Factory for creating Being instances with dialectical structure
export class BeingFactory {
  /**
   * Create a basic Being with minimal determination
   */
  public static createImmediateBeing(
    formPattern: string,
    entityContext: Record<string, any>
  ): z.infer<typeof BeingSchema> {
    return {
      form: {
        id: `form-${Date.now()}`,
        pattern_structure: formPattern,
        identity_signature: `immediate-form-${formPattern}`,
        kriya_potential: {
          actionable_aspects: ['basic-structure'],
          transformation_readiness: 'dormant',
          dharmic_compatibility: ['simple-operations'],
        },
        abstraction_level: 'concrete',
        complexity_degree: 1,
      },

      entity: {
        id: `entity-${Date.now()}`,
        form_reference: `form-${Date.now()}`,
        individuation_principle: 'immediate-instantiation',
        kriya_integration: {
          systematic_organization: entityContext,
          operational_constraints: ['basic-constraints'],
          practical_capabilities: ['existence'],
        },
        concrete_properties: entityContext,
      },

      dialectical_synthesis: {
        form_entity_unity: 'Immediate unity of pattern and instance as pure presence',
        pure_presence_character: 'Indeterminate immediacy',
        actuality_ground: 'Foundation for all further determination',
      },

      kriya_substrate: {
        dharmic_readiness: 'potential',
      },

      presence_quality: 'immediate',
      determination_level: 'indeterminate',
      logical_priority: 1,
    };
  }

  /**
   * Create a determinate Being with developed Form-Entity dialectic
   */
  public static createDeterminateBeing(
    formPattern: string,
    entityProperties: Record<string, any>,
    dialecticalContext: string
  ): z.infer<typeof BeingSchema> {
    return {
      form: {
        id: `form-${Date.now()}`,
        pattern_structure: formPattern,
        identity_signature: `determinate-form-${formPattern}`,
        kriya_potential: {
          actionable_aspects: ['structural-operations', 'pattern-transformations'],
          transformation_readiness: 'emerging',
          dharmic_compatibility: ['systematic-operations', 'practical-transformations'],
        },
        geometric_properties: { complexity: 'moderate', regularity: 'structured' },
        logical_constraints: ['consistency', 'coherence'],
        abstraction_level: 'generic',
        complexity_degree: 5,
      },

      entity: {
        id: `entity-${Date.now()}`,
        form_reference: `form-${Date.now()}`,
        individuation_principle: 'determinate-specification',
        kriya_integration: {
          systematic_organization: entityProperties,
          operational_constraints: ['structural-integrity', 'logical-consistency'],
          practical_capabilities: ['determinate-operations', 'relational-functioning'],
        },
        concrete_properties: entityProperties,
        temporal_aspects: {
          creation_context: dialecticalContext,
          persistence_conditions: ['structural-stability', 'relational-coherence'],
        },
      },

      dialectical_synthesis: {
        form_entity_unity: 'Determinate unity where pattern and instance mutually determine each other',
        pure_presence_character: 'Mediated presence through internal relations',
        actuality_ground: 'Structured foundation enabling essential determinations',
      },

      kriya_substrate: {
        dharmic_readiness: 'emerging',
        actionable_transformation: 'Ready for systematic dharmic operations',
        systematic_evolution: 'Can evolve into practical substrate',
      },

      presence_quality: 'mediated',
      determination_level: 'determinate',
      logical_priority: 3,
    };
  }
}
