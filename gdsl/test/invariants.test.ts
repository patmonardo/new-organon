import { describe, expect, it } from 'vitest';

import {
  parseEventMeta,
  parseTraceEvent,
  parseKernelRunResultStrict,
} from '../src/invariants';

describe('GDSL invariants', () => {
  it('accepts EventMeta with unknown keys but validates known structure', () => {
    const meta = parseEventMeta({
      custom: { a: 1 },
      factStore: { op: 'assert', ids: ['e1'] },
      dialectic: { note: 'ok' },
    });

    expect(meta.custom).toEqual({ a: 1 });
    expect(meta.factStore?.op).toBe('assert');
    expect(meta.factStore?.ids).toEqual(['e1']);
  });

  it('rejects invalid FactStore op', () => {
    expect(() =>
      parseTraceEvent({
        kind: 'x',
        meta: { factStore: { op: 'write' } },
      }),
    ).toThrow(/op/i);
  });

  it('enforces strict kernel run result invariant', () => {
    expect(() => parseKernelRunResultStrict({ ok: true, error: { message: 'nope' } })).toThrow(/ok=true/i);
    expect(() => parseKernelRunResultStrict({ ok: false })).toThrow(/ok=false/i);

    expect(parseKernelRunResultStrict({ ok: true, output: { a: 1 } }).ok).toBe(true);
    expect(parseKernelRunResultStrict({ ok: false, error: { message: 'bad' } }).ok).toBe(false);
  });
});
