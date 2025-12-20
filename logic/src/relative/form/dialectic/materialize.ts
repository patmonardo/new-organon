import type { DialecticIR, DialecticState, DialecticCommand } from '@schema/dialectic';
import type { Event } from '@absolute';

import { ShapeEngine } from '../shape/shape-engine';
import { withReflectionMeta } from './reflection-meta';

export type MaterializeDialecticIROptions = {
  context?: unknown;
  /** Optional trace/meta payload threaded into every emitted dialectic command. */
  meta?: Record<string, unknown>;
  evaluate?: boolean;
  invariants?: boolean;
  forces?: boolean;
  transitions?: boolean;
};

function indexStates(ir: DialecticIR): Map<string, DialecticState> {
  return new Map(ir.states.map(s => [s.id, s] as const));
}

export function dialecticIRToCommands(
  ir: DialecticIR,
  opts: Pick<
    MaterializeDialecticIROptions,
    'context' | 'meta' | 'evaluate' | 'invariants' | 'forces' | 'transitions'
  > = {},
): DialecticCommand[] {
  const evaluate = opts.evaluate ?? true;
  const invariants = opts.invariants ?? true;
  const forces = opts.forces ?? false;
  const transitions = opts.transitions ?? true;

  const commands: DialecticCommand[] = [];
  const byId = indexStates(ir);

  if (evaluate) {
    for (const state of ir.states) {
      commands.push({
        kind: 'dialectic.evaluate',
        payload: { dialecticState: state, context: opts.context },
        meta: withReflectionMeta(opts.meta, { moment: 'shape', pass: 'reflection' }),
      });
    }
  }

  if (invariants) {
    for (const state of ir.states) {
      commands.push({
        kind: 'dialectic.invariant.check',
        payload: { stateId: state.id, invariants: state.invariants },
        meta: withReflectionMeta(opts.meta, { moment: 'context', pass: 'foundation' }),
      });
    }
  }

  if (forces) {
    for (const state of ir.states) {
      for (const force of state.forces ?? []) {
        commands.push({
          kind: 'dialectic.force.apply',
          payload: { stateId: state.id, force },
          meta: withReflectionMeta(opts.meta, { moment: 'context', pass: 'foundation' }),
        });
      }
    }
  }

  if (transitions) {
    for (const fromState of ir.states) {
      for (const transition of fromState.transitions ?? []) {
        const toState = byId.get(transition.to);
        if (!toState) continue;
        commands.push({
          kind: 'dialectic.state.transition',
          payload: {
            fromStateId: fromState.id,
            toStateId: toState.id,
            dialecticState: toState,
          },
          meta: withReflectionMeta(opts.meta, { moment: 'morph', pass: 'ground' }),
        });
      }
    }
  }

  return commands;
}

export async function materializeDialecticIR(
  engine: ShapeEngine,
  ir: DialecticIR,
  opts: MaterializeDialecticIROptions = {},
): Promise<Event[]> {
  const commands = dialecticIRToCommands(ir, opts);
  const events: Event[] = [];
  for (const cmd of commands) {
    events.push(...(await engine.handle(cmd)));
  }
  return events;
}
