import type {
  EmpowermentLike,
  FacetLike,
  SignatureLike,
  EmpowermentProvider,
} from './empowerment-core';
import type { EmpowermentCombineResult } from './empowerment-processor';

/**
 * ProcessorControl — pure witnessing layer for the Being:Essence:Concept triad.
 * - Stateless/pure: does not mutate provider state; provider performs I/O and caching.
 * - Engines implement EmpowermentProvider to supply concrete behavior.
 */
export abstract class ProcessorControl {
  // provider is readonly to emphasize purity of the control layer
  protected readonly provider: EmpowermentProvider;

  constructor(provider: EmpowermentProvider) {
    this.provider = provider;
  }

  // Verify signatures (Being). Pure: delegates to provider but does not alter state.
  async verifyBeing(emp: EmpowermentLike): Promise<boolean> {
    return this.provider.validateSignatures(emp);
  }

  // Evaluate facet constraints (Essence). Pure predicate — engines implement checkFacets on provider.
  async evaluateEssence(
    emp: EmpowermentLike,
    context?: any,
    action?: string,
    resource?: string,
  ): Promise<boolean> {
    if (!emp.facets || emp.facets.length === 0) return true;
    if (typeof (this.provider as any).checkFacets === 'function') {
      return (this.provider as any).checkFacets(emp.facets, {
        context,
        action,
        resource,
      });
    }
    return true;
  }

  // Combine empowerments (Concept). Pure combinator; provider may override to supply semantics.
  combineConcept(
    empowerments: EmpowermentLike[],
    root?: EmpowermentLike,
    opts?: { privilegeBoost?: number },
  ): EmpowermentCombineResult {
    if (typeof (this.provider as any).combine === 'function') {
      return (this.provider as any).combine(empowerments, root, opts);
    }
    const scores = (empowerments || []).map((e) => {
      const w = typeof e.weight === 'number' ? e.weight : 0;
      const c = typeof e.certainty === 'number' ? e.certainty : 1;
      return {
        id: e.id ?? 'unknown',
        subject: e.subject ?? 'unknown',
        score: w * c,
      };
    });
    if (root && !scores.some((s) => s.id === root.id)) {
      const rw = typeof root.weight === 'number' ? root.weight : 0;
      const rc = typeof root.certainty === 'number' ? root.certainty : 1;
      scores.push({
        id: root.id ?? 'root',
        subject: root.subject ?? 'root',
        score: rw * rc,
      });
    }
    const total = scores.reduce((acc, it) => acc + it.score, 0);
    return { total, scores };
  }

  // High-level authorize API: pure orchestration of provider I/O and pure predicates/combinator.
  async authorize(
    subject: string,
    action: string,
    resource?: string,
    opts?: { privilegeBoost?: number },
  ): Promise<EmpowermentCombineResult> {
    const emps = await this.provider.fetchEmpowerments(subject, {
      action,
      resource,
    });
    const applicable: EmpowermentLike[] = [];
    for (const e of emps) {
      const okBeing = await this.verifyBeing(e);
      if (!okBeing) continue;
      const okEssence = await this.evaluateEssence(
        e,
        { subject, resource },
        action,
        resource,
      );
      if (!okEssence) continue;
      applicable.push(e);
    }
    const root = (this.provider as any).rootEmpowerment ?? undefined;
    return this.combineConcept(applicable, root, opts);
  }
}

export type { EmpowermentLike, FacetLike, SignatureLike, EmpowermentProvider };
