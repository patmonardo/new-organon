import { z } from 'zod';

// ==========================================
// DISCURSIVITY RULES (Essence:Reflection)
// ==========================================

// Rules of Reflective Consciousness (Reflection)
export const ReflectionRuleSchema = z.enum([
  'posting',
  'external',
  'determining',
]);
export type ReflectionRule = z.infer<typeof ReflectionRuleSchema>;

// Rules of Logic (within Context)
export const LogicRuleSchema = z.enum([
  'identity',
  'difference',
  'contradiction',
]);
export type LogicRule = z.infer<typeof LogicRuleSchema>;

// Rules of Synthesis (Ground)
export const SynthesisRuleSchema = z.enum([
  'ground',
  'condition',
  'facticity',
]);
export type SynthesisRule = z.infer<typeof SynthesisRuleSchema>;

// Objective Rules (Objectivity: Thing/World/Relation)
export const ObjectivityRuleSchema = z.enum(['thing', 'world', 'relation']);
export type ObjectivityRule = z.infer<typeof ObjectivityRuleSchema>;

export const DiscursiveLayerSchema = z.enum([
  'shape',
  'context',
  'morph',
  // Objective carriers (Thing/Law/etc)
  'entity',
  'property',
  // Reserved: explicit relation engine/layer
  'relation',
]);
export type DiscursiveLayer = z.infer<typeof DiscursiveLayerSchema>;

export const DiscursiveRuleSchema = z.union([
  ReflectionRuleSchema,
  LogicRuleSchema,
  SynthesisRuleSchema,
  ObjectivityRuleSchema,
]);
export type DiscursiveRule = z.infer<typeof DiscursiveRuleSchema>;

export const DiscursiveRuleTagSchema = z.object({
  layer: DiscursiveLayerSchema,
  rule: DiscursiveRuleSchema,
});
export type DiscursiveRuleTag = z.infer<typeof DiscursiveRuleTagSchema>;

// ==========================================
// TRIAD-OF-TRIADS PROGRESSION (Explicit)
// ==========================================

/**
 * We treat Shape/Context/Morph as a triad of triads.
 *
 * - Vertical progression: within a triad (e.g. posting→external→determining)
 * - Horizontal progression: across layers at the “same index” (e.g. posting→identity→ground)
 * - Diagonal progression: the “result” vector (e.g. determining→contradiction→facticity)
 */
export const DiscursiveAxisSchema = z.enum(['vertical', 'horizontal', 'diagonal']);
export type DiscursiveAxis = z.infer<typeof DiscursiveAxisSchema>;

export const ReflectionRuleOrder = ['posting', 'external', 'determining'] as const;
export const LogicRuleOrder = ['identity', 'difference', 'contradiction'] as const;
export const SynthesisRuleOrder = ['ground', 'condition', 'facticity'] as const;

const LayerOrders = {
  shape: ReflectionRuleOrder,
  context: LogicRuleOrder,
  morph: SynthesisRuleOrder,
} as const;

export function nextVertical(
  tag: DiscursiveRuleTag,
): DiscursiveRuleTag | null {
  const order = (LayerOrders as any)[tag.layer] as readonly string[] | undefined;
  if (!order) return null;
  const idx = (order as readonly string[]).indexOf(tag.rule);
  if (idx < 0 || idx + 1 >= order.length) return null;
  return { layer: tag.layer, rule: order[idx + 1] as any };
}

export function horizontalAtIndex(index: 0 | 1 | 2): DiscursiveRuleTag[] {
  return [
    { layer: 'shape', rule: ReflectionRuleOrder[index] },
    { layer: 'context', rule: LogicRuleOrder[index] },
    { layer: 'morph', rule: SynthesisRuleOrder[index] },
  ];
}

export function diagonalVector(): DiscursiveRuleTag[] {
  return [
    { layer: 'shape', rule: 'determining' },
    { layer: 'context', rule: 'contradiction' },
    { layer: 'morph', rule: 'facticity' },
  ];
}

export function triadOfTriadsMatrix(): {
  shape: typeof ReflectionRuleOrder;
  context: typeof LogicRuleOrder;
  morph: typeof SynthesisRuleOrder;
} {
  return {
    shape: ReflectionRuleOrder,
    context: LogicRuleOrder,
    morph: SynthesisRuleOrder,
  };
}

export function toDialecticalInfo(tags: DiscursiveRuleTag[]) {
  const parsed = z.array(DiscursiveRuleTagSchema).parse(tags);
  return {
    tags: parsed,
    progressions: [
      {
        axis: 'diagonal',
        from: { layer: 'shape', rule: 'determining' },
        to: { layer: 'morph', rule: 'facticity' },
      },
    ],
  };
}

// Optional dialectical annotation for Shape→Context→Morph
export const FormDialecticSchema = z.object({
  shape: z
    .object({
      rules: z.array(ReflectionRuleSchema).optional(),
    })
    .optional(),
  context: z
    .object({
      rules: z.array(LogicRuleSchema).optional(),
    })
    .optional(),
  morph: z
    .object({
      rules: z.array(SynthesisRuleSchema).optional(),
      telos: z.literal('facticity').optional(),
    })
    .optional(),
});
export type FormDialectic = z.infer<typeof FormDialecticSchema>;

// ==========================================
// CRUD-ISH FORM SHAPE → REFLECTION (Heuristic)
// ==========================================

/**
 * Heuristic: interpret common FormShape “CRUD schema” parts as reflection moments.
 *
 * - Posting: the form is posited as a named unit (id/name/schemaId)
 * - External: the form is externally arranged/presented (layout)
 * - Determining: the form determinates content via fields/constraints
 */
export function inferReflectionRulesFromCrudFormShape(input: unknown): ReflectionRule[] {
  if (!input || typeof input !== 'object' || Array.isArray(input)) return [];
  const obj = input as Record<string, unknown>;

  const rules = new Set<ReflectionRule>();

  // Posting: the unit is posited (identity as a thing)
  if (typeof obj.id === 'string' || typeof obj.name === 'string' || typeof obj.schemaId === 'string') {
    rules.add('posting');
  }

  // External: layout/presentation as external reflection
  if (obj.layout && typeof obj.layout === 'object') {
    rules.add('external');
  }

  // Determining: fields/validation as determining reflection
  if (Array.isArray(obj.fields) && obj.fields.length > 0) {
    rules.add('determining');
  }

  // Preserve canonical order
  return ReflectionRuleOrder.filter((r) => rules.has(r));
}

export function inferDialecticFromCrudFormShape(input: unknown): FormDialectic {
  const shapeRules = inferReflectionRulesFromCrudFormShape(input);
  return {
    shape: shapeRules.length ? { rules: shapeRules } : undefined,
  };
}
