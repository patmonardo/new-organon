/**
 * Power Schema - "Force and Expression"
 *
 * Power represents the dynamic aspect of Essence - the movement of Force expressing itself.
 * It is the bridge between static Essence and dynamic Activity.
 *
 * Dialectical Role:
 * - Force: The internal potential or capacity (In-itself)
 * - Expression: The external manifestation or action (For-another)
 * - Power: The unity of Force and Expression (Actuality)
 *
 * Relationship to Shape Engines:
 * - Used by **Entity** and **Aspect** engines to model capabilities and dynamics
 * - Connects to **Active** schema for runtime execution
 */

import { z } from 'zod';
import { SignatureSchema } from './signature';
import { FacetSchema } from './facet';

// Facets for Power/Force logic
export const PowerFacets = z.object({
  // Force: Internal capacity
  force: z.object({
    type: z.string(),                   // Type of force (e.g., "computational", "logical")
    magnitude: z.number().default(1),   // Strength/Intensity
    potential: z.array(z.string()),     // What it can do
  }).optional(),

  // Expression: External manifestation
  expression: z.object({
    mode: z.enum(['immediate', 'mediated', 'reflected']).default('immediate'),
    target: z.array(z.string()).optional(), // What it acts upon
    effect: z.string().optional(),          // Description of effect
  }).optional(),

  // Manifestation: Concrete realization
  manifestation: z.object({
    status: z.enum(['latent', 'active', 'exhausted']).default('latent'),
    history: z.array(z.string()).optional(),
  }).optional(),

}).catchall(z.any());
export type PowerFacets = z.infer<typeof PowerFacets>;

// Canonical Empowerment schema (Zod)
export const EmpowermentSchema = z.object({
  id: z.string().min(1),
  subject: z.string().min(1),
  scope: z.array(z.string()).optional(), // resource ids or patterns
  actions: z.array(z.string()).min(1),
  weight: z.number().nonnegative().default(1), // action potency
  certainty: z.number().min(0).max(1).default(1), // epistemic confidence
  confidence: z.number().min(0).max(1).optional(),
  provenance: z.any().optional(),
  signatures: z.array(SignatureSchema).optional(),
  facets: z.array(FacetSchema).optional(),

  // Dialectical facets (new)
  powerFacets: PowerFacets.optional(),

  validUntil: z.string().optional(), // ISO timestamp
  meta: z.record(z.string(), z.any()).optional(),
});

export type Empowerment = z.infer<typeof EmpowermentSchema>;

export function createRootEmpowerment(
  opts?: Partial<Empowerment>,
): Empowerment {
  const base: Empowerment = {
    id: opts?.id ?? 'empowerment:root',
    subject: opts?.subject ?? 'system.bootstrap',
    scope: opts?.scope ?? ['*'],
    actions: opts?.actions ?? ['*'],
    weight: opts?.weight ?? 1000,
    certainty: opts?.certainty ?? 1,
    confidence: opts?.confidence ?? 1,
    provenance: opts?.provenance ?? { source: 'system.bootstrap' },
    signatures: opts?.signatures,
    facets: opts?.facets,
    powerFacets: opts?.powerFacets,
    validUntil: opts?.validUntil,
    meta: opts?.meta ?? {},
  } as Empowerment;

  return EmpowermentSchema.parse(base);
}

/**
 * Helper: Get Force structure
 */
export function getForce(power: Empowerment): any | undefined {
  return power.powerFacets?.force;
}

/**
 * Helper: Get Expression structure
 */
export function getExpression(power: Empowerment): any | undefined {
  return power.powerFacets?.expression;
}

