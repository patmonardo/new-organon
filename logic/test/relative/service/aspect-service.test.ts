import { describe, it, expect } from 'vitest';
import { AspectService } from '../../../src/relative/form/aspect';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { AspectSchema } from '../../../src/schema/aspect';

describe('AspectService', () => {
  it('creates, updates, describes, and deletes aspects with in-memory engine (no repo)', async () => {
    const svc = new AspectService();

    const receivedEvents: any[] = [];
    const tap = (k: string) => svc.on(k, (e) => receivedEvents.push(e));
    [
      'aspect.created',
      'aspect.deleted',
      'aspect.setCore',
      'aspect.setState',
      'aspect.patchState',
      'aspect.describe',
    ].forEach(tap);

    // create
    const id = await svc.create({ type: 'system.Aspect', name: 'A' });
    expect(id).toBeTruthy();

    // emitted create event contains canonical payload.id
    expect(receivedEvents.length).toBeGreaterThanOrEqual(1);
    expect(receivedEvents[0].payload?.id).toBe(id);

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Aspect');
    expect(info1.name).toBe('A');

    // update core - keep canonical type
    await svc.setCore(id, { name: 'A2', type: 'system.Aspect' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('A2');
    expect(info2.type).toBe('system.Aspect');

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

    // kind sequence sanity - use correct event names
    const kinds = receivedEvents.map((e) => e.kind);
    expect(kinds[0]).toBe('aspect.created');
    expect(kinds).toContain('aspect.setCore');
    expect(kinds).toContain('aspect.setState');
    expect(kinds).toContain('aspect.patchState');
    expect(kinds).toContain('aspect.describe');
    expect(kinds[kinds.length - 1]).toBe('aspect.deleted');
  });

  it('prefers repo for get() and persists changes when repo is provided', async () => {
    const repo = makeInMemoryRepository(AspectSchema as any);
    const svc = new AspectService(repo as any);

    const id = await svc.create({ type: 'system.Aspect', name: 'Repo' });

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
