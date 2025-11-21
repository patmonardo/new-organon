import { describe, it, expect } from "vitest";
import { FormShape } from "../../../src/relative/form/shape";
import { ShapeSchema } from "../../../src/schema/shape";

describe("form/FormShape — happy path", () => {
  it("creates, mutates, and serializes a principle Shape", () => {
    const s = FormShape.create({ type: "system.Shape", name: "S" })
      .setName("S1")
      .setState({ status: "active", tags: [], meta: {} })
      .mergeFacets({ a: 1 })
      .patchSignature({ k: "v" });

    const doc = s.toSchema();
    const parsed = ShapeSchema.parse(doc);
    expect(parsed.shape.core.type).toBe("system.Shape");
    expect(parsed.shape.core.name).toBe("S1");
    expect(parsed.shape.state.status).toBe("active");
    expect(parsed.shape.facets).toEqual({ a: 1 });
    expect(parsed.shape.signature && (parsed.shape.signature as any).k).toBe("v");
  });

  it("preserves id/type, supports chained updates, and bumps revision deterministically", () => {
    const s = FormShape.create({ type: "system.Shape", name: "A" });
    const d0 = s.toSchema();
    const id = d0.shape.core.id;
    const type = d0.shape.core.type;

    s.setName("A1").patchSignature({ x: 1 }).mergeFacets({ k: "v" });
    const d1 = s.toSchema();

    expect(d1.shape.core.id).toBe(id);
    expect(d1.shape.core.type).toBe(type);
    expect(d1.shape.core.name).toBe("A1");
    expect(d1.revision).toBe((d0.revision ?? 0) + 3);
  });

  it("signature: patch merges, set replaces, set(undefined) clears", () => {
    const s = FormShape.create({ type: "system.Shape", name: "Sig" });

    // Start with a merge
    s.patchSignature({ a: 1, b: 2 });
    const d1 = s.toSchema();
    expect(d1.shape.signature).toEqual({ a: 1, b: 2 });

    // Replace entirely
    s.setSignature({ c: 3 });
    const d2 = s.toSchema();
    expect(d2.shape.signature).toEqual({ c: 3 });

    // Clear explicitly
    s.setSignature(undefined);
    const d3 = s.toSchema();
    expect(d3.shape.signature).toBeUndefined();
  });

  it("facets: merge adds/overwrites keys; set replaces", () => {
    const s = FormShape.create({ type: "system.Shape", name: "F" });
    s.mergeFacets({ a: 1 }).mergeFacets({ b: 2, a: 9 }); // overwrite a
    const d1 = s.toSchema();
    expect(d1.shape.facets).toEqual({ a: 9, b: 2 });

    s.setFacets({ c: 3 });
    const d2 = s.toSchema();
    expect(d2.shape.facets).toEqual({ c: 3 });
  });

  it("state: set replaces; patch merges while preserving other fields", () => {
    const s = FormShape.create({ type: "system.Shape", name: "Stateful" });

    s.setState({ status: "active", tags: [], meta: { note: "ok" } });
    let d = s.toSchema();
    expect(d.shape.state.status).toBe("active");
    expect(d.shape.state.meta).toEqual({ note: "ok" });

    s.patchState({ tags: ["x"] });
    d = s.toSchema();
    expect(d.shape.state.status).toBe("active");
    expect(d.shape.state.tags).toEqual(["x"]);
    expect(d.shape.state.meta).toEqual({ note: "ok" });
  });

  it("round-trips via from() and includes id in toJSON()", () => {
    const s0 = FormShape.create({ type: "system.Shape", name: "Round" })
      .patchSignature({ t: true })
      .mergeFacets({ m: 1 });

    const doc0 = s0.toSchema();
    const s1 = FormShape.from(doc0);
    const doc1 = s1.toSchema();

    expect(doc1).toEqual(ShapeSchema.parse(doc0));
    const j = s1.toJSON();
    expect(j.id).toBe(doc1.shape.core.id);
    expect(j.shape.core.id).toBe(j.id);
  });
});
