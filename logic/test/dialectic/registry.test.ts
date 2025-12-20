import { describe, it, expect } from 'vitest';

import {
  dialecticIRRegistryEntries,
  dialecticIRRegistry,
  getDialecticIRMeta,
  getDialecticIRKeysById,
} from '../../src/relative/form/dialectic/index.js';

describe('dialectic IR registry', () => {
  it('has unique registry keys', () => {
    const keys = dialecticIRRegistryEntries.map(e => e.key);
    expect(new Set(keys).size).toBe(keys.length);
    expect(Object.keys(dialecticIRRegistry).length).toBe(keys.length);
  });

  it('indexes known IR ids', () => {
    const beingKeys = getDialecticIRKeysById('being-ir');
    expect(beingKeys.length).toBeGreaterThan(0);

    const beingMeta = getDialecticIRMeta(beingKeys[0]!);
    expect(beingMeta?.id).toBe('being-ir');

    const existenceKeys = getDialecticIRKeysById('existence-ir');
    expect(existenceKeys.length).toBeGreaterThanOrEqual(2);
  });
});
