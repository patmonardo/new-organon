import { z } from "zod";
import { BaseCore, BaseSchema, BaseState, Type, Label } from "./base";
import { EntityRef } from "./entity";

export const SyllogismCore = BaseCore.extend({
  type: Type,
  name: Label.optional(),
});
export type SyllogismCore = z.infer<typeof SyllogismCore>;

export const Figure = z.enum(["1", "2", "3", "4"]);
export type Figure = z.infer<typeof Figure>;

// Mood as AAA/EAE/... (three letters: major, minor, conclusion)
export const Mood = z.string().regex(/^[AEIO]{3}$/);
export type Mood = z.infer<typeof Mood>;

const Terms = z.object({
  S: EntityRef, // Subject
  P: EntityRef, // Predicate
  M: EntityRef, // Middle
});

const SyllogismDoc = z.object({
  core: SyllogismCore,
  state: BaseState.default({}),
  figure: Figure,
  mood: Mood,
  terms: Terms,
  facets: z.record(z.string(), z.any()).default({}),
});

export const SyllogismSchema = BaseSchema.extend({ shape: SyllogismDoc });
export type Syllogism = z.infer<typeof SyllogismSchema>;

function genId() {
  return `syll:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

type CreateSyllogismInput = {
  id?: string;
  type: string;
  name?: string;
  figure: Figure;
  mood: Mood;
  terms: z.input<typeof Terms>;
  state?: z.input<typeof BaseState>;
  facets?: Record<string, unknown>;
};

export function createSyllogism(input: CreateSyllogismInput): Syllogism {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      figure: input.figure,
      mood: input.mood,
      terms: {
        S: EntityRef.parse(input.terms.S),
        P: EntityRef.parse(input.terms.P),
        M: EntityRef.parse(input.terms.M),
      },
      facets: input.facets ?? {},
    },
  };
  return SyllogismSchema.parse(draft);
}
