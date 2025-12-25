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

/**
 * ApplicationForm (wire/program envelope marker)
 *
 * Optional discriminator to make it explicit that a request payload is a submitted
 * Application Form (client ENC → kernel DEC → execution).
 *
 * This is intentionally optional to keep the wire format permissive while the
 * protocol is still evolving.
 */
export const GdsApplicationFormKindSchema = z.literal('ApplicationForm');
export type GdsApplicationFormKind = z.infer<typeof GdsApplicationFormKindSchema>;


