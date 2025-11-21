import { describe, it, expect } from 'vitest';
import { EntityService } from '../../../src/relative/form/entity';
import { makeInMemoryRepository } from '../../support/inMemoryRepo';

describe('EntityService', () => {
  it('creates, updates, describes, and deletes entities with in-memory engine (no repo)', async () => {
    const svc = new EntityService();

    const received: string[] = [];
    const tap = (k: string) => svc.on(k, (e) => received.push(e.kind));
    [
      'entity.create',
      'entity.delete',
      'entity.describe',
      'entity.setCore',
      'entity.setState',
      'entity.patchState',
    ].forEach(tap);

    // create
    const id = await svc.create({ type: 'system.Entity', name: 'E' });
    expect(id).toBeTruthy();

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Entity');
    expect(info1.name).toBe('E');

    // update core
    await svc.setCore(id, { name: 'E2', type: 'system.Entity.Updated' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('E2');
    expect(info2.type).toBe('system.Entity.Updated');

    // state
    await svc.setState(id, { status: 'active' } as any);
    const info3 = await svc.describe(id);
    expect((info3.state as any).status).toBe('active');
    await svc.patchState(id, { meta: { ok: true } } as any);
    const info4 = await svc.describe(id);
    expect((info4.state as any).meta.ok).toBe(true);

    // delete
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();

    // event sequence sanity: first is created, last is deleted
    expect(received[0]).toBe('entity.create');
    expect(received[received.length - 1]).toBe('entity.delete');

    // contains expected mutation events at least once
    const has = (k: string) => received.includes(k);
    ['entity.setCore', 'entity.setState', 'entity.patchState'].forEach(
      (k) => expect(has(k)).toBe(true),
    );

    // number of describes equals number of describe() calls we made
    const describedCount = received.filter(
      (k) => k === 'entity.describe',
    ).length;
    expect(describedCount).toBe(4);
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    const repo = makeInMemoryRepository<any>();
    const svc = new EntityService(repo as any);

    const id = await svc.create({ type: 'system.Entity', name: 'Repo' });

    // get via repo-backed path
    const doc1 = await svc.get(id);
    expect(doc1?.shape.core.name).toBe('Repo');

    // mutate core and verify persisted
    await svc.setCore(id, { name: 'Repo2' });
    const doc2 = await svc.get(id);
    expect(doc2?.shape.core.name).toBe('Repo2');

    // delete and ensure gone
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();
  });
});
