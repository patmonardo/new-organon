import { BaseEngine } from '../base-engine';
import type { EmpowermentLike } from '../empowerment-core';

/**
 * AgentEngine - an engine representing a single Agent's capabilities.
 * - stores a static set of empowerments (capabilities) for the agent
 * - stronger signature requirement by default (agents sign with 'agent' issuer)
 * - used in proto pipeline to represent the endpoint that actually performs work
 */
export class AgentEngine extends BaseEngine {
  name: string;
  private readonly agentId: string;
  private readonly staticEmpowerments: EmpowermentLike[];

  constructor(
    agentId: string,
    empowerments: EmpowermentLike[] = [],
    opts?: { scope?: string },
  ) {
    super({ scope: opts?.scope ?? `agent:${agentId}` });
    this.agentId = agentId;
    this.name = `agent-engine:${agentId}`;
    // tag empowerments with provider and default subject
    this.staticEmpowerments = (empowerments || []).map((e) => ({
      ...e,
      subject: e.subject ?? agentId,
      provider: this.name,
    }));
  }

  // Return agent-specific empowerments. In a real engine this would query a store.
  async fetchEmpowerments(
    subject: string,
    _opts?: any,
  ): Promise<EmpowermentLike[]> {
    // preserve provider tag and normalize subject for the caller
    return this.staticEmpowerments.map((e) => ({
      ...e,
      subject: e.subject ?? subject,
      provider: this.name,
    }));
  }

  // Require signatures issued by this agent (simple rule for prototype)
  protected async verifySignatures(sigs?: any[]): Promise<boolean> {
    if (!sigs || sigs.length === 0) return false; // agent empowerments must be signed
    return sigs.every(
      (s) =>
        typeof s.signature === 'string' &&
        s.signature.length > 0 &&
        (s.issuer === this.agentId || (s.issuer ?? '').startsWith('agent')),
    );
  }

  // Agent-specific facet checks: allow facets that mention the agent or resource match
  checkFacets(
    facets: Array<any>,
    ctx: { action?: string; resource?: string } = {},
  ) {
    if (!facets || facets.length === 0) return false; // require facets for agent-level capabilities
    const action = ctx.action;
    const resource = ctx.resource;
    for (const f of facets) {
      if (typeof f === 'string') {
        if (f === '*' || f === action || f === resource || f === this.agentId)
          return true;
        continue;
      }
      const scope = Array.isArray(f.scope) ? f.scope : [];
      if (scope.length === 0) return true;
      for (const s of scope) {
        if (s === '*' || (resource && s === resource)) return true;
        if (resource && typeof s === 'string' && s.endsWith('*')) {
          const prefix = s.slice(0, -1);
          if (resource.startsWith(prefix)) return true;
        }
      }
    }
    return false;
  }
}
