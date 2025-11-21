import { describe, it, expect } from 'vitest';
import {
  DefaultThingDriver,
  processEntities,
  commitEntities,
} from '../../../src/absolute/essence/thing';
import { EntityEngine } from '../../../src/relative/form/entity';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { EntitySchema } from '../../../src/schema/entity';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';

const repo = makeInMemoryRepository(EntitySchema as any);

// helper: resilient payload id extractor (accepts evt or raw payload)
function payloadId(evtOrPayload: any): string | undefined {
  const p =
    evtOrPayload && evtOrPayload.payload ? evtOrPayload.payload : evtOrPayload;
  const id =
    p?.shape?.core?.id ?? p?.shape?.id ?? p?.id ?? p?.entityId ?? undefined;
  return typeof id === 'undefined' ? undefined : String(id);
}

describe('ThingDriver → EntityEngine (ActiveEntity)', () => {
  it('processes and commits ActiveEntities (create + update)', async () => {
    const entities = [
      { id: 'entity:one', entityType: 'system.Entity', active: true },
      { id: 'entity:two', entityType: 'system.Entity', active: true },
    ];

    const { actions, snapshot } = await DefaultThingDriver.processEntities(
      entities,
      { repo },
    );
    expect(snapshot.count).toBe(2);
    expect(
      actions.some((a) => a.type === 'entity.upsert' && a.id === 'entity:one'),
    ).toBe(true);

    const result = await DefaultThingDriver.commitEntities(entities, { repo });
    expect(result.commitResult.success).toBe(true);

    const engine = new EntityEngine(repo as any);
    const [evt] = await engine.handle({
      kind: 'entity.describe',
      payload: { id: 'entity:one' },
    } as any);
    expect(evt.kind).toBe('entity.describe');
    expect(payloadId(evt)).toBe('entity:one');
  });

  it('handles revoked (deletes)', async () => {
    await DefaultThingDriver.commitEntities(
      [{ id: 'entity:x', entityType: 'system.Entity', active: true }],
      { repo },
    );
    const { actions } = await processEntities(
      [{ id: 'entity:x', entityType: 'system.Entity', revoked: true }],
      { repo },
    );
    expect(
      actions.some((a) => a.type === 'entity.delete' && a.id === 'entity:x'),
    ).toBe(true);
    const { commitResult } = await commitEntities(
      [{ id: 'entity:x', entityType: 'system.Entity', revoked: true }],
      { repo },
    );
    expect(commitResult.success).toBe(true);
  });

  it('publishes events on the provided bus (create + setCore in one batch)', async () => {
    const localRepo = makeInMemoryRepository(EntitySchema as any);
    const bus = new InMemoryEventBus();
    const events: string[] = [];
    const payloads: any[] = [];
    bus.subscribe('entity.create', (e) => {
      events.push(e.kind);
      payloads.push(e);
    });
    bus.subscribe('entity.setCore', (e) => {
      events.push(e.kind);
      payloads.push(e);
    });

    const entities = [
      { id: 'entity:evt', entityType: 'system.Entity', active: true },
      { id: 'entity:evt', entityType: 'system.Entity', active: true },
    ];

    const { commitResult } = await commitEntities(entities, {
      repo: localRepo,
      bus,
    });
    expect(commitResult.success).toBe(true);
    expect(events).toEqual(['entity.create', 'entity.setCore']);

    // ensure payloads include canonical id (or legacy fallback)
    expect(payloadId(payloads[0])).toBe('entity:evt');
    expect(payloadId(payloads[1])).toBe('entity:evt');
  });
});
