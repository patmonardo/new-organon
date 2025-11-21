import { z } from "zod";
import { BaseCore, BaseSchema, BaseState, Type, Label } from "./base";
import { ThingRef } from "./thing";

// Core identity for World
export const WorldCore = BaseCore.extend({
  type: Type, // e.g., "system.World"
  name: Label.optional(),
});
export type WorldCore = z.infer<typeof WorldCore>;

// Minimal relation edge inside a World (principle-level)
export const WorldEdge = z.object({
  kind: Label.default("related_to"),
  direction: z.enum(["directed", "bidirectional"]).default("directed"),
  source: ThingRef,
  target: ThingRef,
});
export type WorldEdge = z.infer<typeof WorldEdge>;

// Optional horizon/context envelope (kept open)
const Horizon = z.object({}).catchall(z.any());

// World document shape: identity + state + a set of things and their edges
const WorldShapeDoc = z.object({
  core: WorldCore,
  state: BaseState.default({}),
  things: z.array(ThingRef).default([]),
  relations: z.array(WorldEdge).default([]),
  horizon: Horizon.optional(),
});

// Canonical World schema
export const WorldSchema = BaseSchema.extend({
  shape: WorldShapeDoc,
});
export type World = z.infer<typeof WorldSchema>;

// Helpers
function genId() {
  return `world:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

type CreateWorldInput = {
  id?: string;
  type: string;
  name?: string;
  state?: z.input<typeof BaseState>;
  things?: z.input<typeof ThingRef>[];
  relations?: z.input<typeof WorldEdge>[];
  horizon?: Record<string, unknown>;
};

export function createWorld(input: CreateWorldInput): World {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      things: input.things ?? [],
      relations: input.relations ?? [],
      horizon: input.horizon,
    },
  };
  return WorldSchema.parse(draft);
}

type WorldCoreOut = z.output<typeof WorldCore>;
type BaseStateOut = z.output<typeof BaseState>;

type UpdateWorldPatch = Partial<{
  core: Partial<WorldCoreOut>;
  state: Partial<BaseStateOut>;
  things: z.input<typeof ThingRef>[];
  relations: z.input<typeof WorldEdge>[];
  horizon: Record<string, unknown> | undefined;
}>;

export function updateWorld(doc: World, patch: UpdateWorldPatch): World {
  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as WorldCoreOut), ...(patch.core ?? {}) },
      state: { ...(doc.shape.state as BaseStateOut), ...(patch.state ?? {}) },
      things: patch.things ?? doc.shape.things,
      relations: patch.relations ?? doc.shape.relations,
      horizon:
        patch.horizon === undefined ? doc.shape.horizon : patch.horizon,
    },
    revision: (doc.revision ?? 0) + 1,
  };
  return WorldSchema.parse(next);
}
