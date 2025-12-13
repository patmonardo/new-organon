import { describe, expect, it } from 'vitest';

import { evaluateAgentPolicy } from '../src/runtime/agent-policy.js';
import { AgentPolicySchema, AgentActionSchema } from '../src/schema/agent-control.js';

describe('agent policy evaluation (runtime helper)', () => {
  it('denies when denylist matches', () => {
    const policy = AgentPolicySchema.parse({
      id: 'p1',
      deny: { gdsOps: ['gds.graph_store_catalog.drop_*'] },
    });

    const action = AgentActionSchema.parse({
      id: 'a1',
      kind: 'gds.call',
      operationId: 'gds.graph_store_catalog.drop_graph',
    });

    const decision = evaluateAgentPolicy(policy, action);
    expect(decision.decision).toBe('deny');
  });

  it('requires approval when requireApproval matches', () => {
    const policy = AgentPolicySchema.parse({
      id: 'p1',
      requireApproval: { toolOps: ['tool.delete*'] },
    });

    const action = AgentActionSchema.parse({
      id: 'a1',
      kind: 'tool.call',
      operationId: 'tool.deleteAll',
    });

    const decision = evaluateAgentPolicy(policy, action);
    expect(decision.decision).toBe('require-approval');
  });

  it('enforces allowlist when present', () => {
    const policy = AgentPolicySchema.parse({
      id: 'p1',
      allow: { mcpOps: ['mcp.fs.read*'] },
    });

    const allowed = AgentActionSchema.parse({
      id: 'a-ok',
      kind: 'mcp.call',
      operationId: 'mcp.fs.readFile',
    });

    const denied = AgentActionSchema.parse({
      id: 'a-no',
      kind: 'mcp.call',
      operationId: 'mcp.fs.writeFile',
    });

    expect(evaluateAgentPolicy(policy, allowed).decision).toBe('allow');
    expect(evaluateAgentPolicy(policy, denied).decision).toBe('deny');
  });
});
