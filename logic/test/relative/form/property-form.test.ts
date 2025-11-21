import { describe, it, expect } from "vitest";
import { FormProperty } from "../../../src/relative/form/property";

describe("FormProperty (skeletal)", () => {
  it("creates with defaults and exposes core fields", () => {
    const p = FormProperty.create({
      type: "system.Property",
      name: "Title",
    });
    expect(p.id).toBeTruthy();
    expect(p.type).toBe("system.Property");
    expect(p.name).toBe("Title");
  });

  it("updates core and state via schema-safe mutators and bumps revision", () => {
    const p = FormProperty.create({ type: "system.Property", name: "P" });
    const r0 = p.toSchema().revision ?? 0;

    p.setCore({ name: "P2", type: "system.Property.Updated" });
    expect(p.name).toBe("P2");
    expect(p.type).toBe("system.Property.Updated");

    p.setState({ any: "value" } as any);
    expect((p.state as any).any).toBe("value");

    p.patchState({ more: true } as any);
    expect((p.state as any).more).toBe(true);

    const r1 = p.toSchema().revision ?? 0;
    expect(r1).toBeGreaterThan(r0);
  });

  it("supports signature and facets extension helpers", () => {
    const p = FormProperty.create({ type: "system.Property", name: "SigFacet" });

    // signature: merge and clear semantics
    p.setSignature({ s: 1 }).mergeSignature({ t: 2 });
    let sig = p.toSchema().shape.signature as Record<string, any>;
    expect(sig.s).toBe(1);
    expect(sig.t).toBe(2);

    p.setSignature(null); // clear
    expect(p.toSchema().shape.signature).toBeUndefined();

    // facets: merge map and namespaced merge
    p.setFacets({ ns: { a: 1 } });
    p.mergeFacets({ other: { b: 2 } });
    p.mergeFacet("ns", { c: 3 });

    const facets = p.toSchema().shape.facets as Record<string, any>;
    expect(facets.other.b).toBe(2);
    expect(facets.ns.a).toBe(1);
    expect(facets.ns.c).toBe(3);
  });

  it("toJSON returns the underlying schema doc", () => {
    const p = FormProperty.create({ type: "system.Property", name: "P" });
    const json = p.toJSON();
    expect(json.shape.core.id).toBe(p.id);
    expect(json.shape.core.type).toBe(p.type);
  });
});
