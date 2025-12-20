import { describe, it, expect } from 'vitest';

import {
  appearanceSemanticRoleForMoment,
  withAppearanceMeta,
} from '../../src/relative/form/dialectic/appearance-meta.js';

describe('appearance meta semantics', () => {
  it('maps moments to canonical semantic roles', () => {
    expect(appearanceSemanticRoleForMoment('entity')).toBe('thing');
    expect(appearanceSemanticRoleForMoment('property')).toBe('property');
    expect(appearanceSemanticRoleForMoment('aspect')).toBe('relation-as-aspect');
  });

  it('tags container/moment/semanticRole', () => {
    const meta = withAppearanceMeta({ correlationId: 'x' }, { moment: 'aspect', pass: 'appearance' });
    expect(meta.container).toBe('appearance');
    expect(meta.mathematical).toBe(true);
    expect(meta.presupposes).toEqual(['space', 'time']);
    expect(meta.moment).toBe('aspect');
    expect(meta.semanticRole).toBe('relation-as-aspect');
    expect(meta.correlationId).toBe('x');
    expect(meta.pass).toBe('appearance');
  });
});
