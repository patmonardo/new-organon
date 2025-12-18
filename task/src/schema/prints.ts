import { z } from 'zod';
import { ProvenanceSchema } from './provenance.js';

export const PrintKind = z.enum([
  'knowing',
  'conceiving',
  'taw',
  'facttrace',
  'ml',
  'graph',
  'proof',
]);

export const PrintRole = z.enum(['kernel', 'user', 'system']);
export const EpistemicLevel = z.enum(['tacit', 'inferred', 'proven', 'conclusive']);

// Ontology discriminator: `monadic` for kernel-level (Being/Presence) prints, `triadic` for
// conceptual/logic-layer (Understanding / Triadic Concept) prints.
export const Ontology = z.enum(['monadic', 'triadic']);
export type Ontology = z.infer<typeof Ontology>;

const KnowingPayloadSchema = z.object({
  modality: z.string().optional(),
  embedding: z.array(z.number()).optional(),
  trace: z.record(z.any()).optional(),
  summary: z.string().optional(),
});

const ConceivingProofSchema = z.object({
  steps: z.array(z.string()),
  evidenceIds: z.array(z.string()).optional(),
  rationale: z.string().optional(),
});

const ConceivingPayloadSchema = z.object({
  proposition: z.string(),
  proof: ConceivingProofSchema.optional(),
  narrative: z.string().optional(),
});

const PrintEnvelopeBase = z.object({
  id: z.string(),
  kind: PrintKind,
  role: PrintRole,
  timestamp: z.preprocess((val) => (typeof val === 'string' ? new Date(val) : val), z.date()).default(() => new Date()),
  provenance: ProvenanceSchema.optional(),
  derivedFrom: z.array(z.string()).optional(),
  schemaVersion: z.string().default('1.0'),
  metadata: z.record(z.any()).optional(),
  epistemicLevel: EpistemicLevel.optional(),
  confidence: z.number().min(0).max(1).optional(),
  // Optional ontology discriminator and phases (triadic phases like Being/Nothing/Becoming etc.)
  ontology: Ontology.optional(),
  phases: z.array(z.string()).optional(),
});

const KnowingPrintSchema = PrintEnvelopeBase.extend({
  kind: z.literal('knowing'),
  payload: KnowingPayloadSchema,
});

const ConceivingPrintSchema = PrintEnvelopeBase.extend({
  kind: z.literal('conceiving'),
  payload: ConceivingPayloadSchema,
});

const GenericPrintSchema = PrintEnvelopeBase.extend({
  kind: z.enum(['taw', 'facttrace', 'ml', 'graph', 'proof']),
  payload: z.any(),
});

export const PrintEnvelopeSchema = z.discriminatedUnion('kind', [
  KnowingPrintSchema,
  ConceivingPrintSchema,
  GenericPrintSchema,
]);

export type PrintEnvelope = z.infer<typeof PrintEnvelopeSchema>;

export { KnowingPayloadSchema, ConceivingPayloadSchema };
