import { describe, expect, it } from 'vitest';
import { WorkflowUnitySchema } from '../src/schema/workflow.js';

describe('WorkflowUnitySchema (W = Concept + Control Surface)', () => {
  it('parses a minimal workflow unity', () => {
    const wf = WorkflowUnitySchema.parse({
      id: 'wf1',
      concept: {
        goal: { id: 'g1', type: 'seed', description: 'Seed dialectical cube' },
      },
      controller: {
        goalId: 'g1',
        steps: [{ id: 's1', description: 'Commit', action: 'graph.commit' }],
      },
    });

    expect(wf.concept.goal.id).toBe('g1');
    expect(wf.controller.steps[0].action).toBe('graph.commit');
  });
});
