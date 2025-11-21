import { describe, it, expect } from "vitest";
import { gdsl } from '@organon/gdsl';
import { logic } from '@organon/logic';
import { reality } from '@organon/reality';

describe("@organon/gdsl resolution", () => {
  it('imports from "@organon/gdsl" and exports "model"', () => {
    expect(gdsl).toBe("gdsl");
  });
});

describe("@organon/logic resolution", () => {
  it('imports from "@organon/logic" and exports "model"', () => {
    expect(logic).toBe("logic");
  });
});

describe('@organon/reality resolution', () => {
  it('imports from "@organon/reality" and exports "model"', () => {
    expect(reality).toBe('reality');
  });
});
