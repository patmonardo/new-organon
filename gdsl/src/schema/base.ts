import { z } from 'zod';

import {
	GdsApplicationFormKindSchema,
	GdsDatabaseIdSchema,
	GdsUserSchema,
} from './common';

/**
 * Shared base shape for all GDS application calls.
 *
 * Rust expects `{ facade, op, ... }` payloads for TS-JSON routing.
 */
export function gdsApplicationBase<F extends z.ZodTypeAny>(facade: F) {
	return z.object({
		kind: GdsApplicationFormKindSchema.optional(),
		facade,
		user: GdsUserSchema,
		databaseId: GdsDatabaseIdSchema,
	});
}
