import { describe, expect, it } from 'vitest';

import {
  AgentActionSchema,
  AgentControlCommandSchema,
  AgentPolicySchema,
} from '../src/schema/agent-control.js';

describe('Agent control protocol (schema-first)', () => {
  it('parses an action descriptor', () => {
    const action = AgentActionSchema.parse({
      id: 'a1',
      kind: 'gds.call',
      operationId: 'gds.graph_store_catalog.list_graphs',
      input: { facade: 'graph_store_catalog', op: 'list_graphs' },
      correlationId: 'c1',
    });

    expect(action.kind).toBe('gds.call');
    expect(action.operationId).toContain('gds.');
  });

  it('parses a policy and a control command', () => {
    const policy = AgentPolicySchema.parse({
      id: 'p1',
      version: '1',
      allow: { gdsOps: ['gds.graph_store_catalog.*'] },
      requireApproval: { gdsOps: ['gds.graph_store_catalog.drop_*'] },
      limits: { maxActionsPerRun: 50 },
    });

    const cmd = AgentControlCommandSchema.parse({
      kind: 'agent.policy.set',
      payload: { agentId: 'agent-1', policy },
    });

    expect(cmd.kind).toBe('agent.policy.set');
    if (cmd.kind !== 'agent.policy.set') {
      throw new Error('Expected agent.policy.set');
    }
    expect(cmd.payload.policy.id).toBe('p1');
  });
});
