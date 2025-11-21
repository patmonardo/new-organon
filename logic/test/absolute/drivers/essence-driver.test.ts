import { describe, it, expect } from 'vitest';
import {
  DefaultEssenceDriver,
  processShapes,
  commitShapes,
} from '../../../src/absolute/essence/essence';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { ShapeSchema } from '../../../src/schema/shape';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';

const repo = makeInMemoryRepository(ShapeSchema as any);

describe('EssenceDriver → ShapeEngine (ActiveShape)', () => {
  it('processes and commits ActiveShapes (create + update)', async () => {
    const shapes = [
      { id: 'shape:one', name: 'One', kind: 'shape', active: true },
      { id: 'shape:two', name: 'Two', kind: 'shape', active: true },
    ];

    // process via driver
    const { actions, snapshot } = await DefaultEssenceDriver.processShapes(
      shapes,
      { repo },
    );
    expect(snapshot.count).toBe(2);
    expect(
      actions.some((a) => a.type === 'shape.upsert' && a.id === 'shape:one'),
    ).toBe(true);

    // commit via driver
    const result = await DefaultEssenceDriver.commitShapes(shapes, { repo });
    expect(result.commitResult.success).toBe(true);

    // repository sanity: shape is persisted
    const doc = await repo.get('shape:one');
    expect(doc).toBeTruthy();
    expect((doc as any).shape.core.id).toBe('shape:one');
  });

  it('handles revoked (deletes)', async () => {
    // seed create
    await DefaultEssenceDriver.commitShapes(
      [{ id: 'shape:x', name: 'X', active: true }],
      { repo },
    );
    // revoke
    const { actions } = await processShapes(
      [{ id: 'shape:x', revoked: true }],
      { repo },
    );
    expect(
      actions.some((a) => a.type === 'shape.delete' && a.id === 'shape:x'),
    ).toBe(true);
    const { commitResult } = await commitShapes(
      [{ id: 'shape:x', revoked: true }],
      { repo },
    );
    expect(commitResult.success).toBe(true);
  });

  it('publishes events on the provided bus (create + setCore in one batch)', async () => {
    const localRepo = makeInMemoryRepository(ShapeSchema as any);
    const bus = new InMemoryEventBus();
    const events: string[] = [];
    bus.subscribe('shape.create', (e) => events.push(e.kind));
    bus.subscribe('shape.setCore', (e) => events.push(e.kind));

    // Two upserts for the same id in a single commit batch → create then setCore
    const shapes = [
      { id: 'shape:evt', name: 'Evt', active: true },
      { id: 'shape:evt', name: 'Evt v2', active: true },
    ];

    const { commitResult } = await commitShapes(shapes, {
      repo: localRepo,
      bus,
    });
    expect(commitResult.success).toBe(true);
    expect(events).toEqual(['shape.create', 'shape.setCore']);
  });
});
