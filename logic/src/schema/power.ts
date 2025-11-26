import { z } from 'zod';
import { SignatureSchema } from './signature';
import { FacetSchema } from './facet';

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
  validUntil: z.string().optional(), // ISO timestamp
  meta: z.record(z.any()).optional(),
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
    validUntil: opts?.validUntil,
    meta: opts?.meta ?? {},
  } as Empowerment;

  return EmpowermentSchema.parse(base);
}
