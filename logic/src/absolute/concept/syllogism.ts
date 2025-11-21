import type { ProcessorInputs } from "../core/contracts";
import type { WorldEdge } from "../../schema/world";

// Classical valid moods (minimal set) by figure
const VALID: Record<string, Set<string>> = {
  "1": new Set(["AAA", "EAE", "AII", "EIO"]),
  "2": new Set(["EAE", "AEE", "EIO", "AOO"]),
  "3": new Set(["IAI", "AII", "OAO", "EIO"]),
  "4": new Set(["AAI", "AEE", "IAI", "EIO"]),
};

function isValid(figure: string, mood: string): boolean {
  return VALID[figure]?.has(mood) ?? false;
}

// Map conclusion letter to a relation kind + direction policy
function conclusionToEdgeKind(letter: "A" | "E" | "I" | "O"): {
  kind: string;
  direction: "directed" | "bidirectional";
} {
  switch (letter) {
    case "A":
      return { kind: "subset_of", direction: "directed" }; // S → P
    case "E":
      return { kind: "disjoint_from", direction: "bidirectional" };
    case "I":
      return { kind: "overlaps", direction: "bidirectional" };
    case "O":
      return { kind: "not_subset_of", direction: "directed" }; // S ↛ P
  }
}

/**
 * deriveSyllogisticEdges — “truth from figures”:
 * If a syllogism (figure, mood) is valid, emit an edge for its conclusion S–P.
 * Non-destructive: returns a layer of derived edges; does not mutate World.
 */
export function deriveSyllogisticEdges(input: ProcessorInputs): WorldEdge[] {
  const out: WorldEdge[] = [];
  for (const syl of input.syllogisms) {
    const fig = syl.shape.figure;
    const mood = syl.shape.mood;
    if (!isValid(fig, mood)) continue;

    const letter = mood[2] as "A" | "E" | "I" | "O";
    const { kind, direction } = conclusionToEdgeKind(letter);

    const S = syl.shape.terms.S;
    const P = syl.shape.terms.P;

    out.push({
      kind,
      direction,
      source: { id: S.id, type: "system.Thing" },
      target: { id: P.id, type: "system.Thing" },
    });
  }
  return out;
}
