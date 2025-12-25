import type { KernelPort, KernelRunRequest, KernelRunResult } from '../kernel-api';

import {
	GdsApplicationCallSchema,
	type GdsApplicationCall,
} from '../schema/application';
import { GdsTsjsonResponseSchema } from '../schema/tsjson';

export type TsjsonInvoker = (requestJson: string) => string | Promise<string>;

export type GdsTsjsonKernelPortOptions = {
	/** Optional label for debugging / trace display. */
	name?: string;
};

function isPlainObject(value: unknown): value is Record<string, unknown> {
	return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function tryParseGdsModelId(modelId: string): { facade: string; op: string } | undefined {
	const parts = modelId.split('.').map((p) => p.trim()).filter(Boolean);
	if (parts.length !== 3) return undefined;
	if (parts[0] !== 'gds') return undefined;
	return { facade: parts[1]!, op: parts[2]! };
}

function kernelRequestToGdsApplicationCall(request: KernelRunRequest): GdsApplicationCall {
	if (isPlainObject(request.input) && typeof request.input.facade === 'string' && typeof request.input.op === 'string') {
		return GdsApplicationCallSchema.parse(request.input);
	}

	const parsed = tryParseGdsModelId(request.model.id);
	if (!parsed) {
		throw new Error(
			`GdsTsjsonKernelPort only supports model ids shaped like "gds.<facade>.<op>"; got: ${request.model.id}`,
		);
	}

	if (!isPlainObject(request.input)) {
		throw new Error('KernelRunRequest.input must be an object when using gds.<facade>.<op> model ids');
	}

	return GdsApplicationCallSchema.parse({
		...request.input,
		facade: parsed.facade,
		op: parsed.op,
	});
}

/**
 * GdsTsjsonKernelPort
 *
 * Bridges the generic `KernelPort` API onto the Rust TS-JSON NAPI boundary.
 *
 * Architectural note (terminology):
 * - **GDS-L (GDS Link)**: the client→server protocol for calling into GDS.
 *   - In this repo, the "Link" payload is `GdsApplicationCall` (facade/op + inputs).
 *   - The "Link" transport is the `KernelPort` adapter (this class).
 * - **G-DSL (Generic / Global DSL)**: the higher-level, client-facing DSL space
 *   that *produces* GDS-L payloads (e.g. FormDB, S-DSL specializations).
 *
 * Transport details:
 * - request: JSON string
 * - response: JSON string envelope { ok, op, data|error }
 *
 * This port is intentionally implementation-agnostic: you supply the `invoke` function.
 * That `invoke` can be a native NAPI binding, a mock (tests), or a remote adapter.
 */
export class GdsTsjsonKernelPort implements KernelPort {
	readonly name: string;
	private readonly invoke: TsjsonInvoker;

	constructor(invoke: TsjsonInvoker, opts: GdsTsjsonKernelPortOptions = {}) {
		this.invoke = invoke;
		this.name = opts.name ?? 'gds-tsjson';
	}

	async run(request: KernelRunRequest): Promise<KernelRunResult> {
		try {
			const call = kernelRequestToGdsApplicationCall(request);
			const responseJson = await this.invoke(JSON.stringify(call));
			const parsedJson = JSON.parse(responseJson) as unknown;
			const envelope = GdsTsjsonResponseSchema.parse(parsedJson);

			if (envelope.ok) {
				return { ok: true, output: envelope.data };
			}

			return {
				ok: false,
				error: envelope.error,
			};
		} catch (error) {
			return {
				ok: false,
				error: {
					message: error instanceof Error ? error.message : String(error),
				},
			};
		}
	}
}

/**
 * Alias export for architectural naming:
 * `GdsLinkTsjsonKernelPort` == `GdsTsjsonKernelPort`.
 *
 * Prefer this name when you want to emphasize "GDS Link" (client→server protocol)
 * rather than "TSJSON transport".
 */
export { GdsTsjsonKernelPort as GdsLinkTsjsonKernelPort };
