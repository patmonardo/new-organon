import { z } from 'zod';
import { EntitySchema } from '@schema';
import { RelationSchema } from '@schema';
import { PropertySchema } from '@schema';
import { ContextSchema } from '@schema';
import { MorphSchema } from '@schema';
import { WorldSchema } from '@schema';
// ShapeSchema removed - use FormShapeSchema from @schema/form for form structures
import { ContentSchema } from '@schema';
import { JudgmentSchema } from '@schema';
import { SyllogismSchema } from '@schema';

// Inputs accepted by the Form Processor. Keep defaults for incremental adoption.
export const ProcessorInputs = z.object({
  // shapes removed - use form structures directly via FormShapeRepository
  entities: z.array(EntitySchema).default([]),
  properties: z.array(PropertySchema).default([]),
  contexts: z.array(ContextSchema).default([]),
  morphs: z.array(MorphSchema).default([]),
  relations: z.array(RelationSchema).default([]),
  content: z.array(ContentSchema).default([]),
  judgments: z.array(JudgmentSchema).default([]),
  syllogisms: z.array(SyllogismSchema).default([]),
});
export type ProcessorInputs = z.infer<typeof ProcessorInputs>;

// add shared runtime options for the processor (used by FormProcessor)
export type ProcessorRunOptions = {
  projectContent?: boolean;
  contentIndexSource?: 'inputs';
  deriveSyllogistic?: false;
};

const GrossByThing = z.record(z.string(), z.number().int().nonnegative());
export const ProcessorSnapshot = z.object({
  world: WorldSchema,
  indexes: z
    .object({
      content: z
        .object({
          subtleWorldTotal: z.number().int().nonnegative().default(0),
          grossByThing: GrossByThing.default({}),
        })
        .default({ subtleWorldTotal: 0, grossByThing: {} }),
      // room for logic indexes later
    })
    .default({ content: { subtleWorldTotal: 0, grossByThing: {} } }),
});
export type ProcessorSnapshot = z.infer<typeof ProcessorSnapshot>;
