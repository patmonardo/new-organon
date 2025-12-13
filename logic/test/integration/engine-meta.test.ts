import { describe, it, expect } from 'vitest';

import { InMemoryEventBus } from '../../src/absolute/core/bus';
import { InMemoryRepository } from '../../src/repository/memory';

import { ShapeEngine } from '../../src/relative/form/shape/shape-engine';
import { ContextEngine } from '../../src/relative/form/context/context-engine';
import { MorphEngine } from '../../src/relative/form/morph/morph-engine';

import { MorphSchema } from '../../src/schema/morph';

describe('Engine meta wiring (FactStore + Dialectic)', () => {
  it('ShapeEngine attaches meta on create/setCore/setState', async () => {
    const bus = new InMemoryEventBus();
    const engine = new ShapeEngine(undefined as any, bus);

    const [created] = await engine.handle({
      kind: 'shape.create',
      payload: { id: 'shape:1', name: 'Invoice', type: 'system.Form' },
    });

    expect(created.kind).toBe('shape.create');
    expect((created.meta as any)?.factStore).toMatchObject({
      mode: 'reflection',
      store: 'FormDB',
      op: 'assert',
      kind: 'FormShape',
      ids: ['shape:1'],
    });
    expect((created.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'shape', rule: 'posting' }]),
    );

    const [setCore] = await engine.handle({
      kind: 'shape.setCore',
      payload: { id: 'shape:1', name: 'Invoice2' },
    });
    expect((setCore.meta as any)?.factStore).toMatchObject({
      mode: 'reflection',
      store: 'FormDB',
      op: 'revise',
      kind: 'FormShape',
      ids: ['shape:1'],
    });
    expect((setCore.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'shape', rule: 'external' }]),
    );

    const [setState] = await engine.handle({
      kind: 'shape.setState',
      payload: { id: 'shape:1', state: { status: 'active' } },
    });
    expect((setState.meta as any)?.factStore).toMatchObject({
      mode: 'reflection',
      store: 'FormDB',
      op: 'revise',
      kind: 'FormShape',
      ids: ['shape:1'],
    });
    expect((setState.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'shape', rule: 'determining' }]),
    );
  });

  it('ContextEngine attaches meta on create/setCore/setState', async () => {
    const bus = new InMemoryEventBus();
    const engine = new ContextEngine(undefined as any, bus);

    const [created] = await engine.handle({
      kind: 'context.create',
      payload: { id: 'context:1', type: 'system.Context', name: 'Test' },
    });

    expect(created.kind).toBe('context.create');
    expect((created.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'assert',
      kind: 'Context',
      ids: ['context:1'],
    });
    expect((created.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'context', rule: 'identity' }]),
    );

    const [setCore] = await engine.handle({
      kind: 'context.setCore',
      payload: { id: 'context:1', name: 'Test2' },
    });
    expect((setCore.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'revise',
      kind: 'Context',
      ids: ['context:1'],
    });
    expect((setCore.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'context', rule: 'difference' }]),
    );

    const [setState] = await engine.handle({
      kind: 'context.setState',
      payload: { id: 'context:1', state: { status: 'active' } },
    });
    expect((setState.meta as any)?.factStore).toMatchObject({
      mode: 'logic',
      store: 'FormDB',
      op: 'revise',
      kind: 'Context',
      ids: ['context:1'],
    });
    expect((setState.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'context', rule: 'contradiction' }]),
    );
  });

  it('MorphEngine attaches meta on create/setCore/setState', async () => {
    const bus = new InMemoryEventBus();
    const repo = new InMemoryRepository(MorphSchema);
    const engine = new MorphEngine(repo as any, bus);

    const [created] = await engine.handle({
      kind: 'morph.create',
      payload: { id: 'morph:1', type: 'system.Morph', name: 'Ground' },
    });

    expect(created.kind).toBe('morph.create');
    expect((created.meta as any)?.factStore).toMatchObject({
      mode: 'transcendental',
      store: 'FormDB',
      op: 'assert',
      kind: 'Morph',
      ids: ['morph:1'],
    });
    expect((created.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'morph', rule: 'ground' }]),
    );

    const [setCore] = await engine.handle({
      kind: 'morph.setCore',
      payload: { id: 'morph:1', name: 'Ground2' },
    });
    expect((setCore.meta as any)?.factStore).toMatchObject({
      mode: 'transcendental',
      store: 'FormDB',
      op: 'revise',
      kind: 'Morph',
      ids: ['morph:1'],
    });
    expect((setCore.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'morph', rule: 'condition' }]),
    );

    const [setState] = await engine.handle({
      kind: 'morph.setState',
      payload: { id: 'morph:1', state: { status: 'active' } },
    });
    expect((setState.meta as any)?.factStore).toMatchObject({
      mode: 'transcendental',
      store: 'FormDB',
      op: 'revise',
      kind: 'Morph',
      ids: ['morph:1'],
    });
    expect((setState.meta as any)?.dialectic?.tags).toEqual(
      expect.arrayContaining([{ layer: 'morph', rule: 'facticity' }]),
    );
  });
});
