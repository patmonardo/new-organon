import { z } from 'zod';

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
	op: z.string().optional().default(''),
	error: GdsTsjsonErrorSchema,
});
export type GdsTsjsonErrResponse = z.infer<typeof GdsTsjsonErrResponseSchema>;

export const GdsTsjsonResponseSchema = z.union([
	GdsTsjsonOkResponseSchema,
	GdsTsjsonErrResponseSchema,
]);
export type GdsTsjsonResponse = z.infer<typeof GdsTsjsonResponseSchema>;
