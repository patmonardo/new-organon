import { describe, it, expect } from "vitest";
import { SyllogismSchema, createSyllogism } from '@schema';

describe("schema/syllogism — happy path", () => {
  it("creates a valid AAA figure-1 syllogism", () => {
    const syl = createSyllogism({
      type: "system.Syllogism",
      name: "Barbara",
      figure: "1",
      mood: "AAA",
      terms: {
        S: { id: "S", type: "system.Thing" },
        P: { id: "P", type: "system.Thing" },
        M: { id: "M", type: "system.Thing" },
      },
    });
    const parsed = SyllogismSchema.parse(syl);
    expect(parsed.shape.figure).toBe("1");
    expect(parsed.shape.mood).toBe("AAA");
    expect(parsed.shape.terms.S.id).toBe("S");
    expect(parsed.shape.terms.P.id).toBe("P");
    expect(parsed.shape.terms.M.id).toBe("M");
  });
});
