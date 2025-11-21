/**
 * View Schema: Representation–Perspective (Experiential Manifestation of Essence)
 * -------------------------------------------------------------------------------
 * View represents Essence + Kriya = Active Essence in the Logic of Experience
 *
 * Philosophical Foundation:
 * - View = Essence made experiential through Kriya manifestation
 * - Representation: Context made visible - how the practical substrate appears
 * - Perspective: Property made perspectival - the agential viewpoint filter
 *
 * Kriya Integration:
 * - View serves as "experiential manifestation" - how dharmic substrate appears
 * - Transforms theoretical Essence (Context:Property) into practical Essence (Representation:Perspective)
 * - Enables the Logic of Experience by rendering actionable logic visible
 *
 * Sanskrit Resonances:
 * - Pratyaya (Representation): The content presented to awareness, now actionably rendered
 * - Drishti (Perspective): The standpoint of seeing, now agent-integrated viewpoint
 *
 * Two-fold Structure:
 * - Relative Unconditioned: View as experiential manifestation (dharmic mode)
 * - Conditioned Operation: Representation:Perspective as Kriya-integrated Essence
 */

import { z } from 'zod';

// Kriya Mode for View components
export const ViewKriyaModeSchema = z.enum([
  'experiential_manifestation',  // View as experience rendering
  'dharmic_appearance',          // Essence made visible
  'agential_perspective'         // Perspective integrated with agency
]);

// Representation: Context made visible through Kriya (Active Pratyaya)
export const RepresentationSchema = z.object({
  type: z.string(),                                  // Rendering type (table, graph, etc.)
  essence_source: z.string(),                        // Original BEC Essence.Context reference
  experiential_rendering: z.any(),                   // Context made visible/actionable
  kriya_mode: ViewKriyaModeSchema.default('experiential_manifestation'),

  // Practical manifestation details
  visibility_context: z.object({
    agent_accessible: z.boolean().default(true),
    interaction_modes: z.array(z.string()).optional(), // How agents can interact
    experiential_qualities: z.record(z.string(), z.any()).optional(),
  }),

  // Active Pratyaya (content presented to awareness)
  awareness_content: z.object({
    presented_data: z.any(),                          // What is shown
    contextual_meaning: z.string().optional(),        // How it means in context
    actionable_elements: z.array(z.string()).optional(), // What can be acted upon
  }),

  // Rendering metadata
  last_updated: z.string().optional(),
  rendering_agent: z.string().optional(),
});

// Perspective: Property made perspectival through Kriya (Active Drishti)
export const PerspectiveSchema = z.object({
  viewpoint: z.string(),                             // The seeing standpoint
  essence_source: z.string(),                        // Original BEC Essence.Property reference
  agential_filters: z.record(z.string(), z.any()).optional(), // Agent-integrated filtering
  kriya_mode: ViewKriyaModeSchema.default('agential_perspective'),

  // Practical perspective details
  perspective_context: z.object({
    agent_identity: z.string().optional(),           // Which agent's perspective
    capability_constraints: z.array(z.string()).optional(), // What this perspective can see
    experiential_stance: z.string().optional(),      // How the agent sees
  }),

  // Active Drishti (mode of seeing)
  seeing_mode: z.object({
    focal_properties: z.array(z.string()).optional(), // What properties are focused
    filtering_logic: z.string().optional(),          // How filtering works
    perspectival_transform: z.string().optional(),   // How perspective transforms seeing
  }),

  // Perspective metadata
  perspective_owner: z.string().optional(),
  filtering_history: z.array(z.string()).optional(),
});

// View: Experiential synthesis of visible Representation and perspectival Perspective
export const ViewSchema = z.object({
  representation: RepresentationSchema,
  perspective: PerspectiveSchema,

  // Kriya Integration Metadata
  kriya_synthesis: z.object({
    essence_source: z.string(),                      // Original BEC Essence reference
    experiential_transformation: z.string(),         // How Essence became experiential
    manifestation_capabilities: z.array(z.string()), // What this View can manifest
    agential_integration: z.string().optional(),     // How agents interact with this view
  }),

  // Logic of Experience metadata
  logic_mode: z.literal('dharmic').default('dharmic'),  // Always dharmic (experiential)
  unconditioned_status: z.literal('relative').default('relative'), // Relative Unconditioned
  theoretical_to_experiential: z.boolean().default(true), // Marks the transformation

  // Active Essence metadata
  essence_mode: z.literal('active').default('active'),   // Active vs passive essence
  experiential_context: z.string().optional(),
  manifestation_agent: z.string().optional(),

  // Operational metadata
  version: z.string().optional(),
  context: z.record(z.string(), z.any()).optional(),
}).catchall(z.any());
