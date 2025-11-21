import type { ProcessorInputs } from "./contracts";
import type { WorldEdge } from '@schema';

const THING = "system.Thing";
type ThingRef = { id: string; type: typeof THING };

function toThingRef(x: any): ThingRef | undefined {
  const id =
    x?.id ??
    x?.core?.id ??
    x?.shape?.core?.id ??
    x?.shape?.id ??
    (typeof x === "string" ? x : undefined);
  return id ? { id: String(id), type: THING } : undefined;
}

// Derive asserted edges from judgments (non-destructive; returns a layer)
export function deriveAssertedEdges(input: ProcessorInputs): WorldEdge[] {
  const edges: WorldEdge[] = [];
  for (const j of input.judgments ?? []) {
    const subj = toThingRef(j?.shape?.subject);
    const obj = toThingRef(j?.shape?.object);
    if (!subj || !obj) continue;

    const kind = String(j?.shape?.predicate ?? "related_to");
    const directed = j?.shape?.polarity === "affirm";

    edges.push({
      kind,
      direction: directed ? "directed" : "bidirectional",
      source: subj,
      target: obj,
    });
  }
  return edges;
}
