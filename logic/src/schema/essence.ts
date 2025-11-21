/**
 * Essence Schema: Context–Property (Mediation with Kriya Dynamics)
 * ----------------------------------------------------------------
 * Essence represents mediation - the field of determination and possibility
 * Enhanced with Kriya dynamics for transformation into experiential manifestation
 *
 * Philosophical Foundation:
 * - Essence = The sphere of mediation, reflection, and essential determination
 * - Context: The environment or situation in which an entity is embedded
 * - Property: The attributes or qualities that characterize an entity within context
 *
 * Kriya Integration:
 * - Essence serves as the dynamic field for dharmic manifestation
 * - Context becomes experiential environment through Kriya
 * - Property becomes agential perspective through Kriya
 *
 * Dialectical Structure:
 * - Context (thesis): Environmental embedding
 * - Property (antithesis): Qualitative determination
 * - Essence (synthesis): Their mediated unity as field of possibility
 */

import { z } from "zod";

// Context: The environment or situation in which an entity is embedded
export const ContextSchema = z.object({
  id: z.string(),
  environmental_field: z.string(), // The field of embedding
  situational_parameters: z.record(z.string(), z.any()), // Environmental parameters
  kriya_dynamics: z.object({
    experiential_potentials: z.array(z.string()), // How context can become experiential
    manifestation_readiness: z
      .enum(["latent", "activating", "manifest"])
      .default("latent"),
    dharmic_resonance: z.array(z.string()), // What dharmic qualities this context supports
  }),

  // Environmental characteristics
  spatial_dimensions: z.record(z.string(), z.any()).optional(),
  temporal_horizons: z
    .object({
      past_influences: z.array(z.string()).optional(),
      present_conditions: z.array(z.string()).optional(),
      future_potentials: z.array(z.string()).optional(),
    })
    .optional(),
  relational_matrix: z.array(z.string()).optional(), // Network of relations

  // Context metadata
  scope_level: z
    .enum(["local", "regional", "global", "universal"])
    .default("local"),
  influence_strength: z.number().min(0).max(10).default(5),
});

// Property: The attributes or qualities that characterize an entity within context
export const PropertySchema = z.object({
  id: z.string(),
  context_reference: z.string(), // Reference to the Context this property exists within
  qualitative_character: z.string(), // The essential character of this property
  kriya_manifestation: z.object({
    agential_perspective: z.record(z.string(), z.any()), // How property manifests as perspective
    filtering_mechanisms: z.array(z.string()), // How property filters experience
    practical_expressions: z.array(z.string()), // How property expresses practically
  }),

  // Property characteristics
  attribute_values: z.record(z.string(), z.any()),
  determination_relations: z.array(z.string()).optional(), // How this property is determined
  quality_gradations: z
    .object({
      intensity_levels: z.array(z.string()).optional(),
      variation_ranges: z.record(z.string(), z.any()).optional(),
      modal_expressions: z.array(z.string()).optional(),
    })
    .optional(),

  // Property metadata
  determinacy_level: z
    .enum(["vague", "specific", "precise"])
    .default("specific"),
  contextual_dependency: z.number().min(0).max(10).default(7), // How context-dependent
});

// Essence: The dialectical unity of Context and Property as mediation field
export const EssenceSchema = z.object({
  context: ContextSchema,
  property: PropertySchema,

  // Dialectical unity metadata
  dialectical_mediation: z.object({
    context_property_synthesis: z.string(), // How Context and Property mediate each other
    reflection_dynamics: z.string(), // The reflective character of essence
    possibility_field: z.string(), // How essence opens field of possibilities
  }),

  // Kriya transformation dynamics
  kriya_field: z.object({
    manifestation_readiness: z
      .enum(["potential", "dynamic", "actualized"])
      .default("potential"),
    experiential_transformation: z.string().optional(), // How Essence becomes experiential
    agential_evolution: z.string().optional(), // How Essence develops agential capacities
  }),

  // Essential characteristics
  mediation_quality: z
    .enum(["immediate", "reflected", "absolute"])
    .default("reflected"),
  determination_mode: z
    .enum(["external", "internal", "self-determining"])
    .default("internal"),

  // Metadata
  logical_priority: z.number().min(2).max(10).default(5), // Essence mediates Being and Concept
  complexity_level: z.enum(["simple", "complex", "organic"]).default("complex"),
  version: z.string().optional(),
  //context: z.record(z.string(), z.any()).optional(),
});

