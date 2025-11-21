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

export const AspectDoc = z.object({
  core: AspectCore,
  state: BaseState.default({}),
  signature: AspectSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
});

export const AspectSchema = BaseSchema.extend({
  shape: z.object({
    core: AspectCore,
    state: BaseState.default({}),            // <= ensure this
    signature: z.object({}).catchall(z.any()).optional(),
    facets: z.record(z.string(), z.any()).default({}),
  }),
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
