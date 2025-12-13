import { describe, it, expect } from 'vitest';

import { InMemoryEventBus } from '../../src/absolute/core/bus';
import { InMemoryRepository } from '../../src/repository/memory';

import { EntityEngine } from '../../src/relative/form/entity/entity-engine';
import { PropertyEngine } from '../../src/relative/form/property/property-engine';

import { PropertySchema } from '../../src/schema/property';

describe('Objective World System seed (Thing/World/Relation)', () => {
  it('EntityEngine attaches meta (Thing) on create/setCore/setState', async () => {
    const bus = new InMemoryEventBus();
    const engine = new EntityEngine(undefined as any, bus);

    const [created] = await engine.handle({
      kind: 'entity.create',
      payload: { id: 'entity:1', type: 'system.Entity', name: 'Invoice#1', formId: 'shape:invoice' },
    });

    expect((created.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'assert',
      kind: 'Entity',
      ids: ['entity:1'],
    });
    expect((created.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'entity', rule: 'thing' }]),
    );

    const [setCore] = await engine.handle({
      kind: 'entity.setCore',
      payload: { id: 'entity:1', name: 'Invoice#1b' },
    });
    expect((setCore.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'revise',
      kind: 'Entity',
      ids: ['entity:1'],
    });

    const [setState] = await engine.handle({
      kind: 'entity.setState',
      payload: { id: 'entity:1', state: { status: 'active' } },
    });
    expect((setState.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'revise',
      kind: 'Entity',
      ids: ['entity:1'],
    });
  });

  it('PropertyEngine attaches meta (Relation/Law) on create/setCore/setState', async () => {
    const bus = new InMemoryEventBus();
    const repo = new InMemoryRepository(PropertySchema);
    const engine = new PropertyEngine(repo as any, bus);

    const [created] = await engine.handle({
      kind: 'property.create',
      payload: { id: 'property:1', type: 'system.Property', name: 'MustHaveFormId' },
    });

    expect((created.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'assert',
      kind: 'Property',
      ids: ['property:1'],
    });
    expect((created.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'property', rule: 'relation' }]),
    );

    const [setCore] = await engine.handle({
      kind: 'property.setCore',
      payload: { id: 'property:1', name: 'MustHaveFormId2' },
    });
    expect((setCore.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'revise',
      kind: 'Property',
      ids: ['property:1'],
    });

    const [setState] = await engine.handle({
      kind: 'property.setState',
      payload: { id: 'property:1', state: { status: 'active' } },
    });
    expect((setState.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'revise',
      kind: 'Property',
      ids: ['property:1'],
    });
  });

  it('Law bites: EntityEngine dialectic.invariant.check can violate an eq/exists predicate', async () => {
    const bus = new InMemoryEventBus();
    const engine = new EntityEngine(undefined as any, bus);

    await engine.handle({
      kind: 'entity.create',
      payload: { id: 'entity:2', type: 'system.Entity', name: 'Draft', formId: 'shape:invoice' },
    });

    const events = await engine.handle({
      kind: 'dialectic.invariant.check',
      payload: {
        stateId: 'entity:2',
        invariants: [
          {
            id: 'inv.formId',
            constraint: 'Entity must have a formId',
            predicate: 'exists:shape.entity.formId',
          },
          {
            id: 'inv.statusActive',
            constraint: 'Entity must be active',
            predicate: 'eq:shape.state.status:"active"',
          },
        ],
      },
    } as any);

    // We did not set status=active, so we expect at least one violation.
    expect(events.some((e) => e.kind === 'dialectic.invariant.violated')).toBe(true);
  });
});
