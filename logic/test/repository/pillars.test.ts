import { describe, it, expect } from "vitest";
import { Repos } from "../../src/repository";
import { createEntity, createContext, createProperty } from "../../src/schema";

describe("InMemoryRepository + Pillars (CRUD only)", () => {
  it("creates, updates and deletes Entity", async () => {
    const entities = Repos.entity();

    const e = await entities.create(createEntity({ type: "system.Entity", name: "A" }));
    expect(e.shape.core.id).toBeTruthy();
    expect(e.shape.core.type).toBe("system.Entity");
    expect(e.shape.core.name).toBe("A");
    expect(e.revision).toBe(0);

    const updated = await entities.update(
      e.shape.core.id,
      (cur) => ({ ...cur, shape: { ...cur.shape, state: { ...cur.shape.state, foo: "bar" } } })
    );
    expect(updated.revision).toBeGreaterThan(e.revision ?? 0);
    expect((updated.shape.state as any).foo).toBe("bar");

    const deleted = await entities.delete(e.shape.core.id);
    expect(deleted).toBe(true);
    const missing = await entities.get(e.shape.core.id);
    expect(missing).toBeNull();
  });

  it("creates, updates and deletes Context (standard fields only)", async () => {
    const ctxs = Repos.context();

    const c = await ctxs.create(createContext({ type: "system.Context", name: "World" }));
    expect(c.shape.core.id).toBeTruthy();
    expect(c.shape.core.type).toBe("system.Context");
    expect(c.shape.core.name).toBe("World");
    expect(c.revision).toBe(0);

    const c2 = await ctxs.update(
      c.shape.core.id,
      (cur) => ({ ...cur, shape: { ...cur.shape, state: { ...cur.shape.state, mark: true } } })
    );
    expect(c2.revision).toBeGreaterThan(c.revision ?? 0);
    expect((c2.shape.state as any).mark).toBe(true);

    const del = await ctxs.delete(c.shape.core.id);
    expect(del).toBe(true);
    expect(await ctxs.get(c.shape.core.id)).toBeNull();
  });

  it("creates, updates and deletes Property (skeletal)", async () => {
    const props = Repos.property();

    const p = await props.create(
      createProperty({
        type: "system.Property",
        name: "title",
      })
    );

    expect(p.shape.core.id).toBeTruthy();
    expect(p.shape.core.type).toBe("system.Property");
    expect(p.shape.core.name).toBe("title");
    expect(p.revision).toBe(0);

    const p2 = await props.update(
      p.shape.core.id,
      (cur) => ({
        ...cur,
        shape: {
          ...cur.shape,
          state: { ...(cur.shape.state as any), any: "Hello" },
          signature: { s: 1 },
          facets: { ns: { a: 1 } },
        },
      })
    );

    expect((p2.shape.state as any).any).toBe("Hello");
    expect((p2.shape.signature as any).s).toBe(1);
    expect((p2.shape.facets as any).ns).toEqual({ a: 1 });

    const ok = await props.delete(p.shape.core.id);
    expect(ok).toBe(true);
    expect(await props.get(p.shape.core.id)).toBeNull();
  });

  it("creates Entity with state, signature and facets", async () => {
    const entities = Repos.entity();
    const e = await entities.create(
      createEntity({
        type: "system.Entity",
        name: "A",
        state: { foo: "bar" } as any,
        signature: { s: 1 },
        facets: { ns: { a: 1 } },
      })
    );

    expect(e.shape.core.id).toBeTruthy();
    expect(e.shape.core.type).toBe("system.Entity");
    expect(e.shape.core.name).toBe("A");
    expect((e.shape.state as any).foo).toBe("bar");
    expect((e.shape.signature as any).s).toBe(1);
    expect((e.shape.facets as any).ns).toEqual({ a: 1 });
  });

  it("creates Context with state", async () => {
    const ctxs = Repos.context();

    const c = await ctxs.create(
      createContext({
        type: "system.Context",
        name: "World",
        state: { mark: true } as any,
      })
    );

    expect(c.shape.core.id).toBeTruthy();
    expect(c.shape.core.type).toBe("system.Context");
    expect(c.shape.core.name).toBe("World");
    expect((c.shape.state as any).mark).toBe(true);
  });

  it("creates Property with state, signature and facets", async () => {
    const props = Repos.property();

    const p = await props.create(
      createProperty({
        type: "system.Property",
        name: "title",
        state: { any: "Hello" } as any,
        signature: { s: 1 },
        facets: { ns: { a: 1 } },
      })
    );

    expect(p.shape.core.name).toBe("title");
    expect((p.shape.state as any).any).toBe("Hello");
    expect((p.shape.signature as any).s).toBe(1);
    expect((p.shape.facets as any).ns).toEqual({ a: 1 });
  });
});
