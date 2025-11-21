import type { ActiveAspect } from "../../schema/active";
import { ActiveAspectSchema } from "../../schema/active";

// Incompatibility table by aspect name (predicate)
const INCOMPATIBLE: Record<string, Set<string>> = {
  subset_of: new Set(["disjoint_from", "not_subset_of"]),
  disjoint_from: new Set(["subset_of", "overlaps"]),
  overlaps: new Set(["disjoint_from"]),
  not_subset_of: new Set(["subset_of"]),
};

function equalRespect(a: ActiveAspect, b: ActiveAspect): boolean {
  // Respect: same particularity scope, or same name when scope absent
  if (a.particularityOf && b.particularityOf) {
    return a.particularityOf === b.particularityOf;
  }
  return (a.name ?? "") === (b.name ?? "");
}

function isIncompatible(aName?: string, bName?: string): boolean {
  if (!aName || !bName) return false;
  if (aName === bName) return false;
  const set = INCOMPATIBLE[aName] || new Set<string>();
  return set.has(bName);
}

function normId(a: ActiveAspect): string {
  return a.id ?? `aspect:${slug(a.name ?? "anon")}`;
}

export type ConflictProjectionInputs = { relations?: ActiveAspect[] };
export type ConflictProjectionResult = {
  upserts: ActiveAspect[];
  deletes: string[];
  conflictedIds: string[];
};

/**
 * computeAspectConflicts — skeletal contradiction markers for pairs of base aspects
 * that conflict within the same respect (same particularity/name) and incompatible predicates.
 *
 * Derived aspect:
 * - id: proj:conflict:<idA+idB> (lexicographically sorted)
 * - kind: "system.Aspect"
 * - name: "conflicts_with"
 * - particularityOf: prefer base scope or ess:proj:conflict
 */
export function computeAspectConflicts(
  input: ConflictProjectionInputs,
): ConflictProjectionResult {
  const rels = [...(input.relations ?? [])]
    .map((r) => ActiveAspectSchema.parse(r))
    .sort((a, b) => (normId(a) < normId(b) ? -1 : 1));

  const upserts: ActiveAspect[] = [];
  const conflicted = new Set<string>();

  for (let i = 0; i < rels.length; i++) {
    for (let j = i + 1; j < rels.length; j++) {
      const a = rels[i];
      const b = rels[j];
      if (!equalRespect(a, b)) continue;
      if (!isIncompatible(a.name, b.name)) continue;

      const [id1, id2] = [normId(a), normId(b)].sort();
      const id = `proj:conflict:${id1}+${id2}`;
      upserts.push(
        ActiveAspectSchema.parse({
          id,
          kind: "system.Aspect",
          name: "conflicts_with",
          particularityOf: a.particularityOf ?? b.particularityOf ?? "ess:proj:conflict",
          active:
            a.active !== undefined
              ? a.active
              : b.active !== undefined
              ? b.active
              : true,
          startsAt: (a as any).startsAt ?? (b as any).startsAt,
          endsAt: (a as any).endsAt ?? (b as any).endsAt,
        }),
      );
      conflicted.add(normId(a));
      conflicted.add(normId(b));
    }
  }

  return { upserts, deletes: [], conflictedIds: Array.from(conflicted.values()) };
}

function slug(s: string): string {
  return s.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
}

export default { computeAspectConflicts };
