import { z } from "zod";
import { BaseCore, BaseSchema, BaseState, Type, Label } from "./base";
import { EntityRef } from "./entity";

// Principle-level Content (Matter+Form), classified as subtle (World) or gross (Thing)
export const ContentCore = BaseCore.extend({
  type: Type,            // e.g., "system.Content"
  name: Label.optional()
});
export type ContentCore = z.infer<typeof ContentCore>;

export const ContentKind = z.enum(["subtle", "gross"]);
export type ContentKind = z.infer<typeof ContentKind>;

export const ContentSignature = z.object({}).catchall(z.any());
export type ContentSignature = z.infer<typeof ContentSignature>;

const ContentDoc = z.object({
  core: ContentCore,
  state: BaseState.default({}),
  kind: ContentKind,         // subtle → World; gross → Thing
  of: EntityRef,             // carrier entity (Thing basis)
  signature: ContentSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
  payload: z.any().optional(), // optional raw/material payload
});

export const ContentSchema = BaseSchema.extend({
  shape: ContentDoc,
});
export type Content = z.infer<typeof ContentSchema>;

// Helpers
function genId() {
  return `content:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

type CreateContentInput = {
  id?: string;
  type: string;
  name?: string;
  kind: ContentKind;
  of: z.input<typeof EntityRef>;
  signature?: z.input<typeof ContentSignature>;
  facets?: Record<string, unknown>;
  payload?: unknown;
  state?: z.input<typeof BaseState>;
};

export function createContent(input: CreateContentInput): Content {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      kind: input.kind,
      of: EntityRef.parse(input.of),
      signature: input.signature,
      facets: input.facets ?? {},
      payload: input.payload,
    },
  };
  return ContentSchema.parse(draft);
}

type ContentCoreOut = z.output<typeof ContentCore>;
type BaseStateOut = z.output<typeof BaseState>;

type UpdateContentPatch = Partial<{
  core: Partial<ContentCoreOut>;
  state: Partial<BaseStateOut>;
  kind: ContentKind;
  of: z.input<typeof EntityRef>;
  signature: z.input<typeof ContentSignature> | undefined;
  facets: Record<string, unknown>;
  payload: unknown;
}>;

export function updateContent(doc: Content, patch: UpdateContentPatch): Content {
  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as ContentCoreOut), ...(patch.core ?? {}) },
      state: { ...(doc.shape.state as BaseStateOut), ...(patch.state ?? {}) },
      kind: patch.kind ?? doc.shape.kind,
      of: patch.of ? EntityRef.parse(patch.of) : doc.shape.of,
      signature: patch.signature === undefined ? doc.shape.signature : patch.signature,
      facets: patch.facets ?? doc.shape.facets,
      payload: patch.payload === undefined ? doc.shape.payload : patch.payload,
    },
    revision: (doc.revision ?? 0) + 1,
  };
  return ContentSchema.parse(next);
}
