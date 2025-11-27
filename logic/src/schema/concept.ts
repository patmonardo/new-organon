/**
 * Concept Schema - "Logic of the Concept" (Subject, Object, Idea)
 *
 * Concept represents the third phase of the dialectic: Freedom/Truth.
 * It is the unity of Being and Essence - the self-determining universal.
 *
 * Dialectical Role:
 * - Subject: Universal, Particular, Singular (UPS)
 * - Object: Mechanism, Chemism, Teleology
 * - Idea: Life, Cognition, Absolute Idea
 *
 * Relationship to Form Engines:
 * - Concept is the logical domain for the **Morph** Form Engine
 * - Morph manifests the active self-determination of the Concept
 * - Concept -> Absolute (The end of logic)
 *
 * Schema Structure:
 * - Extends Shape (Logic in itself)
 * - Facets: Subject (UPS), Object, Idea structures
 * - Signature: Free transitions
 */

import { z } from "zod";
import { BaseState, Type } from "./base";
import { ShapeCore, ShapeSchema, ShapeSignature } from "./shape";
import { EntityRef } from "./entity";

// Core
export const ConceptCore = ShapeCore.extend({
  type: Type.default("concept.Concept"),
});
export type ConceptCore = z.infer<typeof ConceptCore>;

// --- Subject Logic (UPS) ---

export const ConceptUniversal = z.object({
  title: z.string().optional(),
  law: z.string().optional(),
  intent: z.record(z.string(), z.any()).default({}),
});

export const ConceptParticular = z.object({
  determinations: z.array(z.object({
    key: z.string(),
    value: z.any(),
    note: z.string().optional(),
  })).default([]),
  constraints: z.record(z.string(), z.any()).default({}),
});

export const ConceptSingular = z.object({
  exemplars: z.array(EntityRef).default([]),
  witness: z.array(z.string()).default([]),
});

export const ConceptTriad = z.object({
  universal: ConceptUniversal.default({ intent: {} }),
  particular: ConceptParticular.default({ determinations: [], constraints: {} }),
  singular: ConceptSingular.default({ exemplars: [], witness: [] }),
});

// --- Object Logic ---

export const ConceptObject = z.object({
  mechanism: z.object({
    objects: z.array(z.string()),
    forces: z.array(z.string()),
  }).optional(),
  chemism: z.object({
    affinity: z.string().optional(),
    neutrality: z.string().optional(),
  }).optional(),
  teleology: z.object({
    purpose: z.string().optional(),
    means: z.array(z.string()),
    realization: z.string().optional(),
  }).optional(),
}).optional();

// --- Idea Logic ---

export const ConceptIdea = z.object({
  life: z.object({
    organism: z.string().optional(),
    process: z.string().optional(),
  }).optional(),
  cognition: z.object({
    truth: z.string().optional(),
    good: z.string().optional(),
  }).optional(),
  absolute: z.string().optional(),
}).optional();

// --- Facets ---

export const ConceptFacets = z.object({
  // Core dialectical state
  dialecticState: z.any().optional(),

  // Subject Logic (UPS Triad)
  subject: ConceptTriad.optional(),

  // Object Logic
  object: ConceptObject,

  // Idea Logic
  idea: ConceptIdea,

  // Wheel (Process)
  wheel: z.object({
    stage: z.enum(["universal", "particular", "singular", "return"]).default("universal"),
    cycle: z.number().int().nonnegative().default(0),
  }).optional(),

  // Integration with Morph Engine
  morphRef: z.string().optional(),

}).catchall(z.any());
export type ConceptFacets = z.infer<typeof ConceptFacets>;

// Signature
export const ConceptSignature = ShapeSignature.extend({
  // Concept-specific operations
});
export type ConceptSignature = z.infer<typeof ConceptSignature>;

// Document
const ConceptDoc = z.object({
  core: ConceptCore,
  state: BaseState.default({}),
  signature: ConceptSignature.optional(),
  facets: z.record(z.string(), z.any()).default({}),
});

export const ConceptSchema = ShapeSchema.extend({
  shape: ConceptDoc,
});
export type Concept = z.infer<typeof ConceptSchema>;

// Helpers
function genId() {
  return `concept:${Date.now().toString(36)}:${Math.floor(Math.random() * 1e6)
    .toString(36)
    .padStart(4, "0")}`;
}

type CreateConceptInput = {
  id?: string;
  name?: string;
  facets?: Record<string, unknown>;
  state?: z.input<typeof BaseState>;
};

export function createConcept(input: CreateConceptInput): Concept {
  const id = input.id ?? genId();
  const draft = {
    shape: {
      core: { id, type: "concept.Concept", name: input.name },
      state: input.state ?? {},
      facets: input.facets ?? {},
    },
  };
  return ConceptSchema.parse(draft);
}

/**
 * Helper: Extract Subject (Triad) structure
 */
export function getSubject(concept: Concept): any | undefined {
  return (concept.shape.facets as any)?.subject;
}

/**
 * Helper: Extract Object structure
 */
export function getObject(concept: Concept): any | undefined {
  return (concept.shape.facets as any)?.object;
}

/**
 * Helper: Extract Idea structure
 */
export function getIdea(concept: Concept): any | undefined {
  return (concept.shape.facets as any)?.idea;
}

