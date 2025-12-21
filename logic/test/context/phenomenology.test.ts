import { describe, it, expect } from 'vitest';

import { FormContext } from '../../src/relative/form/context/context-form';
import {
  recordContextContradiction,
  setContextFoundation,
  readContextPhenomenology,
  deriveJudgmentFromFoundation,
} from '../../src/relative/form/context/phenomenology';

describe('context phenomenology facet', () => {
  it('records contradictions and sets an infinite foundation', () => {
    const ctx = FormContext.create({ type: 'prompt', name: 'Breakfast Context' });

    recordContextContradiction(ctx, { id: 'c1', claim: 'A and not-A both asserted' });
    setContextFoundation(ctx, {
      moment: 'infinite',
      thesis: 'Contradiction-free foundation is presented',
      resolves: ['c1'],
    });

    const phen = readContextPhenomenology(ctx.toSchema());
    expect(phen?.contradictions?.[0]?.id).toBe('c1');
    expect(phen?.foundation?.moment).toBe('infinite');
    expect(phen?.foundation?.contradictionFree).toBe(true);
  });

  it('derives judgment from foundation', () => {
    const judgment = deriveJudgmentFromFoundation({
      moment: 'positive',
      thesis: 'Determination holds',
      resolves: ['cX'],
    });
    expect(judgment.thesis).toBe('Determination holds');
    expect(judgment.grounds?.some((g) => g.includes('foundation.moment=positive'))).toBe(true);
  });
});
