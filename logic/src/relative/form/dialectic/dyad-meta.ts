export type ReflectionAppearanceSide = 'reflection' | 'appearance';
export type ReflectionAppearanceRole = 'reflecting' | 'conceiving';
export type DyadTelos = 'knowing';

/**
 * Canonical consciousness dyad:
 * - Reflection ↔ Appearance is our Reflecting ↔ Conceiving dyad
 * - The dyad is teleologically oriented; it "ends" in Knowing
 */
export function roleForSide(side: ReflectionAppearanceSide): ReflectionAppearanceRole {
  return side === 'reflection' ? 'reflecting' : 'conceiving';
}

export function withDyadMeta(
  base: Record<string, unknown> | undefined,
  input: { side: ReflectionAppearanceSide },
): Record<string, unknown> {
  return {
    ...(base ?? {}),
    dyad: 'reflection-appearance',
    dyadSide: input.side,
    dyadRole: roleForSide(input.side),
    dyadTelos: 'knowing' satisfies DyadTelos,
  };
}
