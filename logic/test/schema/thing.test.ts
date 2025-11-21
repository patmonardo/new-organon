import { describe, it, expect } from "vitest";
import {
  ThingSchema,
  createThing,
  updateThing,
  createThingRef,
  ThingRef,
  isThingRef,
} from '@schema';

describe("schema/thing — happy path", () => {
  it("creates a Thing with defaults", () => {
    const t = createThing({ type: "system.Thing", name: "X" });
    const p = ThingSchema.parse(t);

    expect(p.shape.core.type).toBe("system.Thing");
    expect(p.shape.core.name).toBe("X");
    expect(p.shape.core.id.startsWith("thing:")).toBe(true);
    expect(typeof p.shape.state).toBe("object");
    expect(p.shape.essence).toBeUndefined();
    expect(p.shape.qualities).toEqual({});
    expect(p.revision).toBe(0);
  });

  it("updates core/state/essence/qualities and bumps revision", () => {
    const t0 = createThing({ type: "system.Thing", name: "A", category: "demo" });

    const t1 = updateThing(t0, {
      core: { name: "A1", category: "test" },
      state: { status: "active", tags: [], meta: {} },
      essence: { law: "being" },
      qualities: { a: 1 },
    });

    const p1 = ThingSchema.parse(t1);
    expect(p1.shape.core.name).toBe("A1");
    expect(p1.shape.core.category).toBe("test");
    expect(p1.shape.state.status).toBe("active");
    expect(p1.shape.essence).toEqual({ law: "being" });
    expect(p1.shape.qualities).toEqual({ a: 1 });
    expect(p1.revision).toBe(t0.revision + 1);

    // Qualities are replaced (not merged)
    const t2 = updateThing(t1, { qualities: { b: 2 } });
    expect(t2.shape.qualities).toEqual({ b: 2 });

    // Omitting essence (undefined) preserves current essence
    const t3 = updateThing(t2, { essence: undefined });
    expect(t3.shape.essence).toEqual({ law: "being" });
  });

  it("creates a ThingRef and validates it", () => {
    const t = createThing({ type: "system.Thing", name: "Ref" });
    const ref = createThingRef(t);

    expect(() => ThingRef.parse(ref)).not.toThrow();
    expect(isThingRef(ref)).toBe(true);
    expect(isThingRef({ id: "X", type: "nope" } as any)).toBe(false);
  });
});

