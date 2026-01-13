import { z } from 'zod';

// ------------------------------------------------------------
// Kernel-facing FormShape + ApplicationForm (JSON-first wire IR)
// ------------------------------------------------------------
// This mirrors the ADR-0011 contract: TS authors/validates, Rust executes.

// Versioning keeps TS/Rust in lockstep. Example: "fs-0.1.0".
export const FormShapeVersionSchema = z.string().min(1);
export type FormShapeVersion = z.infer<typeof FormShapeVersionSchema>;

export const KernelExecutionModeSchema = z.enum([
  'stream',
  'stats',
  'mutate',
  'write',
  'estimate',
]);
export type KernelExecutionMode = z.infer<typeof KernelExecutionModeSchema>;

export const KernelGraphHandleSchema = z.string().min(1);
export type KernelGraphHandle = z.infer<typeof KernelGraphHandleSchema>;

// Morph operator chain (kernel executes patterns; steps are optional planning metadata).
export const KernelFormOpSchema = z.string().min(1);
export type KernelFormOp = z.infer<typeof KernelFormOpSchema>;

export const KernelMorphStepSchema = z.discriminatedUnion('kind', [
  z
    .object({
      kind: z.literal('form'),
      op: KernelFormOpSchema,
      params: z.record(z.string(), z.unknown()).optional(),
    })
    .passthrough(),
  z
    .object({
      kind: z.literal('judge'),
      moment: z
        .enum(['existence', 'reflection', 'necessity', 'concept'])
        .optional(),
    })
    .passthrough(),
  z
    .object({
      kind: z.literal('syllogize'),
    })
    .passthrough(),
]);
export type KernelMorphStep = z.infer<typeof KernelMorphStepSchema>;

export const KernelMorphSchema = z
  .object({
    patterns: z.array(KernelFormOpSchema).min(1),
    steps: z.array(KernelMorphStepSchema).optional(),
  })
  .passthrough();
export type KernelMorph = z.infer<typeof KernelMorphSchema>;

// Core FormShape (kernel-facing; distinct from UI FormShape in form.ts).
export const KernelFormShapeSchema = z
  .object({
    id: z.string(),
    version: FormShapeVersionSchema.default('fs-0.1.0'),
    kind: z.string().optional(),
    shape: z
      .object({
        required_fields: z.array(z.string()).optional(),
        optional_fields: z.array(z.string()).optional(),
        type_constraints: z.record(z.string(), z.string()).optional(),
        validation_rules: z.record(z.string(), z.string()).optional(),
      })
      .passthrough()
      .optional(),
    context: z
      .object({
        dependencies: z.array(z.string()).optional(),
        execution_order: z.array(z.string()).optional(),
        runtime_strategy: z.string().optional(),
        conditions: z.array(z.string()).optional(),
      })
      .passthrough()
      .optional(),
    morph: KernelMorphSchema,
    content: z.unknown().optional(),
    provenance: z.unknown().optional(),
    constraints: z.record(z.string(), z.unknown()).optional(),
    metrics: z.record(z.string(), z.unknown()).optional(),
    controls: z
      .object({
        mode: KernelExecutionModeSchema.optional(),
        retries: z.number().int().nonnegative().optional(),
        timeoutMs: z.number().int().positive().optional(),
        compensate: z.boolean().optional(),
      })
      .optional(),
  })
  .passthrough();
export type KernelFormShape = z.infer<typeof KernelFormShapeSchema>;
// Alias for legacy naming in relative/absolute logic layers.
export type KernelFormProgram = KernelFormShape;

// Graph operations that a Program/ApplicationForm can drive.
const GraphOperationProcedureSchema = z.object({
  kind: z.literal('procedure'),
  name: z.string().min(1),
  args: z.record(z.string(), z.unknown()).optional(),
  graphRef: KernelGraphHandleSchema.optional(),
});

const GraphOperationMlSchema = z.object({
  kind: z.literal('ml'),
  model: z.string().min(1),
  task: z.string().min(1),
  features: z.array(z.string()).optional(),
  target: z.string().optional(),
  graphRef: KernelGraphHandleSchema.optional(),
});

const GraphOperationHybridSchema = z.object({
  kind: z.literal('hybrid'),
  plan: z.array(z.record(z.string(), z.unknown())).optional(),
  graphRef: KernelGraphHandleSchema.optional(),
});

const GraphOperationGdslSchema = z.object({
  kind: z.literal('gdsl'),
  text: z.string().min(1).optional(),
  ast: z.unknown().optional(),
  graphRef: KernelGraphHandleSchema.optional(),
});

export const KernelGraphOperationSchema = z.discriminatedUnion('kind', [
  GraphOperationProcedureSchema,
  GraphOperationMlSchema,
  GraphOperationHybridSchema,
  GraphOperationGdslSchema,
]);
export type KernelGraphOperation = z.infer<typeof KernelGraphOperationSchema>;

export const KernelApplicationFormSchema = z
  .object({
    id: z.string(),
    version: z.string().default('app-0.1.0'),
    graph: KernelGraphHandleSchema.optional(),
    ops: z.array(KernelGraphOperationSchema).min(1),
    inputs: z.record(z.string(), z.unknown()).optional(),
    bindings: z.record(z.string(), z.unknown()).optional(),
    controls: z
      .object({
        mode: KernelExecutionModeSchema.optional(),
        retries: z.number().int().nonnegative().optional(),
        timeoutMs: z.number().int().positive().optional(),
        compensate: z.boolean().optional(),
      })
      .optional(),
    otlp: z.record(z.string(), z.unknown()).optional(),
    return: z
      .enum(['formshape', 'shapestream'])
      .optional()
      .default('formshape'),
    artifacts: z.record(z.string(), z.unknown()).optional(),
  })
  .passthrough();
export type KernelApplicationForm = z.infer<typeof KernelApplicationFormSchema>;

// Streaming envelope for long-running graph/program executions.
export const KernelShapeStreamSchema = z.object({
  stream: z.array(KernelFormShapeSchema).optional(),
  progress: z.record(z.string(), z.unknown()).optional(),
  metrics: z.record(z.string(), z.unknown()).optional(),
});
export type KernelShapeStream = z.infer<typeof KernelShapeStreamSchema>;
