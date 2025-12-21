import { z } from 'zod';

/**
 * Shared GDS boundary primitives.
 *
 * Split out to avoid circular imports between application facades.
 */

export const GdsUserSchema = z.object({
	username: z.string().min(1),
	isAdmin: z.boolean().optional().default(false),
});
export type GdsUser = z.infer<typeof GdsUserSchema>;

export const GdsDatabaseIdSchema = z.string().min(1);
export type GdsDatabaseId = z.infer<typeof GdsDatabaseIdSchema>;

export const GdsGraphNameSchema = z.string().min(1);
export type GdsGraphName = z.infer<typeof GdsGraphNameSchema>;
