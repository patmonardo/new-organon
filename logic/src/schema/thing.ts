import { z } from "zod";
import { BaseCore, BaseSchema, BaseState, Id, Type, Label } from "./base";

// Core identity for Thing
export const ThingCore = BaseCore.extend({
  type: Type, // e.g., "system.Thing"
  name: Label.optional(), // optional human label
  category: z.string().optional(), // optional conceptual category
});
export type ThingCore = z.infer<typeof ThingCore>;

// Essence/qualities as open object, principle-level only
const Essence = z.object({}).catchall(z.any());

// Thing document shape: identity + state + essence + qualities
const ThingShapeDoc = z.object({
  core: ThingCore,
  state: BaseState.default({}), // status: 'active' | 'archived' | 'deleted'
  essence: Essence.optional(),
  qualities: z.record(z.string(), z.any()).default({}), // “what it is like” map
});

// Canonical Thing schema
export const ThingSchema = BaseSchema.extend({
  shape: ThingShapeDoc,
});
export type Thing = z.infer<typeof ThingSchema>;

// Thin reference to a Thing by identity
export const ThingRef = z.object({
  id: Id,
  type: z.literal("system.Thing"),
});
export type ThingRef = z.infer<typeof ThingRef>;

export function isThingRef(v: unknown): v is ThingRef {
  return ThingRef.safeParse(v).success;
}

export function createThingRef(t: { shape: { core: { id: string } } } | { id: string }): ThingRef {
  const id = "shape" in t ? t.shape.core.id : t.id;
  return ThingRef.parse({ id, type: "system.Thing" });
}

// Helpers
function genId() {
  return `thing:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

type CreateThingInput = {
  id?: string;
  type: string;
  name?: string;
  category?: string;
  essence?: Record<string, unknown>;
  qualities?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createThing(input: CreateThingInput): Thing {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name, category: input.category },
      state: input.state ?? {},
      essence: input.essence,
      qualities: input.qualities ?? {},
    },
  };
  return ThingSchema.parse(draft);
}

type ThingCoreOut = z.output<typeof ThingCore>;
type BaseStateOut = z.output<typeof BaseState>;

type UpdateThingPatch = Partial<{
  core: Partial<ThingCoreOut>;
  state: Partial<BaseStateOut>;
  essence: Record<string, unknown> | undefined;
  qualities: Record<string, unknown>;
}>;

export function updateThing(doc: Thing, patch: UpdateThingPatch): Thing {
  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as ThingCoreOut), ...(patch.core ?? {}) },
      state: { ...(doc.shape.state as BaseStateOut), ...(patch.state ?? {}) },
      essence:
        patch.essence === undefined ? doc.shape.essence : patch.essence,
      qualities: patch.qualities ?? doc.shape.qualities,
    },
    revision: (doc.revision ?? 0) + 1,
  };
  return ThingSchema.parse(next);
}
