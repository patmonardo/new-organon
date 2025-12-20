export type AppearanceContainer = 'appearance';
export type AppearanceMoment = 'entity' | 'property' | 'aspect';

export type KantFormOfIntuition = 'space' | 'time';

export type AppearanceSemanticRole =
  | 'thing'
  | 'property'
  | 'relation-as-aspect';

/**
 * Appearance is the “contained” side of the Form Engine.
 *
 * Canonical mapping:
 * - Entity ≈ Thing
 * - Property ≈ Property
 * - Aspect ≈ Relation (as a presupposed thing-in-world)
 *
 * Mathematics/Quantity is treated as contained here (not the container-side law).
 */
export function appearanceSemanticRoleForMoment(
  moment: AppearanceMoment,
): AppearanceSemanticRole {
  switch (moment) {
    case 'entity':
      return 'thing';
    case 'property':
      return 'property';
    case 'aspect':
      return 'relation-as-aspect';
  }
}

export function withAppearanceMeta(
  base: Record<string, unknown> | undefined,
  input: {
    moment: AppearanceMoment;
    /** Optional label for a stage/pass; kept loose on purpose. */
    pass?: string;
  },
): Record<string, unknown> {
  const semanticRole = appearanceSemanticRoleForMoment(input.moment);
  return {
    ...(base ?? {}),
    dyad: 'reflection-appearance',
    dyadSide: 'appearance',
    dyadRole: 'conceiving',
    dyadTelos: 'knowing',
    container: 'appearance' satisfies AppearanceContainer,
    // Appearance is treated as the mathematical/quantity processor side.
    // Entity (thing-in-the-world) presupposes Kantian space/time as forms of intuition.
    mathematical: true,
    presupposes: ['space', 'time'] satisfies KantFormOfIntuition[],
    moment: input.moment,
    semanticRole,
    ...(input.pass ? { pass: input.pass } : {}),
  };
}
