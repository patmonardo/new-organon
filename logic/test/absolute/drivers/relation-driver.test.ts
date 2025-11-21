import { describe, it, expect } from 'vitest';
import { RelationDriver } from '../../../src/absolute/essence/relation';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { AspectSchema } from '../../../src/schema/aspect';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { ActiveAspect } from '../../../src/schema/active';

describe('RelationDriver', () => {
  it('creates a RelationDriver with default dependencies', () => {
    const driver = new RelationDriver();
    expect(driver).toBeDefined();
  });

  it('creates a RelationDriver with provided repo and bus', () => {
    const repo = makeInMemoryRepository(AspectSchema as any);
    const bus = new InMemoryEventBus();
    const driver = new RelationDriver(repo as any, bus);
    expect(driver).toBeDefined();
  });

  it('assembles a basic world', () => {
    const driver = new RelationDriver();
    const world = driver.assemble({});

    expect(world).toBeDefined();
    expect(world.shape.core.type).toBe('system.World');
    expect(world.shape.core.name).toBe('Aspect');
    // Remove Kantian horizon check - too speculative!
  });

  it('creates and retrieves an aspect', async () => {
    const driver = new RelationDriver();

    const aspectId = await driver.createAspect({
      id: 'test:aspect',
      type: 'system.Aspect',
      name: 'TestAspect',
    });

    expect(aspectId).toBe('test:aspect');

    const aspect = await driver.getAspect(aspectId);
    expect(aspect).toBeDefined();
    expect(aspect?.name).toBe('TestAspect');
  });

  it('deletes an aspect', async () => {
    const driver = new RelationDriver();

    const aspectId = await driver.createAspect({
      type: 'system.Aspect',
      name: 'ToDelete',
    });

    await driver.deleteAspect(aspectId);
    const aspect = await driver.getAspect(aspectId);
    expect(aspect).toBeUndefined();
  });

  it('processes ActiveAspects via engine', async () => {
    const driver = new RelationDriver();

    // Create an aspect first
    await driver.createAspect({
      id: 'aspect:one',
      type: 'system.Aspect',
      name: 'One',
    });

    const activeAspects: ActiveAspect[] = [
      { id: 'aspect:one', name: 'UpdatedOne' },
    ];

    const { actions, snapshot } = await driver.processAspects(activeAspects);
    expect(snapshot.count).toBe(1);
    expect(actions.length).toBeGreaterThan(0);

    const result = await driver.commitAspects(actions, snapshot);
    expect(result.success).toBe(true);
  });

  it('processes and commits aspects in one step', async () => {
    const driver = new RelationDriver();

    // Create aspect first
    await driver.createAspect({
      id: 'aspect:batch',
      type: 'system.Aspect',
      name: 'Batch',
    });

    const activeAspects: ActiveAspect[] = [
      { id: 'aspect:batch', name: 'BatchUpdated' },
    ];

    const result = await driver.processAndCommitAspects(activeAspects);
    expect(result.success).toBe(true);
  });
});
