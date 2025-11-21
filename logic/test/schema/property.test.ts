import { describe, it, expect } from "vitest";
import {
  PropertySchema,
  createProperty,
  updateProperty,
} from '@schema';

describe("schema/property — happy path (skeletal)", () => {
  it("creates a Property with defaults and exposes core fields", () => {
    const p = createProperty({
      type: "system.Property",
      name: "color",
    });

    const parsed = PropertySchema.parse(p);
    expect(parsed.shape.core.type).toBe("system.Property");
    expect(parsed.shape.core.name).toBe("color");
    expect(parsed.revision ?? 0).toBe(0);
    expect(parsed.shape.signature).toBeUndefined();
    expect(parsed.shape.facets).toEqual({});
  });

  it("updates core/state/signature/facets and bumps revision", () => {
    const p0 = createProperty({
      type: "system.Property",
      name: "size",
    });

    const p1 = updateProperty(p0, {
      core: { name: "Size" },
      state: { any: 42 } as any,
      signature: { s: 1 },
      facets: { ns: { a: 1 } },
      version: "1.0.0",
      ext: { a: 1 },
    });

    expect(p1.shape.core.name).toBe("Size");
    expect((p1.shape.state as any).any).toBe(42);
    expect((p1.shape.signature as any).s).toBe(1);
    expect((p1.shape.facets as any).ns.a).toBe(1);
    expect(p1.revision).toBe((p0.revision ?? 0) + 1);
    expect(p1.version).toBe("1.0.0");
    expect(p1.ext).toMatchObject({ a: 1 });

    // clear signature by setting null, preserve facets if undefined
    const p2 = updateProperty(p1, { signature: null });
    expect(p2.shape.signature).toBeUndefined();
  });

  it("validates final doc after updates", () => {
    const p0 = createProperty({ type: "system.Property", name: "valid" });
    const p1 = updateProperty(p0, { state: { foo: "bar" } as any, facets: { k: "v" } });
    expect(() => PropertySchema.parse(p1)).not.toThrow();
  });
});

