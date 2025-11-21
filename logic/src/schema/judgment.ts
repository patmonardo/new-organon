import { z } from "zod";
import { BaseCore, BaseSchema, BaseState, Type, Label } from "./base";
import { EntityRef } from "./entity";

export const JudgmentCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type JudgmentCore = z.infer<typeof JudgmentCore>;

export const Polarity = z.enum(["affirm", "deny"]);
export const Modality = z.enum(["possible", "actual", "necessary"]);
export type Polarity = z.infer<typeof Polarity>;
export type Modality = z.infer<typeof Modality>;

const JudgmentDoc = z.object({
  core: JudgmentCore,
  state: BaseState.default({}),
  subject: EntityRef,           // S
  predicate: z.string(),        // “kind” or relation label
  object: EntityRef.optional(), // O (optional; S–P or S–P–O)
  polarity: Polarity.default("affirm"),
  modality: Modality.default("actual"),
  agentId: z.string().optional(), // self-assertion anchor (optional)
  warrant: z.any().optional(),    // evidence/grounds (opaque)
  facets: z.record(z.string(), z.any()).default({}),
});

export const JudgmentSchema = BaseSchema.extend({ shape: JudgmentDoc });
export type Judgment = z.infer<typeof JudgmentSchema>;

function genId() {
  return `judgment:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

type CreateJudgmentInput = {
  id?: string;
  type: string;
  name?: string;
  subject: z.input<typeof EntityRef>;
  predicate: string;
  object?: z.input<typeof EntityRef>;
  polarity?: Polarity;
  modality?: Modality;
  agentId?: string;
  warrant?: unknown;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createJudgment(input: CreateJudgmentInput): Judgment {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      subject: EntityRef.parse(input.subject),
      predicate: input.predicate,
      object: input.object ? EntityRef.parse(input.object) : undefined,
      polarity: input.polarity ?? "affirm",
      modality: input.modality ?? "actual",
      agentId: input.agentId,
      warrant: input.warrant,
      facets: input.facets ?? {},
    },
  };
  return JudgmentSchema.parse(draft);
}
