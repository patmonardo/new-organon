import { describe, expect, it } from 'vitest';

import { parseRootAgentAbsorbRequest, parseRootAgentAbsorbResult } from '../src/root-agent-absorb';

describe('RootAgent absorption surface (GDSL)', () => {
  it('parses absorb request/result artifacts', () => {
    const req = parseRootAgentAbsorbRequest({
      previous: {
        id: 'ctx-0',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
      },
      traceDelta: [{ kind: 'kernel.run.result', payload: { ok: true } }],
      strategy: 'append',
      maxFacts: 10,
    });

    expect(req.previous.id).toBe('ctx-0');
    expect(req.traceDelta).toHaveLength(1);

    const res = parseRootAgentAbsorbResult({
      next: {
        id: 'ctx-1',
        timestamp: new Date().toISOString(),
        facts: [],
        schema: { id: 's', fieldCount: 0, requiredFields: [], optionalFields: [] },
      },
      absorbedCount: 1,
    });

    expect(res.absorbedCount).toBe(1);
    expect(res.next.id).toBe('ctx-1');
  });
});
