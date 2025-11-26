import { z } from "zod";

// Primitives
export const Id = z.string().min(1, "id required");
export type Id = z.infer<typeof Id>;

export const Label = z.string().min(1, "label required");
export type Label = z.infer<typeof Label>;

export const Type = z.string().min(1, "type required");
export type Type = z.infer<typeof Type>;

export const IsoDateTime = z.string().datetime();
export type IsoDateTime = z.infer<typeof IsoDateTime>;

// Timestamps
export const Timestamps = z.object({
  createdAt: IsoDateTime.default(() => new Date().toISOString()),
  updatedAt: IsoDateTime.default(() => new Date().toISOString()),
});
export type Timestamps = z.infer<typeof Timestamps>;

// Helpers
export function stamp<T extends { createdAt?: string; updatedAt?: string }>(
  x: T
): T & Timestamps {
  const now = new Date().toISOString();
  return {
    createdAt: x.createdAt ?? now,
    updatedAt: x.updatedAt ?? now,
    ...x,
  } as T & Timestamps;
}
export function touch<T extends { updatedAt?: string }>(x: T): T {
  return { ...x, updatedAt: new Date().toISOString() };
}

// BaseCore (id + optional name/description + timestamps)
export const BaseCore = z.object({
  id: Id,
  type: Type,
  name: Label.optional(),
  description: z.string().optional(),
  createdAt: IsoDateTime.default(() => new Date().toISOString()),
  updatedAt: IsoDateTime.default(() => new Date().toISOString()),
});
export type BaseCore = z.infer<typeof BaseCore>;

// BaseState (common state)
// Make state permissive so Forms can stash arbitrary runtime fields
export const BaseState = z.object({}).catchall(z.unknown());
export type BaseState = z.infer<typeof BaseState>;

// BaseShape (Core + State)
export const BaseShape = z.object({
  core: BaseCore,
  state: BaseState,
});
export type BaseShape = z.infer<typeof BaseShape>;

// BaseSchema (Shape + meta header)
export const BaseSchema = z.object({
  shape: BaseShape,
  revision: z.number().int().nonnegative().default(0),
  version: z.string().optional(),
  ext: z.record(z.string(), z.unknown()).default({}),
});
export type Base = z.infer<typeof BaseSchema>;

/**
 * Return the canonical core object from a variety of schema shapes.
 *
 * Accepts:
 * - full Base (with .shape.core)
 * - BaseShape (with .core)
 * - a direct core-like object
 *
 * Returns null when no core-like structure is found.
 */
export function getCore<T extends BaseCore = BaseCore>(obj: unknown): T | null {
  const o = obj as any;
  if (!o || typeof o !== 'object') return null;
  if (o.shape && o.shape.core && typeof o.shape.core === 'object') {
    return o.shape.core as T;
  }
  if (o.core && typeof o.core === 'object') {
    return o.core as T;
  }
  return null;
}

/**
 * Return the canonical state object from a variety of schema shapes.
 *
 * Accepts:
 * - full Base (with .shape.state)
 * - BaseShape (with .state)
 * - a direct state-like object
 *
 * Returns null when no state-like structure is found.
 */
export function getState<T extends BaseState = BaseState>(obj: unknown): T | null {
  const o = obj as any;
  if (!o || typeof o !== 'object') return null;
  if (o.shape && o.shape.state && typeof o.shape.state === 'object') {
    return o.shape.state as T;
  }
  if (o.state && typeof o.state === 'object') {
    return o.state as T;
  }
  return null;
}

/**
 * Return the canonical shape object (core + state) from a variety of schema shapes.
 *
 * Accepts:
 * - full Base (with .shape)
 * - BaseShape (core+state)
 *
 * Returns null when no shape-like structure is found.
 */
export function getShape<T extends BaseShape = BaseShape>(obj: unknown): T | null {
  const o = obj as any;
  if (!o || typeof o !== 'object') return null;
  if (o.shape && typeof o.shape === 'object') {
    return o.shape as T;
  }
  if (o.core || o.state) {
    return { core: (o.core ?? {}), state: (o.state ?? {}) } as T;
  }
  return null;
}
