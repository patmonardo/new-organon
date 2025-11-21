import { describe, it, expect } from 'vitest';
import { gdsl } from '@organon/gdsl';
import { model } from '@organon/model';
import { logic } from '@organon/logic';
import { reality } from '@organon/reality';

describe('@organon/gdsl resolution', () => {
  it('imports from "@organon/gdsl" and exports "task"', () => {
    expect(gdsl).toBe('gdsl');
  });
});

describe('@organon/model resolution', () => {
  it('imports from "@organon/model" and exports "task"', () => {
    expect(model).toBe('model');
  });
});

describe('@organon/logic resolution', () => {
  it('imports from "@organon/logic" and exports "task"', () => {
    expect(logic).toBe('logic');
  });
});

describe('@organon/reality resolution', () => {
  it('imports from "@organon/reality" and exports "task"', () => {
    expect(reality).toBe('reality');
  });
});
