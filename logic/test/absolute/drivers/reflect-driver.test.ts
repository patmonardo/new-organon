import { describe, it, expect, vi } from 'vitest';
import { ReflectDriver } from '../../../src/absolute/essence/reflect';
import { makeInMemoryRepository } from '../../../src/repository/memory';
import { ContextSchema } from '../../../src/schema/context';
import { InMemoryEventBus } from '../../../src/absolute/core/bus';
import { ActiveContext } from '../../../src/schema/active';

describe('ReflectDriver', () => {
  it('processes and commits ActiveContexts via ContextEngine', async () => {
    const repo = makeInMemoryRepository(ContextSchema as any);
    const driver = new ReflectDriver(repo as any);

    // Create context first
    const contextId = await driver.createContext({
      id: 'context:one',
      type: 'system.Context',
      name: 'One',
    });
    expect(contextId).toBe('context:one');

    // Process ActiveContext updates
    const contexts: ActiveContext[] = [
      { id: 'context:one', name: 'Updated One' }
    ];

    const { actions, snapshot } = await driver.processContexts(contexts);
    expect(snapshot.count).toBe(1);
    expect(actions.length).toBeGreaterThan(0);

    // Commit the changes
    const result = await driver.commitContexts(actions, snapshot);
    expect(result.success).toBe(true);

    // Verify the context was updated
    const context = await driver.getContext('context:one');
    expect(context).toBeTruthy();
    expect(context?.name).toBe('Updated One');
  });

  it('handles context creation and deletion', async () => {
    const repo = makeInMemoryRepository(ContextSchema as any);
    const driver = new ReflectDriver(repo as any);

    // Create
    const id = await driver.createContext({
      type: 'system.Context',
      name: 'TestContext'
    });
    expect(id).toBeTruthy();

    // Verify exists
    const context = await driver.getContext(id);
    expect(context?.name).toBe('TestContext');

    // Delete
    await driver.deleteContext(id);

    // Verify gone
    const deletedContext = await driver.getContext(id);
    expect(deletedContext).toBeUndefined();
  });

  it('emits events through event bus', async () => {
    const bus = new InMemoryEventBus();
    const repo = makeInMemoryRepository(ContextSchema as any);
    const driver = new ReflectDriver(repo as any, bus);

    const receivedEvents: any[] = [];
    bus.subscribe('context.create', (evt) => receivedEvents.push(evt));

    // Create context should emit event
    await driver.createContext({
      type: 'system.Context',
      name: 'EventTest'
    });

    expect(receivedEvents.length).toBe(1);
    expect(receivedEvents[0].kind).toBe('context.create');
  });

  it('processes and commits in one step', async () => {
    const repo = makeInMemoryRepository(ContextSchema as any);
    const driver = new ReflectDriver(repo as any);

    // Create initial context
    await driver.createContext({
      id: 'context:batch',
      type: 'system.Context',
      name: 'Batch'
    });

    // Process and commit in one step
    const contexts: ActiveContext[] = [
      { id: 'context:batch', name: 'BatchUpdated' }
    ];

    const result = await driver.processAndCommitContexts(contexts);
    expect(result.success).toBe(true);

    // Verify update
    const context = await driver.getContext('context:batch');
    expect(context?.name).toBe('BatchUpdated');
  });

  it('handles revoked contexts (deletion via ActiveContext)', async () => {
    const repo = makeInMemoryRepository(ContextSchema as any);
    const driver = new ReflectDriver(repo as any);

    // Create context first
    await driver.createContext({
      id: 'context:revoke-test',
      type: 'system.Context',
      name: 'ToRevoke'
    });

    // Process revoked context
    const contexts: ActiveContext[] = [
      { id: 'context:revoke-test', revoked: true }
    ];

    const result = await driver.processAndCommitContexts(contexts);
    expect(result.success).toBe(true);

    // Verify context is gone
    const context = await driver.getContext('context:revoke-test');
    expect(context).toBeUndefined();
  });
});
