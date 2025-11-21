import { describe, it, expect } from 'vitest';
import { AspectEngine } from '@relative/form/aspect/aspect-engine';
import { InMemoryEventBus } from '@absolute';
import { makeInMemoryRepository } from '@repository';
// import { AspectSchema } from '@schema/aspect';
import { AspectSchema } from '@schema';

describe('AspectEngine (process/commit)', () => {
  it('creates and persists aspects via repo (process/commit)', async () => {
    const repo = makeInMemoryRepository(AspectSchema as any);
    const engine = new AspectEngine(repo as any, new InMemoryEventBus());

    const input = [{ id: 'aspect:alpha', name: 'alpha' }];

    const { actions, snapshot } = await engine.process(input as any);
    expect(snapshot.count).toBe(1);
    expect(actions.length).toBe(1);
    expect(actions[0].type).toBe('aspect.upsert');
    expect(actions[0].id).toBe('aspect:alpha');

    const res = await engine.commit(actions, snapshot);
    const kinds = (res.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['aspect.created']);

    const saved = await repo.get('aspect:alpha');
    expect(saved).toBeTruthy();
    expect(saved.shape.core.name).toBe('alpha');
  });

  it('deletes when revoked=true and id present', async () => {
    const repo = makeInMemoryRepository(AspectSchema as any);
    const engine = new AspectEngine(repo as any, new InMemoryEventBus());

    // Create first
    await engine.handle({
      kind: 'aspect.create',
      payload: { id: 'aspect:beta', type: 'system.Aspect', name: 'beta' },
    } as any);

    // Then delete via process/commit
    const { actions } = await engine.process([{ id: 'aspect:beta', revoked: true }] as any);
    const res = await engine.commit(actions, { count: 1 });

    expect((res.events ?? []).map((e: any) => e.kind)).toEqual(['aspect.deleted']);
    expect(await repo.get('aspect:beta')).toBeNull();
  });

  it('updates existing aspect via setCore', async () => {
    const repo = makeInMemoryRepository(AspectSchema as any);
    const engine = new AspectEngine(repo as any, new InMemoryEventBus());

    // Create initial aspect
    const input1 = [{ id: 'aspect:gamma', name: 'gamma' }];
    const p1 = await engine.process(input1 as any);
    const r1 = await engine.commit(p1.actions, p1.snapshot);

    // Update the same aspect (triggers setCore)
    const input2 = [{ id: 'aspect:gamma', name: 'gamma-updated' }];
    const p2 = await engine.process(input2 as any);
    const r2 = await engine.commit(p2.actions, p2.snapshot);

    expect((r1.events ?? []).map((e: any) => e.kind)).toEqual(['aspect.created']);
    expect((r2.events ?? []).map((e: any) => e.kind)).toEqual(['aspect.setCore']);

    const saved = await repo.get('aspect:gamma');
    expect(saved?.shape.core.name).toBe('gamma-updated');
  });
});
