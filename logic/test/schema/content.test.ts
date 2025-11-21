import { describe, it, expect } from "vitest";
import { ContentSchema, createContent, updateContent } from '@schema';
import { EntityRef } from '@schema';

describe("schema/content — happy path", () => {
  it("creates subtle and gross content with deterministic carrier refs", () => {
    const carrier = EntityRef.parse({ id: "E1", type: "system.Thing" });

    const subtle = createContent({
      type: "system.Content",
      name: "worldly",
      kind: "subtle",
      of: carrier,
    });
    const gross = createContent({
      type: "system.Content",
      name: "thingy",
      kind: "gross",
      of: carrier,
    });

    // Parse round-trip
    expect(ContentSchema.parse(subtle).shape.kind).toBe("subtle");
    expect(ContentSchema.parse(gross).shape.kind).toBe("gross");

    // Carrier preserved
    expect(subtle.shape.of.id).toBe("E1");
    expect(gross.shape.of.id).toBe("E1");
  });

  it("updates kind and facets via updateContent", () => {
    const c0 = createContent({
      type: "system.Content",
      kind: "gross",
      of: { id: "X", type: "system.Thing" },
    });

    const c1 = updateContent(c0, { kind: "subtle" });
    expect(c1.shape.kind).toBe("subtle");

    const c2 = updateContent(c1, { facets: { a: 1, b: "ok" } });
    expect(c2.shape.facets).toEqual({ a: 1, b: "ok" });

    // Schema validates final doc
    expect(() => ContentSchema.parse(c2)).not.toThrow();
  });
});
