import { describe, expect, it } from 'vitest';

import { GdsTsjsonKernelPort, type KernelRunRequest } from '../src/index';

describe('GdsTsjsonKernelPort', () => {
	it('maps gds.<facade>.<op> to TSJSON invoke and returns data as output', async () => {
		const invocations: unknown[] = [];
		const invoke = async (requestJson: string) => {
			invocations.push(JSON.parse(requestJson));
			return JSON.stringify({ ok: true, op: 'list_graphs', data: { entries: [{ name: 'g1' }] } });
		};

		const port = new GdsTsjsonKernelPort(invoke);
		const request: KernelRunRequest = {
			model: { id: 'gds.graph_store_catalog.list_graphs' },
			input: { user: { username: 'alice', isAdmin: true }, databaseId: 'db1' },
		};

		const result = await port.run(request);
		expect(result).toEqual({ ok: true, output: { entries: [{ name: 'g1' }] } });
		expect(invocations).toHaveLength(1);

		const call = invocations[0] as any;
		expect(call.facade).toBe('graph_store_catalog');
		expect(call.op).toBe('list_graphs');
		expect(call.user.username).toBe('alice');
	});

	it('propagates TSJSON ok=false envelopes as KernelRunResult ok=false', async () => {
		const invoke = (requestJson: string) => {
			// sanity: should be valid JSON
			JSON.parse(requestJson);
			return JSON.stringify({ ok: false, op: 'graph_memory_usage', error: { code: 'NOT_FOUND', message: 'Graph not found' } });
		};

		const port = new GdsTsjsonKernelPort(invoke);
		const request: KernelRunRequest = {
			model: { id: 'gds.graph_store_catalog.graph_memory_usage' },
			input: { user: { username: 'alice', isAdmin: true }, databaseId: 'db1', graphName: 'missing' },
		};

		const result = await port.run(request);
		expect(result.ok).toBe(false);
		expect(result.error).toEqual({ code: 'NOT_FOUND', message: 'Graph not found' });
	});
});
