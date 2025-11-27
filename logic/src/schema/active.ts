/**
 * Active Schema - "Kriya Integration / Runtime"
 *
 * Active represents the runtime instantiation of the logical forms.
 * It is where Logic (Idea) meets Energy (Kriya) to become actual.
 *
 * Dialectical Role:
 * - ActiveShape: Runtime instance of a Shape (Logic in action)
 * - ActiveContext: Runtime instance of a Context (Scope in action)
 * - ActiveMorph: Runtime instance of a Morph (Transformation in action)
 *
 * Kriya Integration:
 * - Adds "Activation" properties (active, revoked, confidence, weight)
 * - Connects static schemas to dynamic execution engines
 */

import { z } from 'zod';

// Shared helpers
const clamp01 = (n: number) =>
  Number.isFinite(n) ? Math.min(1, Math.max(0, n)) : n;

const ProvenanceSchema = z.any().optional();

const ActivationSchema = z.object({
  active: z.boolean().optional(),
  revoked: z.boolean().optional(),
  confidence: z
    .number()
    .optional()
    .transform((v) => (typeof v === 'number' ? clamp01(v) : v)),
  weight: z.number().finite().optional(),
  provenance: ProvenanceSchema,

  // Dialectical runtime state
  dialecticState: z.any().optional(), // Runtime dialectic state
  currentPhase: z.string().optional(), // Current phase of execution
});

// Base
export const ActiveBaseSchema = z
  .object({
    id: z.string().min(1),
    kind: z.string().optional(),
  })
  .merge(ActivationSchema);

// ActiveShape (id is optional; engine may derive from name deterministically)
export const ActiveShapeSchema = z
  .object({
    id: z.string().min(1).optional(),
    kind: z.string().optional(),
    name: z.string().optional(),
    schema: z.any().optional(),
    particularityOf: z.string().optional(),

    // Runtime facets
    facets: z.record(z.string(), z.unknown()).optional(),
  })
  .merge(ActivationSchema);
export type ActiveShape = z.infer<typeof ActiveShapeSchema>;

export const ActiveContextSchema = z.object({
  id: z.string().optional(),
  contextId: z.string().optional(),
  name: z.string().optional(),
  kind: z.string().optional(),
  revoked: z.boolean().optional(),
  state: z.record(z.string(), z.unknown()).optional(),
  signature: z.record(z.string(), z.unknown()).optional(),
  facets: z.record(z.string(), z.unknown()).optional(),
  version: z.string().optional(),
  ext: z.record(z.string(), z.unknown()).optional(),
  shape: z.any().optional(),

  // Runtime extensions
  active: z.boolean().optional(),
  confidence: z.number().optional(),
});

export type ActiveContext = z.infer<typeof ActiveContextSchema>;

// ActiveMorph
export const ActiveMorphSchema = ActiveBaseSchema.merge(
  z.object({
    particularityOf: z.string().optional(),
    transform: z.string().min(1),
    params: z.record(z.string(), z.any()).optional(),

    // Runtime composition
    pipeline: z.array(z.string()).optional(),
  }),
);
export type ActiveMorph = z.infer<typeof ActiveMorphSchema>;

// ActiveEntity
export const ActiveEntitySchema = z
  .object({
    id: z.string().min(1),
    entityType: z.string().min(1),
    particularityOf: z.string().optional(),
    labels: z.array(z.string()).optional(),

    // Runtime properties
    properties: z.record(z.string(), z.any()).optional(),
  })
  .merge(ActivationSchema);
export type ActiveEntity = z.infer<typeof ActiveEntitySchema>;

// ActiveProperty
export const ActivePropertySchema = z
  .object({
    id: z.string().min(1),
    subjectId: z.string().min(1),
    key: z.string().min(1),
    value: z.unknown(),
    dtype: z.string().optional(),
    particularityOf: z.string().optional(),
  })
  .merge(ActivationSchema);
export type ActiveProperty = z.infer<typeof ActivePropertySchema>;

// ActiveAspect is skeletal: reuse ActiveShape (strip all relation extras)
export const ActiveAspectSchema = ActiveShapeSchema;
export type ActiveAspect = ActiveShape;

// Batch validators
export const parseActiveShapes = (data: unknown) =>
  z.array(ActiveShapeSchema).parse(data);
export const parseActiveContexts = (data: unknown) =>
  z.array(ActiveContextSchema).parse(data);
export const parseActiveMorphs = (data: unknown) =>
  z.array(ActiveMorphSchema).parse(data);
export const parseActiveEntities = (data: unknown) =>
  z.array(ActiveEntitySchema).parse(data);
export const parseActiveProperties = (data: unknown) =>
  z.array(ActivePropertySchema).parse(data);
export const parseActiveAspects = (data: unknown) =>
  z.array(ActiveAspectSchema).parse(data);

// Single validators (safeParse variants)
export const safeParseActiveShape = (data: unknown) =>
  ActiveShapeSchema.safeParse(data);
export const safeParseActiveContext = (data: unknown) =>
  ActiveContextSchema.safeParse(data);
export const safeParseActiveMorph = (data: unknown) =>
  ActiveMorphSchema.safeParse(data);
export const safeParseActiveEntity = (data: unknown) =>
  ActiveEntitySchema.safeParse(data);
export const safeParseActiveProperty = (data: unknown) =>
  ActivePropertySchema.safeParse(data);
export const safeParseActiveAspect = (data: unknown) =>
  ActiveAspectSchema.safeParse(data);

// Normalizers
export function normalizeActivation<
  T extends { confidence?: number | undefined },
>(obj: T): T {
  if (typeof obj.confidence === 'number') {
    (obj as any).confidence = clamp01(obj.confidence);
  }
  return obj;
}

// Default runtime export for compatibility with previous modules that
// imported the ActiveAny as a default value.
export type ActiveAny =
  | ActiveShape
  | ActiveContext
  | ActiveMorph
  | ActiveEntity
  | ActiveProperty
  | ActiveAspect;

// Lightweight runtime helpers for working with Active objects.
export const ActiveFactory = {
  // parsers (zod-backed)
  parseShape: (d: unknown) => ActiveShapeSchema.parse(d),
  parseContext: (d: unknown) => ActiveContextSchema.parse(d),
  parseMorph: (d: unknown) => ActiveMorphSchema.parse(d),
  parseEntity: (d: unknown) => ActiveEntitySchema.parse(d),
  parseProperty: (d: unknown) => ActivePropertySchema.parse(d),
  parseAspect: (d: unknown) => ActiveAspectSchema.parse(d),

  // convenience factories with minimal defaults
  createProperty: (p: Partial<z.infer<typeof ActivePropertySchema>> = {}) =>
    ActivePropertySchema.parse({
      id: p.id ?? `prop:${Date.now()}`,
      subjectId: p.subjectId ?? String(p.subjectId ?? 'unknown'),
      key: p.key ?? 'unknown',
      value: p.value,
      ...p,
    }),

  // skeletal aspect factory (no relation endpoints)
  createAspect: (a: Partial<z.infer<typeof ActiveAspectSchema>> = {}) =>
    ActiveAspectSchema.parse({
      id: a.id ?? `aspect:${Date.now()}`,
      name: a.name,
      kind: a.kind ?? 'system.Aspect',
      ...a,
    }),

  normalizeActivation,
};

export default ActiveFactory;

