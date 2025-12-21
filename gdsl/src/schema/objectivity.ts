import { z } from 'zod';

/**
 * Objectivity Artifacts
 *
 * Objectivity is treated as what emerges once the syllogism (truth of Ground)
 * is stabilized into Entity/Property/Aspect and their Essential Relations.
 *
 * Protocol-only: schema-first transport shape.
 */

export const ObjectivityRefSchema = z
	.object({
		kind: z.enum(['entity', 'property', 'aspect']),
		id: z.string().min(1),
	})
	.passthrough();
export type ObjectivityRef = z.infer<typeof ObjectivityRefSchema>;

export const ObjectivityEntitySchema = z
	.object({
		kind: z.literal('entity'),
		id: z.string().min(1).optional(),
		label: z.string().min(1),
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type ObjectivityEntity = z.infer<typeof ObjectivityEntitySchema>;

export const ObjectivityPropertySchema = z
	.object({
		kind: z.literal('property'),
		id: z.string().min(1).optional(),
		entityId: z.string().min(1).optional(),
		name: z.string().min(1),
		value: z.unknown().optional(),
		grounds: z.array(z.string().min(1)).optional(),
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type ObjectivityProperty = z.infer<typeof ObjectivityPropertySchema>;

export const ObjectivityAspectSchema = z
	.object({
		kind: z.literal('aspect'),
		id: z.string().min(1).optional(),
		entityId: z.string().min(1).optional(),
		name: z.string().min(1),
		description: z.string().min(1).optional(),
		grounds: z.array(z.string().min(1)).optional(),
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type ObjectivityAspect = z.infer<typeof ObjectivityAspectSchema>;

export const EssentialRelationSchema = z
	.object({
		kind: z.literal('essentialRelation'),
		id: z.string().min(1).optional(),
		from: ObjectivityRefSchema,
		to: ObjectivityRefSchema,
		relation: z.string().min(1),
		grounds: z.array(z.string().min(1)).optional(),
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type EssentialRelation = z.infer<typeof EssentialRelationSchema>;

export const ObjectivityArtifactSchema = z
	.object({
		kind: z.literal('objectivity'),
		id: z.string().min(1).optional(),
		entities: z.array(ObjectivityEntitySchema).default([]),
		properties: z.array(ObjectivityPropertySchema).default([]),
		aspects: z.array(ObjectivityAspectSchema).default([]),
		essentialRelations: z.array(EssentialRelationSchema).default([]),
		meta: z.record(z.string(), z.unknown()).optional(),
	})
	.passthrough();
export type ObjectivityArtifact = z.infer<typeof ObjectivityArtifactSchema>;
