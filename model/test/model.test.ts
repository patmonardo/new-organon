import { describe, it, expect } from "vitest";
import { model } from '../src/model';

describe("@organon/model", () => {
  it('exports model constant', () => {
    expect(model).toBe("model");
  });
});
