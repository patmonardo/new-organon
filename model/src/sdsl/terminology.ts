/**
 * Terminology (Transcendentally Located)
 *
 * Single source of truth for the Model middleware vocabulary that TAW can depend on.
 *
 * Important constraints:
 * - Keep this dependency-light and structural (no imports from @organon/logic/@organon/task).
 * - Prefer forward-compatible string unions (known terms + extension via string branding).
 */

export const FACT_STORE_MODES = ['reflection', 'logic', 'transcendental'] as const;
export type FactStoreMode = (typeof FACT_STORE_MODES)[number];

export type FactStoreOp = 'assert' | 'retract' | 'revise' | 'index' | 'project';

export type FactStoreInfo = {
  mode?: FactStoreMode;
  store?: string;
  op?: FactStoreOp;
  kind?: string;
  ids?: string[];
  note?: string;
};

export type DialecticalLayer =
  | 'shape'
  | 'context'
  | 'morph'
  | 'entity'
  | 'property'
  | 'relation'
  | (string & {});

export type DialecticalRule =
  | 'posting'
  | 'external'
  | 'determining'
  | 'identity'
  | 'difference'
  | 'contradiction'
  | 'ground'
  | 'condition'
  | 'facticity'
  | 'thing'
  | 'world'
  | 'relation'
  | (string & {});

export type DialecticalTag = { layer: DialecticalLayer; rule: DialecticalRule };

export type DialecticalProgression = {
  axis: string;
  from: DialecticalTag;
  to: DialecticalTag;
};

export type DialecticalInfo = {
  tags?: DialecticalTag[];
  progressions?: DialecticalProgression[];
  note?: string;
};

export type EventMeta = Record<string, unknown> & {
  factStore?: FactStoreInfo;
  dialectic?: DialecticalInfo;
};

/**
 * Dialectical Cube seeds (3^3 horizon)
 *
 * This is intentionally a vocabulary seed (terms), not a normative engine.
 */
export const DIALECTICAL_TRIADS = {
  reflection: ['posting', 'external', 'determining'] as const,
  logic: ['identity', 'difference', 'contradiction'] as const,
  transcendental: ['ground', 'condition', 'facticity'] as const,
  objectivity: ['thing', 'world', 'relation'] as const,
} as const;
