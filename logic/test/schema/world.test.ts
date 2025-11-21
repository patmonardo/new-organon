import { describe, it, expect } from "vitest";
import {
  WorldSchema,
  createWorld,
  updateWorld,
} from '@schema';

function thing(id: string) {
  return { id, type: "system.Thing" as const };
}

function edge(kind: string, s: string, t: string) {
  return {
    kind,
    direction: "directed" as const,
    source: thing(s),
    target: thing(t),
  };
}

describe("schema/world — happy path", () => {
  it("creates a World with defaults (no things/relations, optional horizon)", () => {
    const w = createWorld({
      type: "system.World",
      name: "W",
      // state omitted -> defaulted
    });
    const parsed = WorldSchema.parse(w);

    expect(parsed.shape.core.type).toBe("system.World");
    expect(parsed.shape.core.name).toBe("W");
    expect(parsed.shape.core.id.startsWith("world:")).toBe(true);
    expect(parsed.shape.things).toEqual([]);
    expect(parsed.shape.relations).toEqual([]);
    expect(parsed.shape.horizon).toBeUndefined();
    expect(parsed.revision).toBe(0);
  });

  it("adds things/relations/horizon and bumps revision deterministically", () => {
    const w0 = createWorld({ type: "system.World" });

    const w1 = updateWorld(w0, {
      things: [thing("A"), thing("B")],
      relations: [edge("related_to", "A", "B")],
      state: { status: "active", tags: [], meta: {} },
      horizon: { scope: "demo" },
    });

    const p1 = WorldSchema.parse(w1);
    expect(p1.shape.state.status).toBe("active");
    expect(p1.shape.things.map((t) => t.id)).toEqual(["A", "B"]);
    expect(p1.shape.relations[0].kind).toBe("related_to");
    expect(p1.shape.relations[0].direction).toBe("directed");
    expect(p1.shape.horizon).toEqual({ scope: "demo" });
    expect(p1.revision).toBe(w0.revision + 1);
  });

  it("supports chained updates with revision increments", () => {
    const w0 = createWorld({ type: "system.World" });
    const w1 = updateWorld(w0, { things: [thing("X")] });
    const w2 = updateWorld(w1, {
      relations: [edge("contains", "X", "X")],
    });

    expect(w1.revision).toBe(w0.revision + 1);
    expect(w2.revision).toBe(w1.revision + 1);

    const p2 = WorldSchema.parse(w2);
    expect(p2.shape.things[0].id).toBe("X");
    expect(p2.shape.relations[0].kind).toBe("contains");
  });
});

