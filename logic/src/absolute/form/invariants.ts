import { z } from 'zod';

import { KernelRunResultSchema } from './kernel-port';

export const FACT_STORE_MODES = [
  'reflection',
  'logic',
  'transcendental',
] as const;
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

export const DIALECTICAL_TRIADS = {
  reflection: ['posting', 'external', 'determining'] as const,
  logic: ['identity', 'difference', 'contradiction'] as const,
  transcendental: ['ground', 'condition', 'facticity'] as const,
  objectivity: ['thing', 'world', 'relation'] as const,
} as const;

/**
 * Discursive invariants (GDSL)
 *
 * These schemas/validators intentionally stay structural and dependency-light.
 * They provide a shared way to validate that “discourse” (events, meta, results)
 * is well-formed without entangling Model or Logic.
 */

export const FactStoreOpSchema = z.union([
  z.literal('assert'),
  z.literal('retract'),
  z.literal('revise'),
  z.literal('index'),
  z.literal('project'),
]);

export const FactStoreModeSchema = z.enum(FACT_STORE_MODES);

export const FactStoreInfoSchema = z
  .object({
    mode: FactStoreModeSchema.optional(),
    store: z.string().optional(),
    op: FactStoreOpSchema.optional(),
    kind: z.string().optional(),
    ids: z.array(z.string()).optional(),
    note: z.string().optional(),
  })
  .strict();

export const DialecticalTagSchema = z
  .object({
    layer: z.string(),
    rule: z.string(),
  })
  .strict();

export const DialecticalProgressionSchema = z
  .object({
    axis: z.string(),
    from: DialecticalTagSchema,
    to: DialecticalTagSchema,
  })
  .strict();

export const DialecticalInfoSchema = z
  .object({
    tags: z.array(DialecticalTagSchema).optional(),
    progressions: z.array(DialecticalProgressionSchema).optional(),
    note: z.string().optional(),
  })
  .strict();

/**
 * `EventMeta` is intentionally open (passthrough), but its known structural fields
 * should remain valid when present.
 */
export const EventMetaSchema = z
  .object({
    factStore: FactStoreInfoSchema.optional(),
    dialectic: DialecticalInfoSchema.optional(),
  })
  .passthrough();

export const TraceEventSchema = z
  .object({
    kind: z.string(),
    payload: z.unknown().optional(),
    meta: EventMetaSchema.optional(),
  })
  .strict();

/**
 * Optional stricter invariant for kernel results.
 *
 * - If `ok` then `error` must be absent.
 * - If `!ok` then `error` must be present.
 */
export const KernelRunResultStrictSchema = KernelRunResultSchema.superRefine(
  (result, ctx) => {
    if (result.ok) {
      if (result.error !== undefined) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message:
            'KernelRunResult invariant violated: ok=true requires error to be undefined',
        });
      }
    } else {
      if (result.error === undefined) {
        ctx.addIssue({
          code: z.ZodIssueCode.custom,
          message:
            'KernelRunResult invariant violated: ok=false requires error to be defined',
        });
      }
    }
  },
);

export type ParsedFactStoreInfo = z.infer<typeof FactStoreInfoSchema>;
export type ParsedEventMeta = z.infer<typeof EventMetaSchema>;
export type ParsedTraceEvent = z.infer<typeof TraceEventSchema>;
export type ParsedFactStoreOp = FactStoreOp;

export function parseEventMeta(meta: unknown): ParsedEventMeta {
  return EventMetaSchema.parse(meta);
}

export function parseTraceEvent(event: unknown): ParsedTraceEvent {
  return TraceEventSchema.parse(event);
}

export function parseKernelRunResultStrict(result: unknown) {
  return KernelRunResultStrictSchema.parse(result);
}
