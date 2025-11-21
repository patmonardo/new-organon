import { describe, it, expect } from "vitest";
import {
  MorphSchema,
  createMorph,
  updateMorph,
  defineMorph,
  defineMorphPipeline,
} from '@schema';

describe("schema/morph — happy path", () => {
  it("creates a single Morph with defaults", () => {
    const m = createMorph({
      type: "system.Morph",
      name: "Normalize",
      transformFn: "@pkg/mod#normalize",
      // state omitted -> defaulted
    });

    const parsed = MorphSchema.parse(m);
    expect(parsed.shape.core.type).toBe("system.Morph");
    expect(parsed.shape.core.name).toBe("Normalize");
    expect(parsed.shape.core.inputType).toBe("FormShape");
    expect(parsed.shape.core.outputType).toBe("FormShape");
    expect(parsed.shape.composition.kind).toBe("single");
    expect(parsed.shape.composition.steps).toEqual([]);
    expect(typeof parsed.shape.state).toBe("object");
    expect(parsed.revision).toBe(0);
  });

  it("updates core/state/composition/config/meta and bumps revision", () => {
    const m0 = createMorph({ type: "system.Morph", name: "A" });

    const m1 = updateMorph(m0, {
      core: { name: "A1", outputType: "World" },
      state: { optimized: true, status: "active", tags: [], meta: {} },
      composition: { kind: "pipeline", mode: "sequential", steps: ["m:x", "m:y"] },
      config: { a: 1 },
      meta: { note: "ok" },
      version: "1.0.0",
      ext: { e: true },
    });

    const p = MorphSchema.parse(m1);
    expect(p.shape.core.name).toBe("A1");
    expect(p.shape.core.outputType).toBe("World");
    expect(p.shape.state.optimized).toBe(true);
    expect(p.shape.state.status).toBe("active");
    expect(p.shape.composition.kind).toBe("pipeline");
    expect(p.shape.composition.steps).toEqual(["m:x", "m:y"]);
    expect(p.shape.config).toEqual({ a: 1 });
    expect(p.shape.meta).toEqual({ note: "ok" });
    expect(p.version).toBe("1.0.0");
    expect(p.ext).toMatchObject({ e: true });
    expect(p.revision).toBe(m0.revision + 1);
  });

  it("defines single and pipeline morphs via helpers", () => {
    const single = defineMorph({
      type: "system.Morph",
      name: "One",
      transformFn: "@x#one",
    });
    const pipe = defineMorphPipeline("pipe:1", "Pipe", ["m:1", "m:2"], {
      description: "demo",
      inputType: "FormShape",
      outputType: "FormShape",
    });

    const s = MorphSchema.parse(single);
    const p = MorphSchema.parse(pipe);

    expect(s.shape.composition.kind).toBe("single");
    expect(s.shape.composition.steps).toEqual([]);

    expect(p.shape.core.name).toBe("Pipe");
    expect(p.shape.composition.kind).toBe("pipeline");
    expect(p.shape.composition.mode).toBe("sequential");
    expect(p.shape.composition.steps).toEqual(["m:1", "m:2"]);
  });
});

