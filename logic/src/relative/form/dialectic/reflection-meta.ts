export type ReflectionContainer = 'reflection';
export type ReflectionMoment = 'shape' | 'context' | 'morph';

export type ReflectionSemanticRole =
  | 'active-consciousness'
  | 'determination-of-reflection'
  | 'active-ground';

export type ReflectionPass = 'reflection' | 'foundation' | 'ground';

export function reflectionSemanticRoleForMoment(
  moment: ReflectionMoment,
): ReflectionSemanticRole {
  switch (moment) {
    case 'shape':
      return 'active-consciousness';
    case 'context':
      return 'determination-of-reflection';
    case 'morph':
      return 'active-ground';
  }
}

export function withReflectionMeta(
  base: Record<string, unknown> | undefined,
  input: {
    moment: ReflectionMoment;
    pass: ReflectionPass;
  },
): Record<string, unknown> {
  const semanticRole = reflectionSemanticRoleForMoment(input.moment);
  return {
    ...(base ?? {}),
    dyad: 'reflection-appearance',
    dyadSide: 'reflection',
    dyadRole: 'reflecting',
    dyadTelos: 'knowing',
    container: 'reflection' satisfies ReflectionContainer,
    moment: input.moment,
    semanticRole,
    pass: input.pass,
  };
}
