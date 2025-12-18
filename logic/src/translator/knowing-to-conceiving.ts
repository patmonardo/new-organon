import type { PrintEnvelope } from '@organon/task';

/**
 * Small translator: Prajna (knowing) -> Jnana (conceiving)
 * This is intentionally tiny: accepts a knowing print and returns a conceiving print.
 * Real implementations would subscribe to RealityPipe and incorporate richer adjudication.
 */
export function translateKnowingToConceiving(p: PrintEnvelope): Partial<PrintEnvelope> {
  if (p.kind !== 'knowing') throw new Error('translateKnowingToConceiving only accepts knowing prints');

  const subject = (p as any).payload?.subject ?? (p as any).payload?.trace?.node ?? 'subject:unknown';

  return {
    id: `jnana-${p.id}`,
    kind: 'conceiving',
    role: 'user',
    timestamp: new Date(),
    provenance: { id: `prov-jnana-${p.id}`, origin: 'reflective', createdAt: new Date() } as any,
    derivedFrom: [p.id],
    epistemicLevel: 'conclusive',
    payload: {
      proposition: `Inferred salient change for ${subject}`,
      proof: { steps: ['observe', 'correlate'], evidenceIds: [p.id] },
      narrative: `Auto-conceived from ${p.id}`,
      subject,
    } as any,
  } as Partial<PrintEnvelope>;
}
