import { describe, it, expect } from "vitest";
import {
  ContextSchema,
  createContext,
  updateContext,
  addEntitiesToContext,
  addRelationsToContext,
} from '@schema';

describe("schema/context — happy path", () => {
  it("creates a Context with defaults (entities/relations empty, state defaulted)", () => {
    const ctx = createContext({ type: "system.Context", name: "Scope" });
    const parsed = ContextSchema.parse(ctx);

    expect(parsed.shape.core.type).toBe("system.Context");
    expect(typeof parsed.shape.core.id).toBe("string");
    expect(parsed.shape.entities).toEqual([]);
    expect(parsed.shape.relations).toEqual([]);
    expect(typeof parsed.shape.state).toBe("object");
    expect(parsed.revision).toBe(0);
  });

  it("adds entities/relations without duplicates", () => {
    const ctx0 = createContext({
      type: "system.Context",
      entities: [{ id: "A", type: "system.Thing" }],
      relations: ["rel:1"],
    });

    const ctx1 = addEntitiesToContext(ctx0, [
      { id: "A", type: "system.Thing" }, // duplicate
      { id: "B", type: "system.Thing" },
    ]);
    const ctx2 = addRelationsToContext(ctx1, ["rel:1", "rel:2"]); // one duplicate

    expect(ctx1.shape.entities.map((e) => e.id)).toEqual(["A", "B"]);
    expect(ctx2.shape.relations).toEqual(["rel:1", "rel:2"]);
  });

  it("patches core/state/entities and bumps revision deterministically", () => {
    const ctx0 = createContext({ type: "system.Context", name: "X" });

    const ctx1 = updateContext(ctx0, {
      core: { name: "Y" },
      state: { status: "active", tags: [], meta: {} },
      entities: [
        { id: "E1", type: "system.Thing" },
        { id: "E2", type: "system.Thing" },
      ],
      relations: ["r1", "r2"],
      version: "1.0.0",
      ext: { a: 1 },
    });

    const parsed = ContextSchema.parse(ctx1);
    expect(parsed.shape.core.name).toBe("Y");
    expect(parsed.shape.state.status).toBe("active");
    expect(parsed.shape.entities.map((e) => e.id)).toEqual(["E1", "E2"]);
    expect(parsed.shape.relations).toEqual(["r1", "r2"]);
    expect(parsed.revision).toBe(ctx0.revision + 1);
    expect(parsed.version).toBe("1.0.0");
    expect(parsed.ext).toMatchObject({ a: 1 });
  });
});

