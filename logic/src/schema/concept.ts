import { z } from "zod";
import { BaseCore, BaseSchema, BaseState, Type, Label } from "./base";
import { EntityRef } from "./entity";

/**
 * Concept (UPS) — Truth of Reflection (Being → Essence → Concept)
 * - universal: intension/law (what the concept is in itself)
 * - particular: determinations/constraints (its articulated content)
 * - singular: exemplars/witnesses (its appearance in concrete instances)
 * - wheel: deterministic stage machine over U → P → S → return
 */

export const ConceptCore = BaseCore.extend({
  type: Type,             // e.g., "system.Concept"
  name: Label.optional(), // optional display name
});
export type ConceptCore = z.infer<typeof ConceptCore>;

// Universal (intension/law)
export const ConceptUniversal = z.object({
  title: z.string().optional(),
  law: z.string().optional(),                 // optional law statement
  intent: z.record(z.string(), z.any()).default({}), // intension as open record
});
export type ConceptUniversal = z.infer<typeof ConceptUniversal>;

// Particular (determinations/constraints)
export const Determination = z.object({
  key: z.string(),
  value: z.any(),
  note: z.string().optional(),
});
export type Determination = z.infer<typeof Determination>;

export const ConceptParticular = z.object({
  determinations: z.array(Determination).default([]),
  constraints: z.record(z.string(), z.any()).default({}),
});
export type ConceptParticular = z.infer<typeof ConceptParticular>;

// Singular (exemplars/witnesses)
export const ConceptSingular = z.object({
  exemplars: z.array(EntityRef).default([]), // concrete carriers
  witness: z.array(z.string()).default([]),  // evidence ids/notes
});
export type ConceptSingular = z.infer<typeof ConceptSingular>;

// UPS triad
export const ConceptTriad = z.object({
  universal: ConceptUniversal.default({ intent: {} }),
  particular: ConceptParticular.default({ determinations: [], constraints: {} }),
  singular: ConceptSingular.default({ exemplars: [], witness: [] }),
});
export type ConceptTriad = z.infer<typeof ConceptTriad>;

// Wheel (stage machine)
export const ConceptStage = z.enum(["universal", "particular", "singular", "return"]);
export type ConceptStage = z.infer<typeof ConceptStage>;

export const ConceptWheel = z.object({
  stage: ConceptStage.default("universal"),
  cycle: z.number().int().nonnegative().default(0),
});
export type ConceptWheel = z.infer<typeof ConceptWheel>;

// Reflection provenance
export const ConceptReflection = z.object({
  fromBeing: z.array(z.string()).default([]),   // source ids/notes
  fromEssence: z.array(z.string()).default([]), // source ids/notes
  notes: z.string().optional(),
});
export type ConceptReflection = z.infer<typeof ConceptReflection>;

// Canonical Concept document
const ConceptDoc = z.object({
  core: ConceptCore,
  state: BaseState.default({}),
  triad: ConceptTriad.default({
    universal: { intent: {} },
    particular: { determinations: [], constraints: {} },
    singular: { exemplars: [], witness: [] },
  }),
  wheel: ConceptWheel.default({ stage: "universal", cycle: 0 }),
  reflection: ConceptReflection.default({ fromBeing: [], fromEssence: [] }),
  facets: z.record(z.string(), z.any()).default({}),
});

export const ConceptSchema = BaseSchema.extend({
  shape: ConceptDoc,
});
export type Concept = z.infer<typeof ConceptSchema>;

// Helpers

