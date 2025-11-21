import { describe, it, expect } from 'vitest';
import { EntityEngine } from '../../../src/relative/form/entity';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import type { Entity } from '../../../src/schema/entity';
import { makeInMemoryRepository } from '../../support/inMemoryRepo';

describe('EntityEngine.process/commit (ADR-0005)', () => {
  it('upserts an active entity and persists via repo', async () => {
    const repo = makeInMemoryRepository<Entity>();
    const bus = new InMemoryEventBus();
    const engine = new EntityEngine(repo as any, bus);

    const input = [
      { id: 'entity:alpha', entityType: 'system.Entity', active: true },
    ] as any[];

    const { actions, snapshot } = await engine.process(input);
    expect(snapshot.count).toBe(1);
    expect(actions).toEqual([
      { type: 'entity.upsert', id: 'entity:alpha', entityType: 'system.Entity', labels: undefined },
    ]);

    const result = await engine.commit(actions, snapshot);
    const kinds = (result.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['entity.create']);
    const saved = await repo.get('entity:alpha');
    expect(saved).toBeTruthy();
  });

  it('deletes when revoked=true and id present', async () => {
    const repo = makeInMemoryRepository<Entity>();
    const engine = new EntityEngine(repo as any, new InMemoryEventBus());

    // Seed an entity via command interface
    const [created] = await engine.handle({
      kind: 'entity.create',
      payload: { id: 'entity:beta', type: 'system.Entity' },
    } as any);
    expect(created.kind).toBe('entity.create');
    expect(await repo.get('entity:beta')).toBeTruthy();

    // Now process a revoked carrier
    const { actions } = await engine.process([
      { id: 'entity:beta', revoked: true, entityType: 'system.Entity' },
    ] as any[]);
    expect(actions).toEqual([{ type: 'entity.delete', id: 'entity:beta' }]);

    const res = await engine.commit(actions, { count: 1 });
    const kinds = (res.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['entity.delete']);
    expect(await repo.get('entity:beta')).toBeNull();
  });

  it('is idempotent across repeated upsert commits (create then core.set)', async () => {
    const repo = makeInMemoryRepository<Entity>();
    const engine = new EntityEngine(repo as any, new InMemoryEventBus());

    const input = [
      { id: 'entity:gamma', entityType: 'system.Entity', active: true },
    ] as any[];

    const p1 = await engine.process(input);
    const r1 = await engine.commit(p1.actions, p1.snapshot);
    const p2 = await engine.process(input);
    const r2 = await engine.commit(p2.actions, p2.snapshot);

    expect((r1.events ?? []).map((e: any) => e.kind)).toEqual(['entity.create']);
    expect((r2.events ?? []).map((e: any) => e.kind)).toEqual(['entity.setCore']);
  });

  it('preserves action ordering for multiple upserts', async () => {
    const repo = makeInMemoryRepository<Entity>();
    const engine = new EntityEngine(repo as any, new InMemoryEventBus());
    const { actions } = await engine.process([
      { id: 'entity:a', entityType: 'system.Entity' },
      { id: 'entity:b', entityType: 'system.Entity' },
    ] as any[]);
    expect(actions.map((a) => a.id)).toEqual(['entity:a', 'entity:b']);
  });
});
