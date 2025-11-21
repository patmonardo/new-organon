import type { ActiveAspect } from "../../schema/active";
import type { Judgment } from "../../schema/judgment";
import type { AspectTruth } from "./truth";

export type TeleologyGoal = {
	id: string; // tel:<aspectId>
	description?: string;
	priority?: number; // 1..10 (derived from truth score if present)
};

export type TeleologyInput = {
	relations?: ActiveAspect[]; // aspects (phenomenal)
	judgments?: Judgment[];
	qualquant?: Record<string, { tags?: string[]; score?: number; weight?: number }>;
	aspectTruth?: AspectTruth[];
	threshold?: number; // default 0.85
};

export function planTeleology(input: TeleologyInput): TeleologyGoal[] {
	const aspects = input.relations ?? [];
	const qualquant = input.qualquant ?? {};
	const truthMap = new Map<string, AspectTruth>();
	for (const t of input.aspectTruth ?? []) truthMap.set(t.aspectId, t);
	const threshold = typeof input.threshold === "number" ? input.threshold : 0.85;

	const out: TeleologyGoal[] = [];
	for (const a of aspects) {
		if (a.active === false || a.revoked) continue;
		const q = qualquant[a.id];
		const tags = q?.tags ?? [];
		const hinted = tags.includes("goal") || tags.includes("teleology") || tags.includes("purpose");
		const score = truthMap.get(a.id)?.score ?? (typeof q?.score === "number" ? q!.score! : undefined);
		const confident = typeof score === "number" ? score >= threshold : false;
		if (!hinted && !confident) continue;

		const prio = clamp01(typeof score === "number" ? score : hinted ? 0.88 : 0.5);
		out.push({
			id: `tel:${a.id}`,
			description: `${a.type ?? a.kind ?? "aspect"} goal for ${a.source?.id ?? "?"}→${a.target?.id ?? "?"}`,
			priority: Math.max(1, Math.round(prio * 10)),
		});
	}

	// deterministic ordering by id
	out.sort((x, y) => (x.id < y.id ? -1 : x.id > y.id ? 1 : 0));
	return out;
}

function clamp01(n: number) {
	if (!Number.isFinite(n)) return 0;
	return Math.min(1, Math.max(0, n));
}

export default planTeleology;
