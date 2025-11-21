import type { AspectTruth } from './truth';

export type Stage = {
  aspectId: string;
  level: number; // 1..11
  name: string;
  description?: string;
  evidenceScore: number; // raw truth score used for assessment
  provenance?: {
    algorithm: string;
    version?: string;
    inputs?: unknown;
    timestamp?: string;
  };
};

export type QualQuantShape = {
  [relationId: string]: { tags?: string[]; score?: number; weight?: number };
};

// Default, minimal 11-stage placeholders. Users can provide custom names via config.
const DEFAULT_BHUMI_NAMES = [
  'bhumi-1',
  'bhumi-2',
  'bhumi-3',
  'bhumi-4',
  'bhumi-5',
  'bhumi-6',
  'bhumi-7',
  'bhumi-8',
  'bhumi-9',
  'bhumi-10',
  'bhumi-11',
];

export type Config = {
  names?: string[]; // optional custom 11 names
  // future: thresholds or richer logic can be provided here
};

/**
 * A very small, deterministic mapper that assigns an aspect to one of 11 bhumis
 * based on the aspect truth score in [0..1]. This is intentionally conservative
 * and meant as a seed for richer cultural/phenomenal mappings (Samyama) later.
 */
export function assessForAspects(
  truths: AspectTruth[] | undefined,
  _qualquant: QualQuantShape | undefined,
  config: Config | undefined,
): Stage[] {
  const names = (config?.names ?? DEFAULT_BHUMI_NAMES).slice(0, 11);
  // ensure we have 11 names
  while (names.length < 11) names.push(`bhumi-${names.length + 1}`);

  if (!truths || truths.length === 0) return [];

  return truths.map((t) => {
    // clamp score between 0..1, default to 0
    const s = Number.isFinite(t.score) ? Math.max(0, Math.min(1, t.score)) : 0;
    // map to 1..11 by ceil(score*11), but ensure score=0 maps to 1
    const level = Math.max(1, Math.min(11, Math.ceil(s * 11 || 1)));
    const name = names[level - 1] ?? `bhumi-${level}`;
    return {
      aspectId: t.aspectId,
      level,
      name,
      description: undefined,
      evidenceScore: s,
    } as Stage;
  });
}

export default assessForAspects;
