export type Intent = {
  id: string;
  morphId: string; // refer to Morph.ruleSpec (ground rules)
  goal: string; // human-readable goal
  priority?: number; // for ordering in planning
  params?: Record<string, unknown>; // parameterize the intent
  policy?: { requireApproval?: boolean; maxAttempts?: number };
  createdAt?: string;
};

export type IntentRegistry = {
  intents: Intent[];
};

/**
 * Resolve intents for derived artifacts.
 * - Input: ground result (relations/properties) and intents registry.
 * - Output: prioritized candidate actions for control/planning.
 */
export function intentsForGround(
  ground: { relations: unknown[]; properties: unknown[] },
  registry: IntentRegistry,
) {
  // naive merge: map relations -> intents via ruleId -> morphId -> intent
  // realistic impl: match provenance.ruleId -> morphId -> lookup intent
  const candidates: Array<{ intentId: string; score: number; payload: unknown }> = [];

  // placeholder logic: produce one candidate per intent
  for (const it of registry.intents) {
    const score = (it.priority ?? 1);
    candidates.push({ intentId: it.id, score, payload: { goal: it.goal, sample: ground.relations.slice(0, 3) } });
  }

  // sort by score desc
  candidates.sort((a, b) => b.score - a.score);
  return candidates;
}
