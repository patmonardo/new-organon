import FormEmpowerment from './empowerment-form';

export type EmpowermentCombineResult = {
  total: number;
  scores: Array<{
    id: string;
    subject: string;
    score: number;
    provenance?: string;
  }>;
};

export function combineEmpowerments(
  tokens: Array<any>,
  root?: any,
): EmpowermentCombineResult {
  // Prefer FormEmpowerment.combine if available (keeps core defensive & engine-agnostic)
  if (FormEmpowerment && typeof (FormEmpowerment as any).combine === 'function') {
    return (FormEmpowerment as any).combine(tokens, root);
  }

  // Fallback simple aggregation (defensive)
  const scores = (tokens || []).map((t: any) => {
    const score =
      typeof t.score === 'number' ? t.score :
      typeof t.weight === 'number' && typeof t.certainty === 'number' ? t.weight * t.certainty :
      0;
    return {
      id: t?.id ?? String(Math.random()).slice(2),
      subject: t?.subject ?? t?.entity ?? 'unknown',
      score,
      provenance: t?.provenance,
    };
  });

  if (root) {
    const rootScore =
      typeof root.score === 'number' ? root.score :
      typeof root.weight === 'number' && typeof root.certainty === 'number' ? root.weight * root.certainty :
      0;
    if (!scores.some(s => s.id === (root as any).id)) {
      scores.push({
        id: (root as any).id ?? 'root',
        subject: (root as any).subject ?? 'root',
        score: rootScore,
        provenance: (root as any).provenance,
      });
    }
  }

  const total = scores.reduce((s, it) => s + (it.score || 0), 0);
  return { total, scores };
}

// Extended variant that accepts options (e.g., privilegeBoost multiplier).
export function combineEmpowermentsWithOptions(
  tokens: Array<any>,
  root?: any,
  opts?: { privilegeBoost?: number },
): EmpowermentCombineResult {
  // If FormEmpowerment.combine supports options, delegate
  if (FormEmpowerment && typeof (FormEmpowerment as any).combine === 'function') {
    return (FormEmpowerment as any).combine(tokens, root, opts);
  }

  // Fallback: compute then apply boost
  const base = combineEmpowerments(tokens, root);
  const boost = opts && typeof opts.privilegeBoost === 'number' ? opts.privilegeBoost : 1;
  if (boost === 1) return base;
  const boosted = base.scores.map(s => ({ ...s, score: s.score * boost }));
  const total = boosted.reduce((acc, it) => acc + it.score, 0);
  return { total, scores: boosted };
}

export default { combineEmpowerments, combineEmpowermentsWithOptions };
