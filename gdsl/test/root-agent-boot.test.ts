import { describe, expect, it } from 'vitest';

import { parseRootAgentBootEnvelope } from '../src/root-agent-boot';

describe('RootAgent boot envelope (GDSL)', () => {
  it('parses minimal boot envelope (context + intent)', () => {
    const boot = parseRootAgentBootEnvelope({
      context: {
        id: 'ctx-boot',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
      },
      intent: {
        kind: 'taw.intent',
        payload: { goal: { id: 'g1', type: 'boot', description: 'Boot RootAgent' } },
        source: 'test',
      },
    });

    expect(boot.context.id).toBe('ctx-boot');
    expect(boot.intent.kind).toBe('taw.intent');
    expect(boot.intent.payload.goal.id).toBe('g1');
  });

  it('accepts optional syscalls table', () => {
    const boot = parseRootAgentBootEnvelope({
      context: {
        id: 'ctx-boot',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
      },
      intent: {
        kind: 'taw.intent',
        payload: { goal: { id: 'g1', type: 'boot', description: 'Boot RootAgent' } },
      },
      syscalls: {
        syscalls: [
          { id: 'k1', kind: 'kernel.run', title: 'Run rank', model: { id: 'gds.pregel.rank' } },
          { id: 't1', kind: 'tool', title: 'Do thing', name: 'tool.doThing', inputSchema: { type: 'object' } },
        ],
      },
    });

    expect(boot.syscalls?.syscalls).toHaveLength(2);
  });
});
