import { describe, it, expect } from "vitest";
import { FormMorph } from "../../../src/relative/form/morph";
import { createMorph, updateMorph } from "../../../src/schema/morph";

describe("FormMorph (Foundation)", () => {
  it("creates FormMorph from schema using create helper", () => {
    const morph = FormMorph.create({
      id: "morph:normalize",
      type: "system.Morph",
      name: "Normalize",
      state: { transform: "normalize", active: true },
    });

    expect(morph.id).toBe("morph:normalize");
    expect(morph.type).toBe("system.Morph");
    expect(morph.name).toBe("Normalize");
    expect(morph.state.transform).toBe("normalize");
    expect(morph.state.active).toBe(true);
    expect(morph.revision).toBe(0);
  });

  it("wraps existing schema document with fromSchema", () => {
    const doc = createMorph({
      id: "morph:validate",
      type: "system.Morph",
      name: "Validate",
      state: { transform: "validate", active: false },
    });

    const morph = FormMorph.fromSchema(doc);

    expect(morph.id).toBe("morph:validate");
    expect(morph.name).toBe("Validate");
    expect(morph.state.active).toBe(false);
  });

  it("mutates core fields via setCore", () => {
    const morph = FormMorph.create({
      id: "morph:test",
      type: "system.Morph",
      name: "Test",
    });

    morph.setCore({ name: "TestUpdated", type: "custom.Morph" });

    expect(morph.name).toBe("TestUpdated");
    expect(morph.type).toBe("custom.Morph");
    expect(morph.revision).toBe(1); // Schema helper increments revision
  });

  it("mutates state via setState and patchState", () => {
    const morph = FormMorph.create({
      id: "morph:state",
      type: "system.Morph",
      state: { transform: "initial", active: true },
    });

    // Full state replacement
    morph.setState({ transform: "replaced", active: false, extra: "data" });
    expect(morph.state.transform).toBe("replaced");
    expect(morph.state.active).toBe(false);
    expect(morph.state.extra).toBe("data");

    // Partial state patch
    morph.patchState({ active: true });
    expect(morph.state.transform).toBe("replaced"); // preserved
    expect(morph.state.active).toBe(true); // updated
  });

  it("handles signature and facets via mutators", () => {
    const morph = FormMorph.create({
      id: "morph:meta",
      type: "system.Morph",
      signature: { input: "string", output: "number" },
      facets: { category: "transform", version: 1 },
    });

    expect(morph.signature?.input).toBe("string");
    expect(morph.facets?.category).toBe("transform");

    // Merge signature
    morph.mergeSignature({ validation: true });
    expect(morph.signature?.input).toBe("string"); // preserved
    expect(morph.signature?.validation).toBe(true); // added

    // Merge facets
    morph.mergeFacets({ version: 2, status: "active" });
    expect(morph.facets?.category).toBe("transform"); // preserved
    expect(morph.facets?.version).toBe(2); // updated
    expect(morph.facets?.status).toBe("active"); // added
  });

  it("serializes to schema via toSchema", () => {
    const morph = FormMorph.create({
      id: "morph:serialize",
      type: "system.Morph",
      name: "Serialize",
      state: { transform: "serialize", active: true },
    });

    const schema = morph.toSchema();

    expect(schema.shape.core.id).toBe("morph:serialize");
    expect(schema.shape.core.type).toBe("system.Morph");
    expect(schema.shape.core.name).toBe("Serialize");
    expect(schema.shape.state.transform).toBe("serialize");
    expect(schema.shape.state.active).toBe(true);
    expect(schema.revision).toBe(0);
  });

  it("describes morph structure", () => {
    const morph = FormMorph.create({
      id: "morph:describe",
      type: "system.Morph",
      name: "Describe",
      signature: { input: "any", output: "string" },
      facets: { category: "utility" },
    });

    const description = morph.describe();

    expect(description.id).toBe("morph:describe");
    expect(description.type).toBe("system.Morph");
    expect(description.name).toBe("Describe");
    expect(description.signatureKeys).toEqual(["input", "output"]);
    expect(description.facetsKeys).toEqual(["category"]);
  });
});
