import { describe, it, expect } from "vitest";
import { gdsl } from '@organon/gdsl';
import { reality } from '@organon/reality';

describe("@organon/gdsl resolution", () => {
  it('imports from "@organon/gdsl and exports "logic"', () => {
    expect(gdsl).toBe("gdsl");
  });
});

describe('@organon/reality resolution', () => {
  it('imports from "@organon/reality and exports "logic"', () => {
    expect(reality).toBe('reality');
  });
});
