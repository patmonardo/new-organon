import type { DialecticIR } from '@schema/dialectic';
import type { Event } from '@absolute';

import { ShapeEngine } from '../shape/shape-engine';
import { materializeDialecticIR, type MaterializeDialecticIROptions } from './materialize';

export type RunEssenceInferencesOptions = {
  /** Optional evaluation context passed into `dialectic.evaluate`. */
  context?: unknown;

  /** Optional meta payload threaded into every emitted dialectic command/event trace. */
  meta?: Record<string, unknown>;

  /** Defaults to true. */
  reflect?: boolean;

  /** Defaults to true. */
  found?: boolean;

  /** Defaults to true. */
  ground?: boolean;

  /** Defaults to false (kept off until trigger/effect evaluation is formalized). */
  applyForces?: boolean;
};

/**
 * Minimal Essence pass over DialecticIR.
 *
 * This is intentionally “small and strict”: it runs exactly three inference passes
 * that map cleanly onto the current ShapeEngine dialectic command surface:
 *
 * Reflection is treated as the container triad (Shape/Context/Morph).
 * We tag the generated commands with `meta.container='reflection'` and:
 * - Shape: `dialectic.evaluate` (state → active Shape)
 * - Context: `dialectic.invariant.check` (state → law/constraint checks)
 * - Morph: `dialectic.state.transition` (state → transformation edges)
 */
export async function runEssenceInferences(
  engine: ShapeEngine,
  ir: DialecticIR,
  opts: RunEssenceInferencesOptions = {},
): Promise<Event[]> {
  const reflect = opts.reflect ?? true;
  const found = opts.found ?? true;
  const ground = opts.ground ?? true;

  const materializeOpts: MaterializeDialecticIROptions = {
    context: opts.context,
    meta: opts.meta,
    evaluate: reflect,
    invariants: found,
    transitions: ground,
    forces: opts.applyForces ?? false,
  };

  return materializeDialecticIR(engine, ir, materializeOpts);
}
