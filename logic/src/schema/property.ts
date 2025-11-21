import { z } from 'zod';
import { BaseCore, BaseSchema, BaseState, Type, Label } from './base';

// Core mirrors ShapeCore (skeletal)
export const PropertyCore = BaseCore.extend({
  type: Type, // e.g., "system.Property"
  name: Label.optional(),
});
export type PropertyCore = z.infer<typeof PropertyCore>;

// Open signature for extensibility
export const PropertySignature = z.object({}).catchall(z.any());
export type PropertySignature = z.infer<typeof PropertySignature>;

// Doc shape (skeletal)
export const PropertyDoc = z.object({
  core: PropertyCore,
  state: BaseState.default({}),
  signature: PropertySignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
});

export const PropertySchema = BaseSchema.extend({
  shape: z.object({
    core: PropertyCore,
    state: BaseState.default({}),            // <= ensure this
    signature: z.object({}).catchall(z.any()).optional(),
    facets: z.record(z.string(), z.any()).default({}),
  }),
});
export type Property = z.infer<typeof PropertySchema>;

// Helpers
function genId() {
  return `property:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, '0')}`;
}

type CreatePropertyInput = {
  id?: string;
  type: string;
  name?: string;
  signature?: z.input<typeof PropertySignature>;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createProperty(input: CreatePropertyInput): Property {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
    },
  };
  return PropertySchema.parse(draft);
}

export type PropertyCoreOut = z.output<typeof PropertyCore>;
export type BaseStateOut = z.output<typeof BaseState>;

type UpdatePropertyPatch = Partial<{
  core: Partial<z.output<typeof PropertyCore>>;
  state: Partial<z.output<typeof BaseState>>;
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateProperty(
  doc: Property,
  patch: UpdatePropertyPatch,
): Property {
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
      core: { ...(doc.shape.core as PropertyCore), ...(patch.core ?? {}) },
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
  return PropertySchema.parse(next);
}
