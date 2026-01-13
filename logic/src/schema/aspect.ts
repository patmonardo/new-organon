/**
 * Aspect Schema - "Spectral Relation / Appearance"
 *
 * Aspect represents the Spectral Theory of Relation - how essences appear in existence.
 * It is the range of appearing, the spectrum through which relations manifest.
 *
 * Dialectical Role:
 * - Embodies Fichte's concept of "appearance" (Erscheinung)
 * - Represents essential relation as spectrum of polarities
 * - Connects Entity and Property through the lens of Morph (Ground)
 * - Shows how relations appear rather than what they are in-themselves
 *
 * Relationship to Shape Engines:
 * - Used by AspectEngine to manage relational appearances
 * - Aspect.facets.spectrum contains poles and dialectical range
 * - Aspect.facets.essentialRelation shows the underlying connection
 * - Aspect.facets.appearing describes mode of manifestation
 *
 * Shape Engine Pattern:
 * - Facets: Spectral structure (spectrum, relations, appearing)
 * - Signature: Operational interface (relational operations)
 * - State: Runtime status with spectral metadata
 */

import { z } from 'zod';
import { BaseState, Type, Label } from './base';

// Facets structure for spectral/relational data
export const AspectFacets = z
  .object({
    // Core dialectical state
    dialecticState: z.any().optional(),

    // Current phase
    phase: z.string().optional(),

    // Spectrum: range of appearing through polarities
    spectrum: z
      .object({
        poles: z.array(
          z.object({
            name: z.string(),
            definition: z.string(),
            oppositeTo: z.string().optional(),
          }),
        ),
        range: z.number(), // Number of poles
        dialectical: z.boolean(), // Whether poles are dialectically opposed
      })
      .optional(),

    // Essential Relation: the underlying connection
    essentialRelation: z
      .object({
        spectrum: z.any(), // Reference to spectrum above
        connections: z.array(
          z.object({
            from: z.string(),
            to: z.string().optional(),
            relation: z.string().optional(),
            type: z.string(),
          }),
        ),
        appearing: z.any(), // How this relation appears
        groundedIn: z.string().optional(), // Morph/Ground ID
      })
      .optional(),

    // Relations: essential connections between moments
    relations: z
      .array(
        z.object({
          from: z.string(),
          to: z.string().optional(),
          relation: z.string().optional(),
          type: z.string(),
        }),
      )
      .optional(),

    // Appearing: mode of manifestation
    appearing: z
      .object({
        mode: z.enum(['immanent', 'externality', 'reflection', 'passover']),
        triggers: z.array(z.string()),
        effects: z.array(z.string()),
      })
      .optional(),

    // Constraints from invariants
    constraints: z
      .array(
        z.object({
          id: z.string(),
          constraint: z.string(),
          predicate: z.string().optional(),
        }),
      )
      .optional(),

    // Evaluation context
    context: z.any().optional(),

    // Additional aspect-specific facets via catchall
  })
  .catchall(z.any());
export type AspectFacets = z.infer<typeof AspectFacets>;

// ==========================================
// ASPECT SHAPE (Pure Form for Neo4j)
// ==========================================
export const AspectShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),

  state: BaseState.optional().default({}),
  signature: z.object({}).catchall(z.any()).optional(),
  facets: AspectFacets.optional().default({}),

  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: z.record(z.string(), z.unknown()).optional(),

  createdAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
  updatedAt: z
    .number()
    .optional()
    .default(() => Date.now())
    .optional(),
});
export type AspectShapeRepo = z.infer<typeof AspectShapeSchema>;
export type AspectShape = AspectShapeRepo;
export const AspectSchema = AspectShapeSchema;
export type Aspect = AspectShapeRepo;

export function getAspectSpectrum(aspect: Aspect): any | undefined {
  return (aspect.facets as any)?.spectrum;
}

export function getAspectEssentialRelation(aspect: Aspect): any | undefined {
  return (aspect.facets as any)?.essentialRelation;
}

export function getAspectAppearing(aspect: Aspect): any | undefined {
  return (aspect.facets as any)?.appearing;
}
