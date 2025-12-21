import type { KernelPort, KernelRunRequest, KernelRunResult } from '@organon/gdsl';

/**
 * Logic API (Singular surface for Agents)
 *
 * Intent: expose one stable TypeScript entrypoint that an Agent can call.
 *
 * - Knowing (Absolute Form / Science) lives in the Rust `gds` kernel.
 * - GDSL defines the transportable boundary vocabulary.
 * - Logic adds conceiving/projection helpers that *construct* calls and
 *   interpret results, without pretending to execute kernel code directly.
 */

export type LogicApi = {
	readonly kernel: KernelPort;
	/** Kernel Absolute Form (FormProcessor) calls. */
	form: {
		evaluate(call: FormEvalCall): Promise<KernelRunResult>;
	};
};

/**
 * Minimal call shape for the kernel FormProcessor.
 *
 * This intentionally stays structural so it can be constructed by an Agent
 * (or by higher-level Logic helpers) without assuming a specific transport.
 */
export type FormEvalCall = {
	facade: 'form_eval';
	op: 'evaluate';
	user: { username: string; isAdmin?: boolean };
	databaseId: string;
	graphName: string;
	outputGraphName?: string;
	program: { morph: { patterns: string[] } } & Record<string, unknown>;
	artifacts?: Record<string, unknown>;
} & Record<string, unknown>;

function toKernelRunRequest(call: { facade: string; op: string } & Record<string, unknown>): KernelRunRequest {
	// Canonical kernel model id convention used by GDSL KernelPort adapters.
	const modelId = `gds.${call.facade}.${call.op}`;
	const req: KernelRunRequest = {
		model: { id: modelId },
		input: call,
	};
	return req;
}

export function createLogicApi(kernel: KernelPort): LogicApi {
	return {
		kernel,
		form: {
			async evaluate(call: FormEvalCall): Promise<KernelRunResult> {
				const request: KernelRunRequest = toKernelRunRequest(call as any);
				return kernel.run(request);
			},
		},
	};
}
