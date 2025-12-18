import { describe, it, expect } from 'vitest';

import {
  contextFromFactTrace,
  kernelRunToTraceEvents,
  type KernelRunRequest,
  type KernelRunResult,
} from '@organon/gdsl';

describe('kernel-trace', () => {
  it('converts kernel run into trace events', () => {
    const request: KernelRunRequest = {
      model: { id: 'gds.pregel.rank', kind: 'gds', version: '1' },
      input: { graph: 'g://demo', seed: ['n1'] },
      params: { iterations: 10 },
    };

    const result: KernelRunResult = {
      ok: true,
      output: { scores: { n1: 0.5 } },
    };

    const events = kernelRunToTraceEvents(request, result, { runId: 'run-123' });

    expect(events).toHaveLength(2);
    expect(events[0].kind).toBe('kernel.run.request');
    expect(events[1].kind).toBe('kernel.run.result');
    expect(events[0].meta?.factStore?.ids).toEqual(['run-123']);
    expect(events[1].meta?.factStore?.ids).toEqual(['run-123']);
  });

  it('kernel result becomes an inferred fact in context', () => {
    const request: KernelRunRequest = {
      model: { id: 'kernel.demo' },
      input: { x: 1 },
    };

    const result: KernelRunResult = {
      ok: true,
      output: { y: 2 },
    };

    const events = kernelRunToTraceEvents(request, result, { runId: 'r1' });
    const ctx = contextFromFactTrace(events, { schema: { id: 'trace:kernel' } });

    const requestFact = ctx.facts.find((f) => f.type === 'kernel.run.request');
    const resultFact = ctx.facts.find((f) => f.type === 'kernel.run.result');

    expect(requestFact?.label).toBe('kernel.run');
    expect(requestFact?.provenance).toBe('observed');

    expect(resultFact?.label).toBe('kernel.run');
    expect(resultFact?.provenance).toBe('inferred');
  });
});
