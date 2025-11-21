import { describe, it, expect } from 'vitest';
import { ContextService } from '../../../src/relative/form/context';
import { makeInMemoryRepository } from '../../support/inMemoryRepo';

describe('ContextService', () => {
  it('creates, updates, describes, and deletes contexts with in-memory engine (no repo)', async () => {
    const svc = new ContextService();

    const received: string[] = [];
    const tap = (k: string) => svc.on(k, (e) => received.push(e.kind));
    [
      'context.create',
      'context.delete',
      'context.setCore',
      'context.setState',
      'context.patchState',
      'context.describe',
    ].forEach(tap);

    // create
    const id = await svc.create({ type: 'system.Context', name: 'C' });
    expect(id).toBeTruthy();

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Context');
    expect(info1.name).toBe('C');

    // update core
    await svc.setCore(id, { name: 'C2', type: 'system.Context.Updated' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('C2');
    expect(info2.type).toBe('system.Context.Updated');

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
    expect(received[0]).toBe('context.create');
    expect(received[received.length - 1]).toBe('context.delete');

    // contains expected mutation events at least once
    const has = (k: string) => received.includes(k);
    ['context.setCore', 'context.setState', 'context.patchState'].forEach((k) =>
      expect(has(k)).toBe(true),
    );

    // number of describes equals number of describe() calls we made
    const describedCount = received.filter(
      (k) => k === 'context.describe',
    ).length;
    expect(describedCount).toBe(4);
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    const repo = makeInMemoryRepository<any>();
    const svc = new ContextService(repo as any);

    const id = await svc.create({ type: 'system.Context', name: 'Repo' });

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
