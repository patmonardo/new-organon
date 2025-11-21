import { describe, it, expect } from 'vitest';
import { MorphService } from '../../../src/relative/form/morph';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { MorphSchema } from '../../../src/schema/morph';

describe('MorphService', () => {
  it('creates, updates, describes, and deletes morphs with in-memory engine (no repo)', async () => {
    const svc = new MorphService();

    const receivedEvents: any[] = [];
    const tap = (k: string) => svc.on(k, (e) => receivedEvents.push(e));
    [
      'morph.create',
      'morph.delete',
      'morph.setCore',
      'morph.setState',
      'morph.patchState',
      'morph.describe',
    ].forEach(tap);

    // create
    const id = await svc.create({ type: 'system.Morph', name: 'M' });
    expect(id).toBeTruthy();

    // emitted create event contains canonical payload.id
    expect(receivedEvents.length).toBeGreaterThanOrEqual(1);
    expect(receivedEvents[0].payload?.id).toBe(id);

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Morph');
    expect(info1.name).toBe('M');

    // update core
    await svc.setCore(id, { name: 'M2', type: 'system.Morph' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('M2');
    expect(info2.type).toBe('system.Morph');

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

    // optional sanity: kind sequence
    const kinds = receivedEvents.map((e) => e.kind);
    expect(kinds[0]).toBe('morph.create');
    expect(kinds).toContain('morph.setCore');
    expect(kinds).toContain('morph.setState');
    expect(kinds).toContain('morph.patchState');
    expect(kinds).toContain('morph.describe');
    expect(kinds[kinds.length - 1]).toBe('morph.delete');
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    // Use the same repository factory as MorphService
    const repo = makeInMemoryRepository(MorphSchema as any);
    const svc = new MorphService(repo as any);

    const id = await svc.create({ type: 'system.Morph', name: 'Repo' });

    // get via repo-backed path
    const doc1 = await svc.get(id);
    expect(doc1?.shape.core.name).toBe('Repo');

    // mutate core and verify persisted
    await svc.setCore(id, { name: 'Repo2', type: 'system.Morph' });
    const doc2 = await svc.get(id);
    expect(doc2?.shape.core.name).toBe('Repo2');

    // delete and ensure gone
    await svc.delete(id);
    await expect(svc.get(id)).resolves.toBeUndefined();
  });
});
