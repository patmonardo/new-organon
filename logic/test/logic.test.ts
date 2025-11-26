import { describe, it, expect } from "vitest";
import { logic } from "../src/logic";

describe("@organon/logic", () => {
  it('exports "logic"', () => {
    expect(logic).toBe("logic");
  });
});

// GDSL and Reality are now target IRs, not active dependencies
// The dialectic schemas live here in @organon/logic
// Tests for Form Engines with dialectic.evaluate are in separate test files
