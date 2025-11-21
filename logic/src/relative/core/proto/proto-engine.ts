import { BaseEngine } from '../base-engine';
import type { EmpowermentLike } from '../empowerment-core';

/**
 * ProtoEngine - simple, configurable engine for prototyping.
 * Provide a static list of empowerments; fetchEmpowerments returns them (subject defaulted).
 */
export class ProtoEngine extends BaseEngine {
  name: string;
  private _static: EmpowermentLike[] = [];

  constructor(name: string, empowerments: EmpowermentLike[] = []) {
    super({ scope: name });
    this.name = name;
    // tag and seed the base engine graph so the engine graph is canonical
    this._static = (empowerments || []).map((e) => ({ ...e, provider: name }));
    this.bulkAdd(this._static);
  }

  // Delegate to BaseEngine.fetchEmpowerments which queries the in-memory graph.
  async fetchEmpowerments(subject: string, _opts?: any): Promise<EmpowermentLike[]> {
    return super.fetchEmpowerments(subject, _opts);
  }

  // Simple prototype signature verifier (engines may override)
  protected async verifySignatures(sigs?: any[]): Promise<boolean> {
    if (!sigs || sigs.length === 0) return false;
    return sigs.every((s) => typeof s?.signature === 'string' && s.signature.length > 0);
  }

  // Basic facet check used by proto engines
  checkFacets(facets?: Array<any>, ctx: { action?: string; resource?: string } = {}): boolean {
    if (!facets || facets.length === 0) return false;
    const { action, resource } = ctx;
    for (const f of facets) {
      if (typeof f === 'string') {
        if (f === '*' || f === action || f === resource) return true;
        continue;
      }
      const scope = Array.isArray((f as any).scope) ? (f as any).scope : [];
      if (scope.length === 0) return true;
      if (resource && scope.includes(resource)) return true;
      if (action && scope.includes(action)) return true;
    }
    return false;
  }
}
