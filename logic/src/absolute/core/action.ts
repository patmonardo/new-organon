import crypto from "crypto";

export type Action = {
  id: string;
  source: { entityId: string; propertyId: string };
  target: { entityId: string };
  kind: string;
  payload?: any;
  confidence: number;
  provenance?: { ruleId?: string; contextId?: string; timestamp: string; signature?: string };
};

export type KriyaActionResult = {
  actions: Action[];
  signatures: Record<string, string>;
  evidence: string[];
};

function stableHash(parts: string[]): string {
  return crypto.createHash("sha1").update(parts.sort().join("|")).digest("hex");
}

/**
 * Minimal actionStage prototype.
 * Heuristic: for numeric properties, if value > threshold (default 100), emit an "influence" action from the property owner to nearby targets.
 */
export async function actionStage(
  graph: { entities: any[]; properties: any[]; relations: any[] },
  reflect?: any,
  opts?: { threshold?: number; contextId?: string }
): Promise<KriyaActionResult> {
  const threshold = opts?.threshold ?? 100;
  const actions: Action[] = [];
  const signatures: Record<string, string> = {};
  const evidence: string[] = [];

  for (const p of graph.properties || []) {
    const value = (p as any).value ?? (p as any).default ?? null;
    const numeric = typeof value === "number";
    const propId = (p as any).id ?? (p as any).propertyId ?? null;
    if (!propId) continue; // skip properties without stable id

    if (numeric && value > threshold) {
      const id = `${propId}:action:influence`;
      const sourceEntityId = (p as any).entity ?? (p as any).entityId ?? (p as any).owner ?? null;
      if (!sourceEntityId) continue; // require source entity id

      const source = { entityId: String(sourceEntityId), propertyId: propId };

      // naive target selection: any entity that's not the source and has a stable id
      const candidateTargets = (graph.entities || [])
        .map((e) => ({
          entity: e,
          id: e.id ?? e.shape?.core?.id ?? e.core?.id ?? null,
        }))
        .filter((t) => t.id && String(t.id) !== source.entityId);

      for (const trec of candidateTargets) {
        const targetId = String(trec.id);
        if (!targetId) continue;

        const actionId = stableHash([id, source.entityId, targetId, String(value)]);
        const signature = stableHash([actionId, opts?.contextId ?? ""]);
        const act: Action = {
          id: actionId,
          source,
          target: { entityId: targetId },
          kind: "influence",
          payload: { magnitude: value },
          confidence: Math.min(1, value / (threshold * 2)),
          provenance: { contextId: opts?.contextId, timestamp: new Date().toISOString(), signature },
        };
        actions.push(act);
        signatures[actionId] = signature;
        evidence.push(`action:${actionId}:sig:${signature}`);

        // lightweight reflect hook: if reflect.verbose is true, add a short trace entry
        if (reflect && reflect.verbose) {
          try {
            evidence.push(`reflect:${propId}->${targetId}`);
          } catch {
            // ignore reflect failures
          }
        }
      }
    }
  }

  return { actions, signatures, evidence };
}

export default actionStage;
