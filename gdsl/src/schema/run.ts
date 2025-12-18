import { z } from 'zod';

import { GdslAlgorithmSpecSchema } from './algorithm';
import { GdslGraphRefSchema } from './graph';

export const GdslRunModeSchema = z.enum(['stream', 'write', 'stats']);
export type GdslRunMode = z.infer<typeof GdslRunModeSchema>;

export const GdslRunRequestSchema = z.object({
	kind: z.literal('run'),
	algorithm: GdslAlgorithmSpecSchema,
	graph: GdslGraphRefSchema,
	mode: GdslRunModeSchema,
	concurrency: z.number().int().positive().optional(),
});
export type GdslRunRequest = z.infer<typeof GdslRunRequestSchema>;

export const GdslResultHandleSchema = z.object({
	id: z.string().min(1),
	kind: z.enum(['node_properties', 'paths', 'stats']),
	graph: GdslGraphRefSchema.optional(),
});
export type GdslResultHandle = z.infer<typeof GdslResultHandleSchema>;
