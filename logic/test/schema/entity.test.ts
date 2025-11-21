import { describe, it, expect } from "vitest";
import {
  EntitySchema,
  createEntity,
  updateEntity,
  createEntityRef,
  formatEntityKey,
  parseEntityKey,
} from '@schema';

describe("schema/entity — Foundation patterns", () => {
  it("creates an Entity with defaults and updates deterministically", () => {
    const e0 = createEntity({ type: "system.Entity", name: "A" });
    const p0 = EntitySchema.parse(e0);
    expect(p0.shape.core.type).toBe("system.Entity");
    expect(p0.shape.core.name).toBe("A");
    expect(typeof p0.shape.state).toBe("object");
    expect(p0.revision).toBe(0);

    const e1 = updateEntity(e0, {
      core: { name: "A1" },
      state: { status: "active", tags: [], meta: {} },
      version: "1.0.0",
      ext: { a: 1 },
    });
    const p1 = EntitySchema.parse(e1);
    expect(p1.shape.core.name).toBe("A1");
    expect(p1.shape.state.status).toBe("active");
    expect(p1.version).toBe("1.0.0");
    expect(p1.ext).toMatchObject({ a: 1 });
    expect(p1.revision).toBe(p0.revision + 1);
  });

  it("creates refs, formats and parses entity keys (round-trip)", () => {
    const e = createEntity({ type: "system.Entity", name: "K" });
    const ref = createEntityRef(e);
    const key = formatEntityKey(ref);
    const parsed = parseEntityKey(key);
    expect(parsed).toEqual(ref);
  });

  it("handles simple dotted types in keys", () => {
    const e = createEntity({ type: "system.Engine", name: "Engine" });
    const ref = createEntityRef(e);
    const key = formatEntityKey(ref); // "system.Engine:<uuid>"
    const parsed = parseEntityKey(key);
    expect(parsed.type).toBe("system.Engine");
    expect(parsed.id).toBe(ref.id);
  });
});
