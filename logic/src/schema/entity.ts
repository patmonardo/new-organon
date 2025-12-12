/**
 * Entity Schema - "Fact → Existence"
 *
 * Entity represents Fact coming into Existence - the dialectical state of actualization.
 * It is the concrete instantiation of abstract logical structures.
 *
 * Dialectical Role:
 * - Embodies the passage from essence to existence
 * - Stores dialectical state in facets (the "in-itself")
 * - Exposes operational interface via signature (the "for-another")
 * - Represents "what is" as opposed to "what could be"
 *
 * Relationship to Shape Engines:
 * - Used by EntityEngine to manage existential instances
 * - Entity.facets.dialecticState contains the embedded DialecticState
 * - Entity.signature contains moments as operational interface
 *
 * Shape Engine Pattern:
 * - Facets: Internal dialectical structure (moments, invariants, phase)
 * - Signature: External operational interface (what this entity does)
 * - State: Runtime status and metadata
 */

import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';

// EntityRef
export const EntityRef = z.object({
  id: z.string().min(1),
  type: Type,
});
export type EntityRef = z.infer<typeof EntityRef>;

// Core
export const EntityCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
});
export type EntityCore = z.infer<typeof EntityCore>;

// EntityState: keep common runtime fields (status/tags/meta) on top of BaseState
export const EntityState = BaseState.extend({
  status: z.string().optional(),
  tags: z.array(z.string()).optional(),
  meta: z.record(z.string(), z.unknown()).optional(),
});
export type EntityState = z.infer<typeof EntityState>;

// Signature / facets
export const EntitySignature = z.object({}).catchall(z.any());
export type EntitySignature = z.infer<typeof EntitySignature>;

// Facets structure for dialectical data
// Entity stores the complete dialectical state as it exists
export const EntityFacets = z.object({
  // Core dialectical state (from Dialectic IR)
  dialecticState: z.any().optional(), // DialecticState - avoiding circular dependency

  // Current phase in dialectical progression
  phase: z.string().optional(), // CpuGpuPhase

  // Moments active in this entity
  moments: z.array(z.any()).optional(),

  // Invariants that must hold
  invariants: z.array(z.any()).optional(),

  // Evaluation context
  context: z.any().optional(),

  // Additional entity-specific facets via catchall
}).catchall(z.any());
export type EntityFacets = z.infer<typeof EntityFacets>;

// ==========================================
// ENTITY SHAPE (Empirical - Principled Effect)
// ==========================================
// Entity is the reciprocation of Form (Rational) and Data (Empirical)
// It holds formId (reference to Principle) and values (actual data)
export const EntityShapeSchema = z.object({
  id: z.string(),
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),

  // Reference to Form Principle (Form:Entity relationship)
  formId: z.string(), // Required - every Entity has a Form

  // Actual field values (Data from Model system)
  values: z.record(z.string(), z.any()).optional().default({}),

  // Signature: operational interface
  signature: EntitySignature.optional(),

  // Facets: dialectical structure
  facets: EntityFacets.optional(),

  // Runtime state (Empirical concerns)
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
export type EntityShape = z.infer<typeof EntityShapeSchema>;

// ==========================================
// ENTITY DOCUMENT (Envelope)
// ==========================================
// Shape doc
const EntityDoc = z.object({
  core: EntityCore,
  state: EntityState.default({}),
  signature: EntitySignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
  entity: EntityShapeSchema.optional(), // EMBED EntityShape
});

export const EntitySchema = BaseSchema.extend({
  shape: EntityDoc,
});
export type Entity = z.infer<typeof EntitySchema>;

// Helpers
function genId() {
  return `entity:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreateEntityInput = {
  id?: string;
  type: string;
  name?: string;
  description?: string;
  // If provided, we embed an EntityShape (Empirical) into the Entity envelope.
  // This is required for Neo4j persistence via EntityShapeRepository.
  formId?: string;
  values?: Record<string, unknown>;
  signature?: z.input<typeof EntitySignature>;
  facets?: Record<string, unknown>;
  state?: z.input<typeof EntityState>;
  version?: string;
  ext?: Record<string, unknown>;
};

export function createEntity(input: CreateEntityInput): Entity {
  const id = input.id ?? genId();

  const maybeEntityShape =
    input.formId !== undefined
      ? EntityShapeSchema.parse({
          id,
          type: input.type,
          name: input.name,
          description: input.description,
          formId: input.formId,
          values: input.values ?? {},
        })
      : undefined;

  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name, description: input.description },
      state: input.state ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
      ...(maybeEntityShape ? { entity: maybeEntityShape } : {}),
    },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  };
  return EntitySchema.parse(draft);
}

// parse/format helpers: use last ':' as separator so types with ':' are supported
export function createEntityRef(input: Entity | { id: string; type: string } | EntityRef): EntityRef {
  if ((input as any).shape) {
    const doc = input as Entity;
    return EntityRef.parse({ id: doc.shape.core.id, type: doc.shape.core.type });
  }
  return EntityRef.parse(input as any);
}

export function formatEntityKey(ref: EntityRef): string {
  // simple human-readable form: type:id (do not percent-encode — tests expect raw colons)
  return `${ref.type}:${ref.id}`;
}

export function parseEntityKey(key: string): EntityRef {
  const idx = key.indexOf(':');
  if (idx <= 0) throw new Error(`invalid entity key: ${key}`);
  const type = key.slice(0, idx);
  const id = key.slice(idx + 1);
  return EntityRef.parse({ id, type });
}

export type EntityCoreOut = z.output<typeof EntityCore>;
export type EntityStateOut = z.output<typeof EntityState>;

type UpdateEntityPatch = Partial<{
  core: Partial<z.output<typeof EntityCore>>;
  state: Partial<z.output<typeof EntityState>>;
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateEntity(doc: Entity, patch: UpdateEntityPatch): Entity {
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
      core: { ...(doc.shape.core as EntityCore), ...(patch.core ?? {}) },
      state: {
        ...(doc.shape.state as z.output<typeof EntityState>),
        ...(patch.state ?? {}),
      },
      signature: nextSignature,
      facets: patch.facets ?? doc.shape.facets,
    },
    version: patch.version ?? doc.version,
    ext: patch.ext ?? doc.ext,
    revision: (doc.revision ?? 0) + 1,
  };
  return EntitySchema.parse(next);
}

/**
 * Helper: Get dialectic state from Entity facets
 * Used by EntityEngine to access embedded dialectical structure
 */
export function getEntityDialecticState(entity: Entity): any | undefined {
  return (entity.shape.facets as any)?.dialecticState;
}

/**
 * Helper: Get moments from Entity facets
 */
export function getEntityMoments(entity: Entity): any[] {
  return ((entity.shape.facets as any)?.moments ?? []) as any[];
}

