export type Sig = { id?: string; issuer?: string; signature?: string; [k: string]: any };
export type Facet = string | { id: string; scope?: string[]; [k: string]: any };

/** simple signature predicate: non-empty signature accepted */
export function isSignatureValid(sig?: Sig): boolean {
  if (!sig) return false;
  const s = (sig.signature ?? '').toString();
  return s.length > 0;
}

/** all signatures valid */
export function allSignaturesValid(sigs?: Sig[] | any[]): boolean {
  if (!sigs || sigs.length === 0) return true;
  return sigs.every(isSignatureValid);
}

/** facet matches action/resource */
export function facetMatches(f: Facet, ctx: { action?: string; resource?: string }): boolean {
  if (typeof f === 'string') {
    return f === '*' || f === ctx.action || f === ctx.resource || f.startsWith('resource:') && ctx.resource === f;
  }
  const scope = Array.isArray(f.scope) ? f.scope : [];
  if (scope.length === 0) return true;
  for (const s of scope) {
    if (s === '*' || s === ctx.resource) return true;
    if (typeof s === 'string' && s.endsWith('*') && ctx.resource?.startsWith(s.slice(0, -1))) return true;
  }
  return false;
}

/** any facet applicable */
export function anyFacetApplicable(facets?: Facet[], ctx?: { action?: string; resource?: string }): boolean {
  if (!facets || facets.length === 0) return true;
  return facets.some((f) => facetMatches(f, ctx ?? {}));
}
