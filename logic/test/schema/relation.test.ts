import { describe, it, expect } from "vitest";
import {
  RelationSchema,
  createRelation,
  updateRelation,
  createBidirectionalRelation,
  invertRelation,
} from '../../src/schema';

describe("schema/relation — happy path", () => {
  it("creates a directed relation with defaults", () => {
    const rel = createRelation({
      type: "system.Relation",
      kind: "related_to",
      source: { id: "E1", type: "system.Thing" },
      target: { id: "E2", type: "system.Thing" },
      state: { status: "active", tags: [], meta: {} }, // BaseState fields
    });

    const parsed = RelationSchema.parse(rel);
    expect(parsed.shape.core.type).toBe("system.Relation");
    expect(parsed.shape.core.kind).toBe("related_to");
    expect(parsed.shape.source.id).toBe("E1");
    expect(parsed.shape.target.id).toBe("E2");
    expect(parsed.shape.direction).toBe("directed");
    expect(parsed.shape.state.status).toBe("active");
    expect(parsed.shape.state.strength).toBe(1); // default
    expect(parsed.revision).toBe(0);
    expect(parsed.ext).toEqual({});
  });

  it("updates core/state/direction and bumps revision deterministically", () => {
    const r0 = createRelation({
      type: "system.Relation",
      kind: "contains",
      source: { id: "A", type: "system.Thing" },
      target: { id: "B", type: "system.Thing" },
      state: { status: "active", tags: [], meta: {} },
    });

    const r1 = updateRelation(r0, {
      core: { name: "A-contains-B" },
      state: { strength: 0.4 },
      direction: "bidirectional",
      version: "1.0.0",
      ext: { note: "ok" },
    });

    const p = RelationSchema.parse(r1);
    expect(p.shape.core.id).toBe(r0.shape.core.id); // id preserved
    expect(p.shape.core.name).toBe("A-contains-B");
    expect(p.shape.state.strength).toBe(0.4);
    expect(p.shape.direction).toBe("bidirectional");
    expect(p.version).toBe("1.0.0");
    expect(p.ext).toMatchObject({ note: "ok" });
    expect(p.revision).toBe(r0.revision + 1);
  });

  it("creates bidirectional and inverts directed relations correctly", () => {
    const bi = createBidirectionalRelation({
      type: "system.Relation",
      kind: "adjacent_to",
      source: { id: "X", type: "system.Thing" },
      target: { id: "Y", type: "system.Thing" },
      state: { status: "active", tags: [], meta: {} },
    });
    expect(bi.shape.direction).toBe("bidirectional");
    // invert of bidirectional is identity
    expect(invertRelation(bi)).toEqual(bi);

    const dir = createRelation({
      type: "system.Relation",
      kind: "causes",
      source: { id: "S", type: "system.Thing" },
      target: { id: "T", type: "system.Thing" },
      state: { status: "active", tags: [], meta: {} },
    });
    const inv = invertRelation(dir);
    expect(inv.shape.direction).toBe("directed");
    expect(inv.shape.source.id).toBe("T"); // swapped
    expect(inv.shape.target.id).toBe("S");
    // new id generated (since invert creates a new relation)
    expect(inv.shape.core.id).not.toBe(dir.shape.core.id);
  });
});

