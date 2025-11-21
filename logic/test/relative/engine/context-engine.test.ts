import { describe, it, expect } from 'vitest';
import { ContextEngine } from '../../../src/relative/form/context';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { makeInMemoryRepository } from '../../support/inMemoryRepo';

describe('ContextEngine.process/commit (ADR-0003)', () => {
  it('upserts an active context and persists via repo', async () => {
    const repo = makeInMemoryRepository<any>();
    const bus = new InMemoryEventBus();
    const engine = new ContextEngine(repo as any, bus);

    const input = [
      { id: 'context:alpha', kind: 'domain', name: 'Alpha', active: true },
    ] as any[];

    const { actions, snapshot } = await engine.process(input);
    expect(snapshot.count).toBe(1);
    expect(actions).toEqual([
      { type: 'context.upsert', id: 'context:alpha', name: 'Alpha', kind: 'domain', scope: undefined },
    ]);

    const result = await engine.commit(actions, snapshot);
    const kinds = (result.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['context.create']);
    const saved = await repo.get('context:alpha');
    expect(saved).toBeTruthy();
  });

  it('deletes when revoked=true and id present', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new ContextEngine(repo as any, new InMemoryEventBus());

    // Seed a context via command interface
    const [created] = await engine.handle({
      kind: 'context.create',
      payload: { id: 'context:beta', type: 'domain', name: 'Beta' },
    } as any);
    expect(created.kind).toBe('context.create');
    expect(await repo.get('context:beta')).toBeTruthy();

    // Now process a revoked carrier
    const { actions } = await engine.process([
      { id: 'context:beta', revoked: true },
    ] as any[]);
    expect(actions).toEqual([{ type: 'context.delete', id: 'context:beta' }]);

    const res = await engine.commit(actions, { count: 1 });
    const kinds = (res.events ?? []).map((e: any) => e.kind);
    expect(kinds).toEqual(['context.delete']);
    expect(await repo.get('context:beta')).toBeNull();
  });

  it('is idempotent across repeated upsert commits (create then setCore)', async () => {
    const repo = makeInMemoryRepository<any>();
    const engine = new ContextEngine(repo as any, new InMemoryEventBus());

    const input = [
      { id: 'context:gamma', kind: 'domain', name: 'Gamma', active: true },
    ] as any[];

    const p1 = await engine.process(input);
    const r1 = await engine.commit(p1.actions, p1.snapshot);
    const p2 = await engine.process(input);
    const r2 = await engine.commit(p2.actions, p2.snapshot);

    expect((r1.events ?? []).map((e: any) => e.kind)).toEqual(['context.create']);
    expect((r2.events ?? []).map((e: any) => e.kind)).toEqual(['context.setCore']);
  });

  it('preserves action ordering for multiple upserts', async () => {
    const engine = new ContextEngine(makeInMemoryRepository<any>() as any, new InMemoryEventBus());
    const { actions } = await engine.process([
      { id: 'context:a', kind: 'domain', name: 'A' },
      { id: 'context:b', kind: 'domain', name: 'B' },
    ] as any[]);
    expect(actions.map((a) => a.id)).toEqual(['context:a', 'context:b']);
  });
});
