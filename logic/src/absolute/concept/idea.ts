import type { Judgment } from '../../schema/judgment';

/**
 * Idea — a lightweight aggregation of judgments into a single analytical unit.
 *
 * Contract (tiny):
 * - Input: judgments[]
 * - Output: ideas[] where each idea groups judgments with a simple score and
 *   a provisional "truthHint" computed from judgment warrants.
 * - Pure, deterministic, and intentionally conservative: idea.score in [0..1]
 */

export type Idea = {
  id: string;
  summary: string;
  judgmentIds: string[];
  score: number; // aggregated confidence in [0..1]
  truthHint?: 'likely' | 'possible' | 'contradicted' | 'unknown';
  // minimal provenance to keep outputs auditable
  provenance?: { algorithm: string; version?: string; inputs?: unknown };
};

export type IdeaConfig = {
  // threshold to promote an idea to a higher-confidence label
  likelyThreshold?: number; // default 0.8
};

export function synthesizeIdeasFromJudgments(
  judgments: Judgment[] | undefined,
  config?: IdeaConfig,
): Idea[] {
  const j = judgments ?? [];
  if (j.length === 0) return [];

  const likelyThreshold = config?.likelyThreshold ?? 0.8;

  // Simple grouping strategy: group by predicate (predicate approximates the
  // ‘‘idea’’ being asserted) and aggregate scores from warrants.confidence.
  const map = new Map<string, { judgments: Judgment[]; scoreSum: number }>();
  for (const jd of j) {
    const key = (jd.shape as any).predicate ?? 'unknown';
    const bucket = map.get(key) ?? { judgments: [], scoreSum: 0 };
    bucket.judgments.push(jd);
    const c = (jd.shape as any).warrant?.confidence;
    bucket.scoreSum += Number.isFinite(c) ? Math.max(0, Math.min(1, c)) : 0;
    map.set(key, bucket);
  }

  const ideas: Idea[] = [];
  let idx = 0;
  for (const [predicate, bucket] of map.entries()) {
    const avg = bucket.judgments.length > 0 ? bucket.scoreSum / bucket.judgments.length : 0;
    const truthHint: Idea['truthHint'] = bucket.judgments.some((x) => (x.shape as any).warrant?.tags?.includes('contradict'))
      ? 'contradicted'
      : avg >= likelyThreshold
      ? 'likely'
      : avg > 0
      ? 'possible'
      : 'unknown';

    ideas.push({
      id: `idea:${predicate}:${idx++}`,
      summary: `Idea aggregated for predicate ${String(predicate)}`,
      judgmentIds: bucket.judgments.map((x) => (x.shape as any).core?.id ?? 'unknown'),
      score: Math.max(0, Math.min(1, avg)),
      truthHint,
      provenance: { algorithm: 'synthesize-ideas-v1', version: '0.1', inputs: { judgments: bucket.judgments.length } },
    });
  }

  return ideas;
}

export default synthesizeIdeasFromJudgments;
