/**
 * Agent Policy Evaluation (runtime helper)
 * =======================================
 *
 * Pure functions that evaluate an AgentPolicy against an AgentAction.
 *
 * This lives outside `src/schema/**` on purpose:
 * - schema remains “defines only” (no methods)
 * - runtime helpers can evolve without changing the canonical schema surface
 */

import type { AgentAction, AgentPolicy, AgentPolicyDecision } from '../schema/agent-control';

export type AgentPolicyEvalOptions = {
  /** If true, absence of an allowlist is treated as deny-by-default. */
  denyByDefault?: boolean;
};

const escapeRegex = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

/**
 * Very small glob matcher:
 * - `*` matches any substring
 * - everything else is treated literally
 */
export function matchesPattern(value: string, pattern: string): boolean {
  if (!pattern) return false;
  if (pattern === '*') return true;

  const parts = pattern.split('*').map(escapeRegex);
  const regex = new RegExp(`^${parts.join('.*')}$`);
  return regex.test(value);
}

function anyMatch(value: string, patterns: readonly string[] | undefined): boolean {
  if (!patterns || patterns.length === 0) return false;
  return patterns.some((p) => matchesPattern(value, p));
}

function patternsForActionKind(policy: AgentPolicy, action: AgentAction) {
  if (action.kind === 'gds.call') {
    return {
      allow: policy.allow?.gdsOps,
      deny: policy.deny?.gdsOps,
      requireApproval: policy.requireApproval?.gdsOps,
    };
  }
  if (action.kind === 'mcp.call') {
    return {
      allow: policy.allow?.mcpOps,
      deny: policy.deny?.mcpOps,
      requireApproval: policy.requireApproval?.mcpOps,
    };
  }
  if (action.kind === 'tool.call') {
    return {
      allow: policy.allow?.toolOps,
      deny: policy.deny?.toolOps,
      requireApproval: policy.requireApproval?.toolOps,
    };
  }
  return {
    allow: policy.allow?.llmOps,
    deny: policy.deny?.llmOps,
    requireApproval: policy.requireApproval?.llmOps,
  };
}

/**
 * Evaluates policy → decision.
 *
 * Order:
 * 1) deny
 * 2) require-approval
 * 3) allow
 * 4) default (allow unless denyByDefault)
 */
export function evaluateAgentPolicy(
  policy: AgentPolicy,
  action: AgentAction,
  opts: AgentPolicyEvalOptions = {},
): AgentPolicyDecision {
  const op = action.operationId ?? '';
  const { allow, deny, requireApproval } = patternsForActionKind(policy, action);

  if (anyMatch(op, deny)) {
    return { policyId: policy.id, actionId: action.id, decision: 'deny', reason: 'Denied by policy' };
  }

  if (anyMatch(op, requireApproval)) {
    return {
      policyId: policy.id,
      actionId: action.id,
      decision: 'require-approval',
      reason: 'Policy requires approval',
    };
  }

  // If there is an allowlist for this kind, enforce it.
  if (allow && allow.length > 0) {
    if (anyMatch(op, allow)) {
      return { policyId: policy.id, actionId: action.id, decision: 'allow', reason: 'Allowed by policy' };
    }
    return { policyId: policy.id, actionId: action.id, decision: 'deny', reason: 'Not on allowlist' };
  }

  // No allowlist provided.
  if (opts.denyByDefault) {
    return { policyId: policy.id, actionId: action.id, decision: 'deny', reason: 'Deny-by-default' };
  }

  return { policyId: policy.id, actionId: action.id, decision: 'allow', reason: 'No allowlist; allowed' };
}
