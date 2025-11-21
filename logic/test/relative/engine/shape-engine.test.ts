import { describe, it, expect } from 'vitest';
import { ShapeEngine } from '../../../src/relative/form/shape';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { makeInMemoryRepository } from '../../support/inMemoryRepo';

describe('ShapeEngine.process/commit (ADR-0002)', () => {
  it('upserts an active shape and persists via repo', async () => {
    const repo = makeInMemoryRepository<any>();
    const bus = new InMemoryEventBus();
    const engine = new ShapeEngine(repo as any, bus);

    const input = [
      { id: 'shape:alpha', kind: 'system.Shape', name: 'Alpha', active: true },
    ] as any[];

    const { actions, snapshot } = await engine.process(input);
    expect(snapshot.count).toBe(1);
    expect(actions).toEqual([
      { type: 'shape.upsert', id: 'shape:alpha', name: 'Alpha', kind: 'system.Shape' },
    ]);

    const result = await engine.commit(actions, snapshot);
    const kinds = (result.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['shape.create']);
    const s = engine.getShape('shape:alpha');
    expect(s?.type).toBe('system.Shape');
    expect(s?.name).toBe('Alpha');
  });

  it('deletes when revoked=true and id present', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new ShapeEngine(repo as any, new InMemoryEventBus());

    // Seed a shape via command interface
    const [created] = await engine.handle({
      kind: 'shape.create',
      payload: { id: 'shape:beta', type: 'system.Shape', name: 'Beta' },
    } as any);
    expect(created.kind).toBe('shape.create');
    expect(engine.getShape('shape:beta')).toBeTruthy();

    // Now process a revoked carrier
    const { actions } = await engine.process([
      { id: 'shape:beta', revoked: true },
    ] as any[]);
    expect(actions).toEqual([
      { type: 'shape.delete', id: 'shape:beta', sourceShapeId: 'shape:beta' },
    ]);

    const res = await engine.commit(actions, { count: 1 });
    const kinds = (res.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['shape.delete']);
    expect(engine.getShape('shape:beta')).toBeUndefined();
  });

  it('is idempotent across repeated upsert commits (create then setCore)', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new ShapeEngine(repo as any, new InMemoryEventBus());

    const input = [
      { id: 'shape:gamma', kind: 'system.Shape', name: 'Gamma', active: true },
    ] as any[];

    const p1 = await engine.process(input);
    const r1 = await engine.commit(p1.actions, p1.snapshot);
    const p2 = await engine.process(input);
    const r2 = await engine.commit(p2.actions, p2.snapshot);

    expect((r1.events ?? []).map((e: any) => e.kind)).toEqual(['shape.create']);
    expect((r2.events ?? []).map((e: any) => e.kind)).toEqual(['shape.setCore']);
  });

  it('derives deterministic ids from name when id is missing', async () => {
    const engine = new ShapeEngine(makeInMemoryRepository<any>() as any, new InMemoryEventBus());

    const { actions } = await engine.process([
      { name: 'Hello World', kind: 'system.Shape', active: true },
    ] as any[]);
    expect(actions[0].id).toBe('shape:hello-world');

    const res = await engine.commit(actions, { count: 1 });
    const id = (res.events?.[0] as any).payload.id as string;
    expect(id).toBe('shape:hello-world');
    expect(engine.getShape('shape:hello-world')).toBeTruthy();
  });

  it('preserves action ordering for multiple upserts', async () => {
    const engine = new ShapeEngine(makeInMemoryRepository<any>() as any, new InMemoryEventBus());
    const { actions } = await engine.process([
      { id: 'shape:a', kind: 'system.Shape', name: 'A' },
      { id: 'shape:b', kind: 'system.Shape', name: 'B' },
    ] as any[]);
    expect(actions.map((a) => a.id)).toEqual(['shape:a', 'shape:b']);
  });
});
