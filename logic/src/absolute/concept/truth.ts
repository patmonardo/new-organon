import type { ActiveAspect } from "../../schema/active";

export type Evidence = {
  score?: number; // [0..1]
  weight?: number; // can be negative
  tags?: string[]; // e.g., ['confirm', 'contradict']
  source?: string; // provenance id
};

export type AspectTruth = {
  aspectId: string;
  score: number; // [0..1]
  weight: number;
  provenance: string[];
  rationale?: string;
};

export function evaluateAspectTruth(
  aspect: ActiveAspect,
  evidence: Evidence[] = [],
): AspectTruth {
  const prov = new Set<string>();
  let wSum = 0;
  let sWeighted = 0;
  let contradictions = 0;

  for (const e of evidence) {
    const w = typeof e.weight === "number" ? e.weight : 1;
    const s = clamp01(typeof e.score === "number" ? e.score : defaultScore(e.tags));
    wSum += w;
    sWeighted += w * s;
    if (e.tags?.includes("contradict")) contradictions += 1;
    if (e.source) prov.add(e.source);
  }

  // base score defaults to 0.5 if no evidence; otherwise weighted average
  let score = wSum === 0 ? 0.5 : clamp01(sWeighted / wSum);
  // penalize contradictions softly
  if (contradictions > 0) score = clamp01(score * 0.7 ** contradictions);

  return {
    aspectId: aspect.id,
    score,
    weight: wSum,
    provenance: Array.from(prov),
  };
}

function defaultScore(tags?: string[]): number {
  if (!tags || tags.length === 0) return 0.5;
  let s = 0.5;
  for (const t of tags) {
    if (t === "confirm" || t === "stable" || t === "law") s += 0.2;
    if (t === "weak") s -= 0.1;
    if (t === "contradict") s -= 0.3;
  }
  return clamp01(s);
}

function clamp01(n: number) {
  if (!Number.isFinite(n)) return 0;
  return Math.min(1, Math.max(0, n));
}
