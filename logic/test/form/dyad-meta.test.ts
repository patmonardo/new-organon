import { describe, it, expect } from 'vitest';

import {
  withDyadMeta,
  roleForSide,
} from '../../src/relative/form/dialectic/dyad-meta.js';

describe('reflection/appearance dyad semantics', () => {
  it('maps side → role (reflecting/conceiving)', () => {
    expect(roleForSide('reflection')).toBe('reflecting');
    expect(roleForSide('appearance')).toBe('conceiving');
  });

  it('always teleologically ends in knowing', () => {
    const r = withDyadMeta(undefined, { side: 'reflection' });
    const a = withDyadMeta({ correlationId: 'c1' }, { side: 'appearance' });

    expect(r.dyad).toBe('reflection-appearance');
    expect(r.dyadTelos).toBe('knowing');

    expect(a.dyadSide).toBe('appearance');
    expect(a.dyadRole).toBe('conceiving');
    expect(a.dyadTelos).toBe('knowing');
    expect(a.correlationId).toBe('c1');
  });
});
