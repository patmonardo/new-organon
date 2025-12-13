/**
 * Agent Control Protocol (Schema-first)
 * ===================================
 *
 * This is the low-level, transport-agnostic protocol used to control *agent actions*.
 *
 * Design intent:
 * - Canonical, minimal schema surface (Zod) that multiple runtimes can implement.
 * - Works with the TAW event vocabulary (intent/plan/act/result) but is not coupled
 *   to any particular execution substrate.
 * - Expresses “what an agent is allowed to do” (policy) and “what it is trying to do” (action).
 */

import { z } from 'zod';

export const AgentIdSchema = z.string().min(1);
export type AgentId = z.infer<typeof AgentIdSchema>;

/**
 * AgentKind is intentionally broad: it describes the *runtime adapter* boundary.
 *
 * Examples:
 * - `llm`: a chat/completions model runtime
 * - `mcp`: an MCP client runtime
 * - `genkit`: a Genkit runtime
 * - `gds`: a GDS application facade client
 */
export const AgentKindSchema = z.enum([
  'llm',
  'mcp',
  'genkit',
  'gds',
  'tool',
  'human',
  'system',
  'composite',
]);
export type AgentKind = z.infer<typeof AgentKindSchema>;

/**
 * A structural descriptor of an action an agent intends to take.
 *
 * This is used for:
 * - policy evaluation (allow/deny/approve)
 * - logging/audit
 * - gating sensitive operations
 */
export const AgentActionSchema = z
  .object({
    id: z.string().min(1),

    /**
     * High-level action class.
     *
     * - `gds.call`: invoke a GDS Application Form operation (e.g. `gds.graph_store_catalog.list_graphs`)
     * - `mcp.call`: invoke an MCP tool
     * - `tool.call`: invoke a local tool/function
     * - `llm.request`: call an LLM runtime
     */
    kind: z.enum(['gds.call', 'mcp.call', 'tool.call', 'llm.request']),

    /**
     * Operation identifier (when applicable).
     *
     * For GDS: `gds.<facade>.<op>`.
     * For MCP/tools: implementation-defined (e.g. `mcp.<server>.<tool>`).
     */
    operationId: z.string().min(1).optional(),

    /** Human-readable label (optional). */
    label: z.string().optional(),

    /** Arbitrary input payload (validated by the target system, not by this protocol). */
    input: z.unknown().optional(),

    /**
     * Correlation identifiers so actions can be linked to TAW envelopes.
     *
     * If used with TAW, `correlationId` should typically match the workflow/run correlation.
     */
    correlationId: z.string().min(1).optional(),

    /** Optional metadata for audit/policy. */
    meta: z.record(z.string(), z.unknown()).optional(),
  })
  .catchall(z.unknown());

export type AgentAction = z.infer<typeof AgentActionSchema>;

/**
 * Policy rules are “repository discipline” for action space.
 *
 * Everything is expressed as string patterns for now.
 * Runtimes may interpret these as exact matches, prefixes, glob-like patterns, etc.
 */
export const AgentPolicySchema = z
  .object({
    id: z.string().min(1),
    version: z.string().min(1).optional(),

    /** Allowlist patterns by action kind. */
    allow: z
      .object({
        gdsOps: z.array(z.string().min(1)).optional(),
        mcpOps: z.array(z.string().min(1)).optional(),
        toolOps: z.array(z.string().min(1)).optional(),
        llmOps: z.array(z.string().min(1)).optional(),
      })
      .optional(),

    /** Denylist patterns by action kind (evaluated before allow). */
    deny: z
      .object({
        gdsOps: z.array(z.string().min(1)).optional(),
        mcpOps: z.array(z.string().min(1)).optional(),
        toolOps: z.array(z.string().min(1)).optional(),
        llmOps: z.array(z.string().min(1)).optional(),
      })
      .optional(),

    /**
     * Approval-gated patterns.
     *
     * If an action matches these patterns, a runtime should require an explicit approval.
     */
    requireApproval: z
      .object({
        gdsOps: z.array(z.string().min(1)).optional(),
        mcpOps: z.array(z.string().min(1)).optional(),
        toolOps: z.array(z.string().min(1)).optional(),
        llmOps: z.array(z.string().min(1)).optional(),
      })
      .optional(),

    /** Optional global guardrails. */
    limits: z
      .object({
        maxActionsPerRun: z.number().int().positive().optional(),
        maxConcurrency: z.number().int().positive().optional(),
      })
      .optional(),

    note: z.string().optional(),
  })
  .catchall(z.unknown());

export type AgentPolicy = z.infer<typeof AgentPolicySchema>;

export const AgentPolicyDecisionSchema = z.object({
  policyId: z.string().min(1),
  actionId: z.string().min(1),
  decision: z.enum(['allow', 'deny', 'require-approval']),
  reason: z.string().optional(),
});
export type AgentPolicyDecision = z.infer<typeof AgentPolicyDecisionSchema>;

/**
 * Control commands: how an orchestrator (or human operator) controls an agent.
 */
export const AgentControlCommandSchema = z.discriminatedUnion('kind', [
  z.object({
    kind: z.literal('agent.policy.set'),
    payload: z.object({ agentId: AgentIdSchema, policy: AgentPolicySchema }),
  }),
  z.object({
    kind: z.literal('agent.action.approve'),
    payload: z.object({ agentId: AgentIdSchema, actionId: z.string().min(1) }),
  }),
  z.object({
    kind: z.literal('agent.action.deny'),
    payload: z.object({ agentId: AgentIdSchema, actionId: z.string().min(1), reason: z.string().optional() }),
  }),
  z.object({
    kind: z.literal('agent.pause'),
    payload: z.object({ agentId: AgentIdSchema, reason: z.string().optional() }),
  }),
  z.object({
    kind: z.literal('agent.resume'),
    payload: z.object({ agentId: AgentIdSchema }),
  }),
  z.object({
    kind: z.literal('agent.stop'),
    payload: z.object({ agentId: AgentIdSchema, reason: z.string().optional() }),
  }),
]);

export type AgentControlCommand = z.infer<typeof AgentControlCommandSchema>;
