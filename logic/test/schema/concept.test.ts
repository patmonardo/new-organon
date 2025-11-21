import { describe, it, expect } from "vitest";
import {
  ConceptSchema,
  createConcept,
  updateConcept,
  addDetermination,
  addExemplar,
  advanceWheel,
} from '@schema';
import { EntityRef } from '@schema';

describe("schema/concept — UPS wheel (happy path)", () => {
  it("creates a Concept and advances the wheel deterministically", () => {
    const c0 = createConcept({
      type: "system.Concept",
      name: "C",
      triad: {
        universal: { intent: { law: "X" } },
      },
    });

    // Parse round-trip
    const p0 = ConceptSchema.parse(c0);
    expect(p0.shape.core.type).toBe("system.Concept");
    expect(p0.shape.wheel.stage).toBe("universal");

    // Advance U -> P -> S -> return -> U (cycle +1)
    const c1 = advanceWheel(c0);
    const c2 = advanceWheel(c1);
    const c3 = advanceWheel(c2);
    const c4 = advanceWheel(c3);

    expect(c1.shape.wheel.stage).toBe("particular");
    expect(c2.shape.wheel.stage).toBe("singular");
    expect(c3.shape.wheel.stage).toBe("return");
    expect(c4.shape.wheel.stage).toBe("universal");
    expect(c4.shape.wheel.cycle).toBe((c0.shape.wheel.cycle ?? 0) + 1);
  });

  it("adds determinations and exemplars, normalizing deterministically", () => {
    const c0 = createConcept({ type: "system.Concept", name: "D" });

    const c1 = addDetermination(c0, { key: "b", value: 2 });
    const c2 = addDetermination(c1, { key: "a", value: 1 });
    const c3 = addExemplar(c2, EntityRef.parse({ id: "X", type: "system.Thing" }));
    const c4 = addExemplar(c3, { id: "A", type: "system.Thing" });

    const c5 = updateConcept(c4, {
      triad: { singular: { witness: ["w2", "w1"] } as any },
    });

    // Determinations sorted by key
    expect(c5.shape.triad.particular.determinations.map((d) => d.key)).toEqual(["a", "b"]);
    // Exemplars sorted by id
    expect(c5.shape.triad.singular.exemplars.map((e) => e.id)).toEqual(["A", "X"]);
    // Witness sorted
    expect(c5.shape.triad.singular.witness).toEqual(["w1", "w2"]);

    // Schema validates final doc
    expect(() => ConceptSchema.parse(c5)).not.toThrow();
  });
});
