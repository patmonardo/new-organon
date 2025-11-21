import { describe, it, expect } from 'vitest';
import { PropertyService } from '../../../src/relative/form/property';

describe('PropertyService', () => {
  it('creates, updates, describes, and deletes properties with in-memory engine (no repo)', async () => {
    const svc = new PropertyService();

    const receivedEvents: any[] = [];
    const tap = (k: string) => svc.on(k, (e) => receivedEvents.push(e));
    // canonical noun.verb event kinds
    [
      'property.create',
      'property.delete',
      'property.setCore',
      'property.setState',
      'property.patchState',
      'property.describe',
    ].forEach(tap);

    // create
    const id = await svc.create({
      type: 'system.Property',
      name: 'P',
      key: 'p',
      contextId: 'ctx:world',
    });
    expect(id).toBeTruthy();

    // emitted create event contains canonical payload.id
    expect(receivedEvents.length).toBeGreaterThanOrEqual(1);
    expect(receivedEvents[0].payload?.id).toBe(id);

    // describe baseline
    const info1 = await svc.describe(id);
    expect(info1.id).toBe(id);
    expect(info1.type).toBe('system.Property');
    expect(info1.name).toBe('P');

    // update core
    await svc.setCore(id, { name: 'P2', type: 'system.Property.Updated' });
    const info2 = await svc.describe(id);
    expect(info2.name).toBe('P2');
    expect(info2.type).toBe('system.Property.Updated');

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
    const receivedKinds = receivedEvents.map((e) => e.kind);
    expect(receivedKinds[0]).toBe('property.create');
    expect(receivedKinds).toContain('property.setCore');
    expect(receivedKinds).toContain('property.setState');
    expect(receivedKinds).toContain('property.patchState');
    expect(receivedKinds).toContain('property.describe');
    expect(receivedKinds[receivedKinds.length - 1]).toBe('property.delete');
  });
});
