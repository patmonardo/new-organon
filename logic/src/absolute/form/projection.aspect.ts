import { ActiveAspectSchema, type ActiveAspect } from "../../schema/active";

export type AspectProjectionInputs = {
  aspects?: ActiveAspect[];
};

export type AspectProjectionResult = {
  upserts: ActiveAspect[];
  deletes: string[];
};

/**
 * computeAspectProjection — Skeletal aspect projection.
 * - No relation endpoints (source/target/type) or legacy fields.
 * - Deterministic ids: proj:<name>:<baseId-or-slug(name)>
 * - Passes through Activation flags; derives active from revoked if needed.
 */
export function computeAspectProjection(
  name: string,
  input: AspectProjectionInputs,
): AspectProjectionResult {
  const base = [...(input.aspects ?? [])]
    .map((a) => ActiveAspectSchema.parse(a))
    .sort((a, b) => (a.id ?? "").localeCompare(b.id ?? ""));

  const n = slug(name || "proj");

  const upserts: ActiveAspect[] = base.map((a) =>
    ActiveAspectSchema.parse({
      id: `proj:${n}:${a.id ?? slug(a.name ?? "anon")}`,
      kind: a.kind ?? "system.Aspect",
      name: a.name,
      schema: a.schema,
      particularityOf: a.particularityOf ?? `ess:proj:${n}`,
      // prefer explicit active; fall back to !revoked; default true
      active:
        a.active !== undefined ? a.active : a.revoked !== true ? true : false,
      // keep any activation window fields if they exist in ActivationSchema
      startsAt: (a as any).startsAt,
      endsAt: (a as any).endsAt,
    }),
  );

  return { upserts, deletes: [] };
}

function slug(s: string): string {
  return s.trim().toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-+|-+$/g, "");
}

export default { computeAspectProjection };