function genId() {
  return `concept:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

export type CreateConceptInput = {
  id?: string;
  type: string;
  name?: string;
  triad?: z.input<typeof ConceptTriad>;
  wheel?: z.input<typeof ConceptWheel>;
  reflection?: z.input<typeof ConceptReflection>;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createConcept(input: CreateConceptInput): Concept {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: input.type, name: input.name },
      state: input.state ?? {},
      triad: input.triad ?? { universal: { intent: {} }, particular: { determinations: [], constraints: {} }, singular: { exemplars: [], witness: [] } },
      wheel: input.wheel ?? { stage: "universal", cycle: 0 },
      reflection: input.reflection ?? { fromBeing: [], fromEssence: [] },
      facets: input.facets ?? {},
    },
  };
  return ConceptSchema.parse(draft);
}

type ConceptCoreOut = z.output<typeof ConceptCore>;
type BaseStateOut = z.output<typeof BaseState>;
type ConceptTriadOut = z.output<typeof ConceptTriad>;
type ConceptWheelOut = z.output<typeof ConceptWheel>;
type ConceptReflectionOut = z.output<typeof ConceptReflection>;

export type UpdateConceptPatch = Partial<{
  core: Partial<ConceptCoreOut>;
  state: Partial<BaseStateOut>;
  triad: Partial<ConceptTriadOut>;
  wheel: Partial<ConceptWheelOut>;
  reflection: Partial<ConceptReflectionOut>;
  facets: Record<string, unknown>;
}>;

export function updateConcept(doc: Concept, patch: UpdateConceptPatch): Concept {
  const next = {
    ...doc,
    shape: {
      ...doc.shape,
      core: { ...(doc.shape.core as ConceptCoreOut), ...(patch.core ?? {}) },
      state: { ...(doc.shape.state as BaseStateOut), ...(patch.state ?? {}) },
      triad: deepMergeTriad(doc.shape.triad, patch.triad),
      wheel: { ...(doc.shape.wheel as ConceptWheelOut), ...(patch.wheel ?? {}) },
      reflection: { ...(doc.shape.reflection as ConceptReflectionOut), ...(patch.reflection ?? {}) },
      facets: patch.facets ?? doc.shape.facets,
    },
    revision: (doc.revision ?? 0) + 1,
  };

  // Normalize deterministically
  next.shape.triad.particular.determinations.sort((a, b) => a.key.localeCompare(b.key));
  next.shape.triad.singular.exemplars.sort((a, b) => a.id.localeCompare(b.id));
  next.shape.triad.singular.witness.sort();

  return ConceptSchema.parse(next);
}

// Convenience mutators (pure)

export function addDetermination(doc: Concept, det: Determination): Concept {
  const list = [...doc.shape.triad.particular.determinations, det];
  return updateConcept(doc, {
    triad: { particular: { determinations: list } as any },
  });
}

export function addExemplar(doc: Concept, ref: z.input<typeof EntityRef>): Concept {
  const ex = [...doc.shape.triad.singular.exemplars, EntityRef.parse(ref)];
  // Deduplicate by id
  const seen = new Set<string>();
  const uniq = ex.filter((r) => (seen.has(r.id) ? false : (seen.add(r.id), true)));
  return updateConcept(doc, {
    triad: { singular: { exemplars: uniq } as any },
  });
}

export function advanceWheel(doc: Concept): Concept {
  const order: ConceptStage[] = ["universal", "particular", "singular", "return"];
  const idx = order.indexOf(doc.shape.wheel.stage);
  const nextIdx = (idx + 1) % order.length;
  const nextStage = order[nextIdx];
  const nextCycle = nextStage === "universal" ? (doc.shape.wheel.cycle ?? 0) + 1 : doc.shape.wheel.cycle ?? 0;
  return updateConcept(doc, { wheel: { stage: nextStage, cycle: nextCycle } });
}

// internal
function deepMergeTriad(current: ConceptTriadOut, patch?: Partial<ConceptTriadOut>): ConceptTriadOut {
  if (!patch) return current;
  return {
    universal: {
      ...(current.universal ?? { intent: {} }),
      ...(patch.universal ?? {}),
      intent: {
        ...((current.universal?.intent as Record<string, unknown>) ?? {}),
        ...((patch.universal?.intent as Record<string, unknown>) ?? {}),
      },
    },
    particular: {
      ...(current.particular ?? { determinations: [], constraints: {} }),
      ...(patch.particular ?? {}),
      constraints: {
        ...((current.particular?.constraints as Record<string, unknown>) ?? {}),
        ...((patch.particular?.constraints as Record<string, unknown>) ?? {}),
      },
      determinations:
        patch.particular?.determinations ?? current.particular?.determinations ?? [],
    },
    singular: {
      ...(current.singular ?? { exemplars: [], witness: [] }),
      ...(patch.singular ?? {}),
      exemplars: patch.singular?.exemplars ?? current.singular?.exemplars ?? [],
      witness: patch.singular?.witness ?? current.singular?.witness ?? [],
    },
  };
}
