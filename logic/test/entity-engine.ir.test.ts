import { EntityEngine } from '../src/relative/form/entity/entity-engine';
import { FormEntity } from '../src/relative/form/entity/entity-form';
import { makeInMemoryRepository } from '../src/repository';
import type { DialecticState } from '@schema';

import { EntitySchema } from '@schema';

describe('EntityEngine IR Integration', () => {
  let engine: EntityEngine;

  beforeEach(() => {
    const repo = makeInMemoryRepository(EntitySchema);
    engine = new EntityEngine(repo);
  });

  it('should handle dialectic.evaluate command', async () => {
    const dialecticState: DialecticState = {
      id: 'test-entity-state-1',
      title: 'Test Entity State',
      concept: 'TestEntityConcept',
      phase: 'appearance',
      moments: [
        { name: 'momentA', definition: 'defA', type: 'quality' }
      ],
      invariants: [],
      provenance: {
        topicMapId: 'tm2',
        lineRange: { start: 1, end: 10 }
      }
    };

    const events = await engine.handle({
      kind: 'dialectic.evaluate',
      payload: { dialecticState }
    });

    expect(events).toHaveLength(1);
    expect(events[0].kind).toBe('dialectic.evaluated');
    expect(events[0].payload).toEqual({
      stateId: 'test-entity-state-1',
      concept: 'TestEntityConcept',
      phase: 'appearance'
    });

    const entity = await engine.getEntity('test-entity-state-1');
    expect(entity).toBeDefined();
    expect(entity?.getDialecticState()).toEqual(dialecticState);
    expect(entity?.getMoments()).toHaveLength(1);
    expect(entity?.getMoments()[0].name).toBe('momentA');
    expect(entity?.type).toBe('TestEntityConcept');
  });
});
