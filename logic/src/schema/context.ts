import { z } from "zod";
import { BaseSchema, BaseState, BaseCore, Type, Id } from "./base";
import { EntityRef } from "./entity";

// Core/state (uniform)
export const ContextCore = BaseCore.extend({
  type: Type,
});
export type ContextCore = z.infer<typeof ContextCore>;

export const ContextState = BaseState;
export type ContextState = z.infer<typeof ContextState>;

// Shape: core/state + memberships + signature/facets (uniform base)
export const ContextShape = z.object({
  core: ContextCore,
  state: ContextState.default({}),
  entities: z.array(EntityRef).default([]),
  relations: z.array(Id).default([]),
  signature: z.object({}).catchall(z.any()).optional(),
  facets: z.record(z.string(), z.any()).default({}),
});
export type ContextShape = z.infer<typeof ContextShape>;

// Schema
export const ContextSchema = BaseSchema.extend({
  shape: ContextShape,
});
export type Context = z.infer<typeof ContextSchema>;

// Create/update (uniform signature handling: null clears, undefined preserves)
export function createContext(input: {
  id?: string;
  type: z.input<typeof Type>;
  name?: string;
  description?: string;
  state?: z.input<typeof ContextState>;
  entities?: z.input<typeof EntityRef>[];
  relations?: z.input<typeof Id>[];
  signature?: Record<string, unknown>;
  facets?: Record<string, unknown>;
  ext?: Record<string, unknown>;
  version?: string;
}): Context {
  return ContextSchema.parse({
    shape: {
      core: ContextCore.parse({
        id:
          input.id ??
          `context:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
            .toString(36)
            .padStart(4, "0")}`,
        type: input.type,
        name: input.name,
        description: input.description,
      }),
      state: ContextState.parse(input.state ?? {}),
      entities: (input.entities ?? []).map((e) => EntityRef.parse(e)),
      relations: (input.relations ?? []).map((r) => Id.parse(r)),
      signature: input.signature,
      facets: input.facets ?? {},
    },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}

type UpdateContextPatch = Partial<{
  core: Partial<z.input<typeof ContextCore>>;
  state: Partial<z.input<typeof ContextState>>;
  entities: z.input<typeof EntityRef>[];
  relations: z.input<typeof Id>[];
  signature: Record<string, unknown> | null | undefined; // null => clear, undefined => preserve
  facets: Record<string, unknown>;
  version: string;
  ext: Record<string, unknown>;
}>;

export function updateContext(current: Context, patch: UpdateContextPatch): Context {
  const nextSignature =
    patch.signature === null
      ? undefined
      : patch.signature !== undefined
      ? patch.signature
      : current.shape.signature;

  const entities =
    patch.entities !== undefined
      ? z.array(EntityRef).parse(patch.entities)
      : current.shape.entities;
  const relations =
    patch.relations !== undefined
      ? z.array(Id).parse(patch.relations)
      : current.shape.relations;

  return ContextSchema.parse({
    ...current,
    shape: {
      ...current.shape,
      core: ContextCore.parse({ ...current.shape.core, ...(patch.core ?? {}) }),
      state: ContextState.parse({ ...current.shape.state, ...(patch.state ?? {}) }),
      entities,
      relations,
      signature: nextSignature,
      facets: patch.facets ?? current.shape.facets,
    },
    revision: (current.revision ?? 0) + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function addEntitiesToContext(
  ctx: Context,
  entities: z.input<typeof EntityRef>[]
): Context {
  const exists = new Set(ctx.shape.entities.map((e) => `${e.type}:${e.id}`));
  const toAdd = (entities ?? []).map((e) => EntityRef.parse(e)).filter(
    (e) => !exists.has(`${e.type}:${e.id}`)
  );
  if (toAdd.length === 0) return ctx;
  return updateContext(ctx, { entities: [...ctx.shape.entities, ...toAdd] });
}

export function addRelationsToContext(
  ctx: Context,
  relationIds: z.input<typeof Id>[]
): Context {
  const exists = new Set(ctx.shape.relations);
  const toAdd = (relationIds ?? []).map((r) => Id.parse(r)).filter((id) => !exists.has(id));
  if (toAdd.length === 0) return ctx;
  return updateContext(ctx, { relations: [...ctx.shape.relations, ...toAdd] });
}
