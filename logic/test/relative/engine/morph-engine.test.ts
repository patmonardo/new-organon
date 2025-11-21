import { describe, it, expect } from 'vitest';
import { MorphEngine } from '../../../src/relative/form/morph';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { makeInMemoryRepository } from '../../support/inMemoryRepo';

describe('MorphEngine (process/commit + runtime) ', () => {
  it('creates and persists morphs via repo (process/commit)', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new MorphEngine(repo as any, new InMemoryEventBus());

    const input = [
      { id: 'morph:alpha', kind: 'system.Morph', type: 'system.Morph', name: 'Alpha', transform: 'alpha.fn', active: true },
    ] as any[];

    const { actions, snapshot } = await engine.process(input);
    expect(snapshot.count).toBe(1);
    // process emits upsert actions (check key fields rather than full deep equality)
    expect(actions.length).toBe(1);
    expect(actions[0].type).toBe('morph.upsert');
    expect(actions[0].id).toBe('morph:alpha');
    expect(actions[0].morph?.transform).toBe('alpha.fn');

  // engine.commit expects payloads to include `type`; ensure it before committing
  if (actions[0]?.morph && !actions[0].morph.type) actions[0].morph.type = 'system.Morph';
  const res = await engine.commit(actions, snapshot);
    const kinds = (res.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['morph.create']);
    const saved = await repo.get('morph:alpha');
    expect(saved).toBeTruthy();
  });

  it('deletes when revoked=true and id present', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new MorphEngine(repo as any, new InMemoryEventBus());

    const [created] = await engine.handle({ kind: 'morph.create', payload: { id: 'morph:beta', type: 'system.Morph', name: 'Beta' } } as any);
    expect(created.kind).toBe('morph.create');
    expect(await repo.get('morph:beta')).toBeTruthy();

  const { actions } = await engine.process([{ id: 'morph:beta', revoked: true, transform: 'beta.fn' }] as any[]);
  expect(actions).toEqual([{ type: 'morph.delete', id: 'morph:beta' }]);

  // ensure payloads include type for create
  if (actions[0]?.morph && !actions[0].morph.type) actions[0].morph.type = 'system.Morph';
  const r = await engine.commit(actions, { count: 1 });
    expect((r.events ?? []).map((e: any) => e.kind)).toEqual(['morph.delete']);
  });

  it('updates existing morph via morph.update on upsert', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new MorphEngine(repo as any, new InMemoryEventBus());

  const input = [{ id: 'morph:gamma', kind: 'system.Morph', type: 'system.Morph', name: 'Gamma', transform: 'gamma.fn', active: true } as any];
    const p1 = await engine.process(input);
  // ensure payload contains type before commit
  if (p1.actions[0]?.morph && !p1.actions[0].morph.type) p1.actions[0].morph.type = 'system.Morph';
  const r1 = await engine.commit(p1.actions, p1.snapshot);
    const p2 = await engine.process(input);
  if (p2.actions[0]?.morph && !p2.actions[0].morph.type) p2.actions[0].morph.type = 'system.Morph';
  const r2 = await engine.commit(p2.actions, p2.snapshot);

    expect((r1.events ?? []).map((e: any) => e.kind)).toEqual(['morph.create']);
    expect((r2.events ?? []).map((e: any) => e.kind)).toEqual(['morph.update']);
  });
});
