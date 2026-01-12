import { z } from 'zod';

import { GdsApplicationCallSchema } from './application';

/**
 * TS-JSON envelope used by the Rust NAPI facade.
 *
 * Mirrors: gds/src/applications/services/tsjson_napi.rs
 */

export const GdsTsjsonOkResponseSchema = z.object({
	ok: z.literal(true),
	op: z.string().min(1),
	data: z.unknown(),
});
export type GdsTsjsonOkResponse = z.infer<typeof GdsTsjsonOkResponseSchema>;

export const GdsTsjsonErrorSchema = z.object({
	code: z.string().min(1),
	message: z.string().min(1),
});
export type GdsTsjsonError = z.infer<typeof GdsTsjsonErrorSchema>;

export const GdsTsjsonErrResponseSchema = z.object({
	ok: z.literal(false),
	op: z.string(),
	error: GdsTsjsonErrorSchema,
});
export type GdsTsjsonErrResponse = z.infer<typeof GdsTsjsonErrResponseSchema>;

export const GdsTsjsonResponseSchema = z.union([
	GdsTsjsonOkResponseSchema,
	GdsTsjsonErrResponseSchema,
]);
export type GdsTsjsonResponse = z.infer<typeof GdsTsjsonResponseSchema>;

/**
 * TS-JSON request envelope.
 *
 * Notes:
 * - The kernel prefers `{ facade, op, ... }` routing.
 * - `ping` and `version` are top-level ops (no facade).
 */
export const GdsTsjsonPingRequestSchema = z
	.object({
		op: z.literal('ping'),
		nonce: z.unknown().optional(),
	})
	.passthrough();
export type GdsTsjsonPingRequest = z.infer<typeof GdsTsjsonPingRequestSchema>;

export const GdsTsjsonVersionRequestSchema = z
	.object({
		op: z.literal('version'),
	})
	.passthrough();
export type GdsTsjsonVersionRequest = z.infer<typeof GdsTsjsonVersionRequestSchema>;

export const GdsTsjsonRequestSchema = z.union([
	GdsApplicationCallSchema,
	GdsTsjsonPingRequestSchema,
	GdsTsjsonVersionRequestSchema,
]);
export type GdsTsjsonRequest = z.infer<typeof GdsTsjsonRequestSchema>;


