import type { AspectTruth } from './truth';

export type AttentionEntry = {
  aspectId: string;
  weight: number; // normalized 0..1
  reason?: string;
};

export type AttentionConfig = {
  // weight multiplier applied to qualquant weights when present
  qualquantBias?: number;
  // conflict penalty applied when an aspect has 'contradict' tag
  contradictionPenalty?: number;
};

/**
 * Simple attention-weighted aggregator that produces a normalized weight per aspect
 * from AspectTruth and optional qualquant hints. Deterministic and pure.
 */
export function computeAttentionWeights(
  truths: AspectTruth[] | undefined,
  qualquant: Record<string, { score?: number; weight?: number; tags?: string[] }>|undefined,
  config?: AttentionConfig,
): AttentionEntry[] {
  const qBias = config?.qualquantBias ?? 1.0;
  const contradictionPenalty = config?.contradictionPenalty ?? 0.5;
  if (!truths || truths.length === 0) return [];

  // raw weights: base on truth.score, optionally modified by qualquant weight/score
  const raws = truths.map((t) => {
    const q = qualquant?.[t.aspectId];
    let w = Number.isFinite(t.score) ? Math.max(0, Math.min(1, t.score)) : 0;
    if (q) {
      const qw = q.weight;
      if (typeof qw === 'number' && Number.isFinite(qw)) w *= qw * qBias;
      const qs = q.score;
      if (typeof qs === 'number' && Number.isFinite(qs)) w = (w + Math.max(0, Math.min(1, qs))) / 2;
      if ((q.tags ?? []).includes('contradict')) w *= contradictionPenalty;
    }
    return { aspectId: t.aspectId, raw: w } as any;
  });

  const max = Math.max(...raws.map((r) => r.raw), 1e-6);
  return raws.map((r) => ({ aspectId: r.aspectId, weight: Math.max(0, Math.min(1, r.raw / max)) }));
}

export default computeAttentionWeights;
