import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';

// Principle-level Shape (no UI concerns)
export const ShapeCore = BaseCore.extend({
  type: Type, // e.g., "system.Shape"
  name: Label.optional(), // optional display name
});
export type ShapeCore = z.infer<typeof ShapeCore>;

// Open signature for extensibility (principle facets)
export const ShapeSignature = z.object({}).catchall(z.any());
export type ShapeSignature = z.infer<typeof ShapeSignature>;

const ShapeDoc = z.object({
  core: ShapeCore,
  state: BaseState.default({}),
  signature: ShapeSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
});

export const ShapeSchema = BaseSchema.extend({
  shape: ShapeDoc,
});
export type Shape = z.infer<typeof ShapeSchema>;

// Helpers
function genId() {
  return `shape:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreateShapeInput = {
  id?: string;
  type: string;
  name?: string;
  signature?: z.input<typeof ShapeSignature>;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createShape(input: CreateShapeInput): Shape {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
    },
  };
  return ShapeSchema.parse(draft);
}

export type ShapeCoreOut = z.output<typeof ShapeCore>;

type UpdateShapePatch = Partial<{
  core: Partial<z.output<typeof ShapeCore>>;
  state: Partial<z.output<typeof BaseState>>;
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateShape(doc: Shape, patch: UpdateShapePatch): Shape {
  const nextSignature =
    patch.signature === null
      ? undefined // explicit clear
      : patch.signature !== undefined
      ? patch.signature // replace
      : doc.shape.signature; // preserve

  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as ShapeCore), ...(patch.core ?? {}) },
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
  return ShapeSchema.parse(next);
}