// Factory for creating Essence instances with dialectical mediation
export class EssenceFactory {
  /**
   * Create a reflective Essence with basic mediation
   */
  public static createReflectiveEssence(
    environmentalField: string,
    qualitativeCharacter: string,
    mediationContext: Record<string, any>
  ): z.infer<typeof EssenceSchema> {
    return {
      context: {
        id: `context-${Date.now()}`,
        environmental_field: environmentalField,
        situational_parameters: mediationContext,
        kriya_dynamics: {
          experiential_potentials: [
            "environmental-awareness",
            "contextual-sensitivity",
          ],
          manifestation_readiness: "latent",
          dharmic_resonance: [
            "contextual-dharma",
            "environmental-responsiveness",
          ],
        },
        scope_level: "local",
        influence_strength: 5,
      },

      property: {
        id: `property-${Date.now()}`,
        context_reference: `context-${Date.now()}`,
        qualitative_character: qualitativeCharacter,
        kriya_manifestation: {
          agential_perspective: {
            perspective_type: "reflective",
            clarity_level: "moderate",
          },
          filtering_mechanisms: [
            "contextual-filtering",
            "qualitative-selection",
          ],
          practical_expressions: [
            "property-manifestation",
            "contextual-adaptation",
          ],
        },
        attribute_values: {
          quality: qualitativeCharacter,
          context_binding: environmentalField,
        },
        determinacy_level: "specific",
        contextual_dependency: 7,
      },

      dialectical_mediation: {
        context_property_synthesis:
          "Context and Property mutually determine through reflective mediation",
        reflection_dynamics:
          "Essential reflection where context determines property and property modifies context",
        possibility_field:
          "Opens field of essential possibilities through contextual-qualitative mediation",
      },

      kriya_field: {
        manifestation_readiness: "potential",
      },

      mediation_quality: "reflected",
      determination_mode: "internal",
      logical_priority: 5,
      complexity_level: "complex",
    };
  }

  /**
   * Create an absolute Essence with self-determining mediation
   */
  public static createAbsoluteEssence(
    universalField: string,
    selfDeterminingQuality: string,
    absoluteContext: Record<string, any>
  ): z.infer<typeof EssenceSchema> {
    return {
      context: {
        id: `context-${Date.now()}`,
        environmental_field: universalField,
        situational_parameters: absoluteContext,
        kriya_dynamics: {
          experiential_potentials: [
            "universal-awareness",
            "absolute-contextuality",
            "total-environmental-integration",
          ],
          manifestation_readiness: "manifest",
          dharmic_resonance: [
            "universal-dharma",
            "absolute-responsiveness",
            "cosmic-harmonization",
          ],
        },
        spatial_dimensions: { scope: "universal", penetration: "absolute" },
        temporal_horizons: {
          past_influences: ["eternal-principles"],
          present_conditions: ["absolute-presence"],
          future_potentials: ["infinite-possibilities"],
        },
        scope_level: "universal",
        influence_strength: 10,
      },

      property: {
        id: `property-${Date.now()}`,
        context_reference: `context-${Date.now()}`,
        qualitative_character: selfDeterminingQuality,
        kriya_manifestation: {
          agential_perspective: {
            perspective_type: "absolute",
            clarity_level: "perfect",
            universal_scope: true,
          },
          filtering_mechanisms: [
            "universal-discrimination",
            "absolute-selection",
            "perfect-judgment",
          ],
          practical_expressions: [
            "absolute-manifestation",
            "universal-adaptation",
            "perfect-expression",
          ],
        },
        attribute_values: {
          quality: selfDeterminingQuality,
          context_binding: universalField,
          self_determination: true,
        },
        quality_gradations: {
          intensity_levels: ["absolute"],
          modal_expressions: ["necessary", "universal", "eternal"],
        },
        determinacy_level: "precise",
        contextual_dependency: 0, // Self-determining
      },

      dialectical_mediation: {
        context_property_synthesis:
          "Absolute synthesis where Context and Property are identical in their self-determination",
        reflection_dynamics:
          "Absolute reflection - essence reflects itself as the totality of its determinations",
        possibility_field:
          "Opens the absolute field of all essential possibilities through self-determining mediation",
      },

      kriya_field: {
        manifestation_readiness: "actualized",
        experiential_transformation:
          "Complete integration of essence and experience",
        agential_evolution: "Perfect agential capacities - universal agent",
      },

      mediation_quality: "absolute",
      determination_mode: "self-determining",
      logical_priority: 8,
      complexity_level: "organic",
    };
  }
}
