import type { Empowerment } from '@schema';
import type { Facet } from '@schema';
import type { Signature } from '@schema';

// Replace the problematic interface extension with a composed type that
// allows facets to be either Facet objects or string ids without conflicting
// with the canonical Empowerment schema type.
export type EmpowermentLike = Partial<Omit<Empowerment, 'facets' | 'signatures'>> & {
  id: string;
  subject: string;
  weight?: number;
  certainty?: number;
  actions?: string[];
  facets?: Array<Facet | string>;
  signatures?: Signature[];
  // origin provider/engine that produced this empowerment (provenance tag)
  provider?: string;
};

export interface FacetLike extends Partial<Facet> {
  id: string;
  name?: string;
}

export interface SignatureLike extends Partial<Signature> {
  id: string;
  issuer?: string;
  signature?: string;
}

/**
 * Core contract for Engines to implement.
 * Keep the Processor logic small and engine-agnostic; engines provide crypto/resolution.
 */
export abstract class EmpowermentProvider {
  abstract name: string;

  // Resolve available empowerments for a subject/context
  abstract fetchEmpowerments(subject: string, opts?: any): Promise<EmpowermentLike[]>;

  // Validate signatures for an empowerment (default accepts no signatures)
  async validateSignatures(emp: EmpowermentLike): Promise<boolean> {
    if (!emp.signatures || emp.signatures.length === 0) return true;
    return this.verifySignatures(emp.signatures as SignatureLike[]);
  }

  // Engines override with real verification
  protected async verifySignatures(sigs?: SignatureLike[]): Promise<boolean> {
    console.log("Verifying signatures:", sigs);
    return true;
  }

  // Core combine routine; engines may override for richer semantics
  combine(empowerments: EmpowermentLike[], root?: EmpowermentLike, opts?: { privilegeBoost?: number }) {
    const boost = typeof opts?.privilegeBoost === 'number' ? opts!.privilegeBoost! : 1;
    const scores = (empowerments || []).map(e => {
      const w = typeof e.weight === 'number' ? e.weight : 0;
      const c = typeof e.certainty === 'number' ? e.certainty : 1;
      return { id: e.id ?? 'unknown', subject: e.subject ?? 'unknown', score: w * c * boost };
    });

    if (root && !scores.some(s => s.id === root.id)) {
      const rw = typeof root.weight === 'number' ? root.weight : 0;
      const rc = typeof root.certainty === 'number' ? root.certainty : 1;
      scores.push({ id: root.id ?? 'root', subject: root.subject ?? 'root', score: rw * rc * boost });
    }

    const total = scores.reduce((acc, it) => acc + it.score, 0);
    return { total, scores };
  }
}
