import type { EssenceGraph, Projections } from './kriya';

export type Concept = {
  id: string;
  label?: string;
  description?: string;
  derivedFrom: string[]; // source node ids
  relations?: string[]; // relation ids
  summary?: string;
  provenance?: { sourceEngine?: string; ts: string; meta?: any };
};

export type LogogenesisOptions = {
  summarizer?: (ctx: {
    entities: any[];
    relations: any[];
    projections?: Projections;
  }) => string;
  emit?: (c: Concept) => void;
  provenanceTag?: string;
};

/**
 * synthesizeConcepts - lightweight, deterministic concept creation from an EssenceGraph.
 * - Intended as a minimal, replaceable logogenesis step (no external ML/LLM calls here).
 * - Keeps provenance and derived-from links so the processor can audit.
 */
export function synthesizeConcepts(
  graph: EssenceGraph,
  projections?: Projections,
  opts?: LogogenesisOptions,
): Concept[] {
  const summarizer =
    opts?.summarizer ??
    ((ctx: { entities: any[]; relations: any[] }) => {
      // naive summary: list entity types and relation count
      const types = Array.from(
        new Set(ctx.entities.map((e) => (e as any)?.type || 'unknown')),
      );
      return `Concept of ${types.join(', ')} (${
        ctx.relations.length
      } relations)`;
    });

  const now = new Date().toISOString();
  const concepts: Concept[] = [];

  for (const ent of graph.entities || []) {
    // guard/normalize entity shape to silence TSC on ent.id / ent.type
    const entId = (ent as any)?.id ?? String(Math.random()).slice(2);
    const entType = (ent as any)?.type ?? undefined;

    const related = (graph.relations || []).filter(
      (r: any) => r.source?.id === entId || r.target?.id === entId,
    );
    const derivedFrom = [entId];
    const relations = related.map((r: any) => r.id).filter(Boolean);

    const summary = summarizer({
      entities: [ent],
      relations: related,
      projections,
    });

    const c: Concept = {
      id: `concept:${entId}`,
      label: entType ?? entId,
      description: (ent as any)?.description ?? undefined,
      derivedFrom,
      relations,
      summary,
      provenance: { sourceEngine: opts?.provenanceTag ?? 'certainty', ts: now },
    };

    opts?.emit?.(c);
    concepts.push(c);
  }

  return concepts;
}

/**
 * generateConceptIndex - produce a map from concept id -> Concept for quick lookup.
 */
export function generateConceptIndex(concepts: Concept[]) {
  const map = new Map<string, Concept>();
  for (const c of concepts) map.set(c.id, c);
  return map;
}

export default {
  synthesizeConcepts,
  generateConceptIndex,
};
