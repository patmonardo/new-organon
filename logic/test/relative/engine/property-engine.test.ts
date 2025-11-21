import { describe, it, expect } from 'vitest';
import { PropertyEngine } from '../../../src/relative/form/property/property-engine';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { PropertySchema } from '../../../src/schema/property';

describe('PropertyEngine.process/commit (simplified)', () => {
  it('creates a property and persists via repo', async () => {
    const repo = makeInMemoryRepository(PropertySchema as any);
    const engine = new PropertyEngine(repo as any, new InMemoryEventBus());

    const input = [{ id: 'prop:alpha', name: 'color' }];

    const { actions, snapshot } = await engine.process(input as any);
    expect(snapshot.count).toBe(1);
    expect(actions[0].type).toBe('property.upsert');

    const res = await engine.commit(actions, snapshot);
    expect(res.success).toBe(true);
    expect((res.events ?? []).map((e: any) => e.kind)).toEqual([
      'property.create',
    ]);

    const saved = await repo.get('prop:alpha');
    expect(saved).toBeTruthy();
    expect(saved.shape.core.name).toBe('color');
  });

  it('deletes when revoked=true', async () => {
    const repo = makeInMemoryRepository(PropertySchema as any);
    const engine = new PropertyEngine(repo as any, new InMemoryEventBus());

    // Create first
    await engine.handle({
      kind: 'property.create',
      payload: { id: 'prop:beta', type: 'system.Property', name: 'size' },
    } as any);

    // Then delete
    const { actions } = await engine.process([
      { id: 'prop:beta', revoked: true },
    ] as any);
    const res = await engine.commit(actions, { count: 1 });

    expect((res.events ?? []).map((e: any) => e.kind)).toEqual([
      'property.delete',
    ]);
    expect(await repo.get('prop:beta')).toBeNull();
  });
});
