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
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';

// Core mirrors ShapeCore
export const AspectCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type AspectCore = z.infer<typeof AspectCore>;

export const AspectState = BaseState;
export type AspectState = z.infer<typeof AspectState>;

// Open signature like ShapeSignature
export const AspectSignature = z.object({}).catchall(z.any());
export type AspectSignature = z.infer<typeof AspectSignature>;

// Facets structure for spectral/relational data
export const AspectFacets = z.object({
  // Core dialectical state
  dialecticState: z.any().optional(),

  // Current phase
  phase: z.string().optional(),

  // Spectrum: range of appearing through polarities
  spectrum: z.object({
    poles: z.array(z.object({
      name: z.string(),
      definition: z.string(),
      oppositeTo: z.string().optional(),
    })),
    range: z.number(), // Number of poles
    dialectical: z.boolean(), // Whether poles are dialectically opposed
  }).optional(),

  // Essential Relation: the underlying connection
  essentialRelation: z.object({
    spectrum: z.any(), // Reference to spectrum above
    connections: z.array(z.object({
      from: z.string(),
      to: z.string().optional(),
      relation: z.string().optional(),
      type: z.string(),
    })),
    appearing: z.any(), // How this relation appears
    groundedIn: z.string().optional(), // Morph/Ground ID
  }).optional(),

  // Relations: essential connections between moments
  relations: z.array(z.object({
    from: z.string(),
    to: z.string().optional(),
    relation: z.string().optional(),
    type: z.string(),
  })).optional(),

  // Appearing: mode of manifestation
  appearing: z.object({
    mode: z.enum(['immanent', 'externality', 'reflection', 'passover']),
    triggers: z.array(z.string()),
    effects: z.array(z.string()),
  }).optional(),

  // Constraints from invariants
  constraints: z.array(z.object({
    id: z.string(),
    constraint: z.string(),
    predicate: z.string().optional(),
  })).optional(),

  // Evaluation context
  context: z.any().optional(),

  // Additional aspect-specific facets via catchall
}).catchall(z.any());
export type AspectFacets = z.infer<typeof AspectFacets>;

// ==========================================
// ASPECT SHAPE (Pure Form for Neo4j)
// ==========================================
export const AspectShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),

  // Signature: operational interface
  signature: AspectSignature.optional(),

  // Facets: spectral/relational structure
  facets: AspectFacets.optional(),

  // State metadata
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: z.record(z.string(), z.unknown()).optional(),

  // Timestamps
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
export type AspectShape = z.infer<typeof AspectShapeSchema>;

// ==========================================
// ASPECT DOCUMENT (Envelope)
// ==========================================
export const AspectDoc = z.object({
  core: AspectCore,
  state: BaseState.default({}),
  signature: AspectSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
  aspect: AspectShapeSchema.optional(), // EMBED AspectShape
});

export const AspectSchema = BaseSchema.extend({
  shape: AspectDoc,
});
export type Aspect = z.infer<typeof AspectSchema>;

// Helpers
function genId() {
  return `aspect:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreateAspectInput = {
  id?: string;
  type: string;
  name?: string;
  signature?: z.input<typeof AspectSignature>;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createAspect(input: CreateAspectInput): Aspect {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
    },
  };
  return AspectSchema.parse(draft);
}

type UpdateAspectPatch = Partial<{
  core: Partial<z.output<typeof AspectCore>>;
  state: Partial<z.output<typeof BaseState>>;
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateAspect(doc: Aspect, patch: UpdateAspectPatch): Aspect {
  const nextSignature =
    patch.signature === null
      ? undefined
      : patch.signature !== undefined
      ? patch.signature
      : doc.shape.signature;

  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as AspectCore), ...(patch.core ?? {}) },
      state: {
        ...(doc.shape.state as z.output<typeof BaseState>),
        ...(patch.state ?? {}),
      },
      signature: nextSignature,
      facets: patch.facets ?? doc.shape.facets,
    },
    version: patch.version ?? doc.version,
    ext: patch.ext ?? doc.ext,
    revision: (doc.revision ?? 0) + 1,
  };
  return AspectSchema.parse(next);
}

/**
 * Helper: Get spectrum from Aspect facets
 */
export function getAspectSpectrum(aspect: Aspect): any | undefined {
  return (aspect.shape.facets as any)?.spectrum;
}

/**
 * Helper: Get essential relation from Aspect facets
 */
export function getAspectEssentialRelation(aspect: Aspect): any | undefined {
  return (aspect.shape.facets as any)?.essentialRelation;
}

/**
 * Helper: Get appearing mode from Aspect facets
 */
export function getAspectAppearing(aspect: Aspect): any | undefined {
  return (aspect.shape.facets as any)?.appearing;
}

