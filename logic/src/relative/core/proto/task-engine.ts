import { BaseEngine } from '../base-engine';
import type { EmpowermentLike } from '../empowerment-core';

/**
 * TaskEngine - represents Task-level empowerments (Goal → Method).
 * Lightweight prototype: static empowerments list, slightly stricter facet check for tasks.
 */
export class TaskEngine extends BaseEngine {
  name: string;
  private readonly staticEmpowerments: EmpowermentLike[] = [];

  constructor(name = 'task-engine', empowerments: EmpowermentLike[] = []) {
    super({ scope: name });
    this.name = name;
    this.staticEmpowerments = (empowerments || []).map((e) => ({
      ...e,
      provider: name,
    }));
  }

  async fetchEmpowerments(
    subject: string,
    _opts?: any,
  ): Promise<EmpowermentLike[]> {
    return this.staticEmpowerments.map((e) => ({
      ...e,
      subject: e.subject ?? subject,
      provider: this.name,
    }));
  }

  // Task empowerments should carry a signature (policy or task system)
  protected async verifySignatures(sigs?: any[]): Promise<boolean> {
    if (!sigs || sigs.length === 0) return false;
    // accept signatures issued by 'task' or 'policy' or any non-empty signature for prototype
    return sigs.every(
      (s) => typeof s.signature === 'string' && s.signature.length > 0,
    );
  }

  // Facet check: require facet to include the task resource or action
  checkFacets(
    facets: Array<any>,
    ctx: { action?: string; resource?: string } = {},
  ) {
    if (!facets || facets.length === 0) return false;
    for (const f of facets) {
      if (typeof f === 'string') {
        if (f === '*' || f === ctx.action || f === ctx.resource) return true;
        continue;
      }
      const scope = Array.isArray((f as any).scope) ? (f as any).scope : [];
      if (scope.length === 0) return true;
      if (ctx.resource && scope.includes(ctx.resource)) return true;
    }
    return false;
  }
}
