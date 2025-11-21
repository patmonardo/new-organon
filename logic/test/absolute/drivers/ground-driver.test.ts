import { describe, it, expect } from 'vitest';
import { GroundDriver } from '../../../src/absolute/essence/ground';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { MorphSchema } from '../../../src/schema/morph';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { ActiveMorph } from '../../../src/schema/active';

describe('GroundDriver', () => {
  it('creates a GroundDriver with default dependencies', () => {
    const driver = new GroundDriver();
    expect(driver).toBeDefined();
  });

  it('creates a GroundDriver with provided repo and bus', () => {
    const repo = makeInMemoryRepository(MorphSchema as any);
    const bus = new InMemoryEventBus();
    const driver = new GroundDriver(repo as any, bus);
    expect(driver).toBeDefined();
  });

  it('assembles a basic world', () => {
    const driver = new GroundDriver();
    const world = driver.assemble({});
    expect(world).toBeDefined();
    expect(world.shape.core.type).toBe('system.World');
    expect(world.shape.core.name).toBe('Ground');
    // No Kantian horizon checks!
  });

  it('creates and retrieves a morph', async () => {
    const driver = new GroundDriver();
    const morphId = await driver.createMorph({
      id: 'test:morph',
      type: 'system.Morph',
      name: 'TestMorph',
    });
    expect(morphId).toBe('test:morph');
    const morph = await driver.getMorph(morphId);
    expect(morph).toBeDefined();
    expect(morph?.name).toBe('TestMorph');
  });

  it('deletes a morph', async () => {
    const driver = new GroundDriver();
    const morphId = await driver.createMorph({
      type: 'system.Morph',
      name: 'ToDelete',
    });
    await driver.deleteMorph(morphId);
    const morph = await driver.getMorph(morphId);
    expect(morph).toBeUndefined();
  });

  it('processes ActiveMorphs via engine', async () => {
    const driver = new GroundDriver();
    await driver.createMorph({
      id: 'morph:one',
      type: 'system.Morph',
      name: 'One',
    });
    const activeMorphs: ActiveMorph[] = [
      { id: 'morph:one', transform: 'noop' /* other fields as needed */ },
    ];
    const { actions, snapshot } = await driver.processMorphs(activeMorphs);
    expect(snapshot.count).toBe(1);
    expect(actions.length).toBeGreaterThan(0);
    const result = await driver.commitMorphs(actions, snapshot);
    expect(result.success).toBe(true);
  });

  it('processes and commits morphs in one step', async () => {
    const driver = new GroundDriver();
    await driver.createMorph({
      id: 'morph:batch',
      type: 'system.Morph',
      name: 'Batch',
    });
    const activeMorphs: ActiveMorph[] = [
      { id: 'morph:one', transform: 'noop' /* other fields as needed */ },
    ];
    const result = await driver.processAndCommitMorphs(activeMorphs);
    expect(result.success).toBe(true);
  });

  it('handles revoked morphs (deletion via ActiveMorph)', async () => {
    const driver = new GroundDriver();
    await driver.createMorph({
      id: 'morph:revoke-test',
      type: 'system.Morph',
      name: 'ToRevoke',
    });
    const activeMorphs: ActiveMorph[] = [
      { id: 'morph:revoke-test', transform: 'noop', revoked: true },
    ];
    const result = await driver.processAndCommitMorphs(activeMorphs);
    expect(result.success).toBe(true);
    const morph = await driver.getMorph('morph:revoke-test');
    expect(morph).toBeUndefined();
  });
});
