import { z } from "zod";
import { randomUUID } from "node:crypto";
import { BaseSchema, BaseState, BaseCore, Type, Label, touch } from "./base";
import { EntityRef } from "./entity";

// Direction
export const RelationDirection = z.enum(["directed", "bidirectional"]);
export type RelationDirection = z.infer<typeof RelationDirection>;

// Core/state
export const RelationCore = BaseCore.extend({
  type: Type, // schema/category, e.g., "system.Relation"
  kind: Label, // relation kind, e.g., "related_to", "contains"
});
export type RelationCore = z.infer<typeof RelationCore>;

export const RelationState = BaseState.extend({
  strength: z.number().min(0).max(1).default(1), // optional weight (0..1)
});
export type RelationState = z.infer<typeof RelationState>;

// Shape: endpoints + direction
export const RelationShape = z.object({
  core: RelationCore,
  state: RelationState,
  source: EntityRef,
  target: EntityRef,
  direction: RelationDirection.default("directed"),
});
export type RelationShape = z.infer<typeof RelationShape>;

// Schema
export const RelationSchema = BaseSchema.extend({
  shape: RelationShape,
});
export type Relation = z.infer<typeof RelationSchema>;

// Create/update
export function createRelation(input: {
  id?: string;
  type: z.input<typeof Type>;
  kind: z.input<typeof Label>;
  source: z.input<typeof EntityRef>;
  target: z.input<typeof EntityRef>;
  direction?: z.input<typeof RelationDirection>;

  name?: string;
  description?: string;

  state?: z.input<typeof RelationState>;
  version?: string;
  ext?: Record<string, unknown>;
}): Relation {
  const core = RelationCore.parse({
    id: input.id ?? randomUUID(),
    type: input.type,
    kind: input.kind,
    name: input.name,
    description: input.description,
  });
  const state = RelationState.parse(input.state ?? {});
  const source = EntityRef.parse(input.source);
  const target = EntityRef.parse(input.target);
  const direction = RelationDirection.parse(input.direction ?? "directed");

  return RelationSchema.parse({
    shape: { core, state, source, target, direction },
    revision: 0,
    version: input.version,
    ext: input.ext ?? {},
  });
}

export function updateRelation(
  current: Relation,
  patch: Partial<{
    core: Partial<z.input<typeof RelationCore>>;
    state: Partial<z.input<typeof RelationState>>;
    source: z.input<typeof EntityRef>;
    target: z.input<typeof EntityRef>;
    direction: z.input<typeof RelationDirection>;
    version: string;
    ext: Record<string, unknown>;
  }>
): Relation {
  const core = RelationCore.parse(
    touch({ ...current.shape.core, ...(patch.core ?? {}) })
  );
  const state = RelationState.parse({
    ...current.shape.state,
    ...(patch.state ?? {}),
  });

  return RelationSchema.parse({
    ...current,
    shape: {
      core,
      state,
      source: patch.source
        ? EntityRef.parse(patch.source)
        : current.shape.source,
      target: patch.target
        ? EntityRef.parse(patch.target)
        : current.shape.target,
      direction: patch.direction
        ? RelationDirection.parse(patch.direction)
        : current.shape.direction,
    },
    revision: current.revision + 1,
    version: patch.version ?? current.version,
    ext: { ...current.ext, ...(patch.ext ?? {}) },
  });
}

// Ergonomics
export function createBidirectionalRelation(
  input: Omit<Parameters<typeof createRelation>[0], "direction">
): Relation {
  return createRelation({ ...input, direction: "bidirectional" });
}

export function invertRelation(rel: Relation): Relation {
  if (rel.shape.direction === "bidirectional") return rel;
  return createRelation({
    type: rel.shape.core.type,
    kind: rel.shape.core.kind,
    source: rel.shape.target,
    target: rel.shape.source,
    direction: rel.shape.direction, // remains "directed"
    name: rel.shape.core.name,
    description: rel.shape.core.description,
    state: rel.shape.state,
    version: rel.version,
    ext: rel.ext,
  });
}

// Small helpers
export type RelationKey = string;
