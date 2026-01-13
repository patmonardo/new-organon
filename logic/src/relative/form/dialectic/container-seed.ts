import type { KernelPort, KernelRunResult } from '@absolute/form/kernel-port';
import type { KernelFormProgram } from '@schema/kernel';

import { createLogicApi, type FormEvalCall } from '@absolute/form';

import { GdsFormProgram } from '@schema';
/**
 * Container → Contained (seed)
 *
 * This module is intentionally small: it establishes the *principle* that
 * discursive “Container” structure (Shape/Context/Morph) can be expressed as a
 * kernel Form program (Absolute Form surface), and that the discursive
 * “Contained” (Entity/Property) can be derived *from* the container.
 *
 * Dialectical mapping convention (semantic; not enforced):
 * - `program.shape`   → Essence envelope (what is held as form)
 * - `program.context` → Determination of Essence / Reflection inputs
 * - `program.morph`   → Ground as Active Ground (operator chain)
 */

export type ReflectionContainerProgramSeed = {
  id?: string;
  /** Optional Essence envelope for the kernel program. */
  shape?: KernelFormProgram['shape'];

  /** Optional Determination-of-Essence / Reflection envelope. */
  context?: KernelFormProgram['context'];

  /** Optional operator chain override (Active Ground). */
  patterns?: string[];
};

export function seedReflectionFormProgram(
  seed: ReflectionContainerProgramSeed = {},
): KernelFormProgram {
  const patterns = seed.patterns ?? ['essence', 'shine', 'reflection'];

  const program: Record<string, unknown> = {
    id: seed.id ?? 'formshape:container-seed',
    morph: { patterns },
  };

  if (seed.shape) program.shape = seed.shape;

  if (seed.context) {
    program.context = seed.context as Record<string, unknown>;
  }

  return program as KernelFormProgram;
}

export type ContainedSeed = {
  /** Seed for discursive Entity emergence (schema fields + basic constraints). */
  entity: {
    fields: string[];
    typeConstraints: Record<string, string>;
  };

  /** Seed for discursive Property emergence (invariants/conditions). */
  property: {
    validationRules: Record<string, string>;
    conditions: string[];
  };
};

/**
 * Minimal derivation: the “contained” emerges from the “container”.
 *
 * This intentionally does NOT try to fully materialize Entities/Properties.
 * It just makes the dependency direction explicit and machine-checkable.
 */
export function seedContainedFromProgram(
  program: KernelFormProgram,
): ContainedSeed {
  const shape = (program.shape ?? {}) as NonNullable<GdsFormProgram['shape']>;
  const ctx = (program.context ?? {}) as NonNullable<GdsFormProgram['context']>;

  const required = Array.isArray((shape as any).required_fields)
    ? ((shape as any).required_fields as string[])
    : [];
  const optional = Array.isArray((shape as any).optional_fields)
    ? ((shape as any).optional_fields as string[])
    : [];

  const typeConstraints =
    (shape as any).type_constraints &&
    typeof (shape as any).type_constraints === 'object'
      ? ((shape as any).type_constraints as Record<string, string>)
      : {};

  const validationRules =
    (shape as any).validation_rules &&
    typeof (shape as any).validation_rules === 'object'
      ? ((shape as any).validation_rules as Record<string, string>)
      : {};

  const conditions = Array.isArray((ctx as any).conditions)
    ? ((ctx as any).conditions as string[])
    : [];

  return {
    entity: {
      fields: [...required, ...optional],
      typeConstraints,
    },
    property: {
      validationRules,
      conditions,
    },
  };
}

/**
 * Convenience: turn a program into a kernel `form_eval.evaluate` call.
 *
 * This is where “Relative Container logic” touches the “Absolute Form surface”
 * via the GDSL boundary.
 */
export function seedFormEvalCallFromProgram(input: {
  user: FormEvalCall['user'];
  databaseId: string;
  graphName: string;
  outputGraphName?: string;
  program: KernelFormProgram;
  artifacts?: Record<string, unknown>;
}): FormEvalCall {
  return {
    facade: 'form_eval',
    op: 'evaluate',
    user: input.user,
    databaseId: input.databaseId,
    graphName: input.graphName,
    outputGraphName: input.outputGraphName,
    program: input.program as any,
    artifacts: input.artifacts ?? {},
  };
}

export type RunReflectionContainerResult = {
  program: GdsFormProgram;
  call: FormEvalCall;
  kernelResult: KernelRunResult;
  contained: ContainedSeed;
};

/**
 * Living link: Discursive Container → Kernel Absolute Form → Discursive Contained.
 *
 * This is the minimal “RS → TS” seam:
 * - TS constructs the Container program (discursive logic)
 * - kernel executes as Absolute Form (proof/witness)
 * - TS derives the Contained seed (entity/property) from the same container
 */
export async function runReflectionContainerOnKernel(input: {
  kernel: KernelPort;
  user: FormEvalCall['user'];
  databaseId: string;
  graphName: string;
  outputGraphName?: string;
  seed?: ReflectionContainerProgramSeed;
  artifacts?: Record<string, unknown>;
}): Promise<RunReflectionContainerResult> {
  const program = seedReflectionFormProgram(input.seed);
  const call = seedFormEvalCallFromProgram({
    user: input.user,
    databaseId: input.databaseId,
    graphName: input.graphName,
    outputGraphName: input.outputGraphName,
    program,
    artifacts: input.artifacts,
  });

  const api = createLogicApi(input.kernel);
  const kernelResult = await api.form.evaluate(call);
  const contained = seedContainedFromProgram(program);

  return { program, call, kernelResult, contained };
}
