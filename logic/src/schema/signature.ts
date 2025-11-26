import { z } from 'zod';

export const SignatureSchema = z.object({
  id: z.string().min(1),
  issuer: z.string().min(1),
  issuedAt: z.string().optional(), // ISO timestamp
  algorithm: z.string().optional(),
  signature: z.string().min(1),
  targetId: z.string().optional(), // id of the signed object (empowerment/facet/etc)
  validUntil: z.string().optional(),
  meta: z.record(z.string(), z.any()).optional(),
});

export type Signature = z.infer<typeof SignatureSchema>;
