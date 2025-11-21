import { describe, it, expect } from "vitest";
import { z } from "zod";
import {
  BaseCore,
  BaseSchema,
  BaseState,
  Type,
  Label,
} from "../../src/schema";

describe("schema/base — primitives", () => {
  it("parses Type and Label", () => {
    const t = Type.parse("system.Shape");
    const n = Label.parse("World");
    expect(t).toBe("system.Shape");
    expect(n).toBe("World");
  });

  it("parses BaseCore with id", () => {
    // Provide type to satisfy schemas where BaseCore includes it;
    // if BaseCore strips unknown keys, this still parses.
    const core = BaseCore.parse({ id: "shape:1", type: "system.Base" });
    expect(core.id).toBe("shape:1");
  });

  it("parses BaseState with status/tags/meta", () => {
    const s = BaseState.parse({
      status: "active",
      tags: ["a", "b"],
      meta: { note: "hello" },
    });
    expect(s.status).toBe("active");
    expect(s.tags).toEqual(["a", "b"]);
    expect(s.meta.note).toBe("hello");
  });
});

describe("schema/base — BaseSchema extension", () => {
  // Minimal principle doc built on BaseSchema
  const MinimalCore = BaseCore.extend({
    type: Type,
    name: Label.optional(),
  });
  const MinimalDoc = z.object({
    core: MinimalCore,
    state: BaseState.default({}),
  });
  const MinimalSchema = BaseSchema.extend({
    shape: MinimalDoc,
  });

  it("parses a minimal document with core/state", () => {
    const doc = MinimalSchema.parse({
      shape: {
        core: { id: "min:1", type: "system.Minimal", name: "M1" },
        state: { status: "active", tags: [], meta: {} },
      },
    });
    expect(doc.shape.core.id).toBe("min:1");
    expect(doc.shape.core.type).toBe("system.Minimal");
    expect(doc.shape.state.status).toBe("active");
  });

  it("allows defaulted state via BaseState.default", () => {
    const doc = MinimalSchema.parse({
      shape: {
        core: { id: "min:2", type: "system.Minimal" },
        // state omitted on purpose (defaulted)
      },
    });
    expect(doc.shape.state).toBeDefined();
    expect(typeof doc.shape.state).toBe("object");
  });
});

