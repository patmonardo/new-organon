import {
  parseRootAgentAbsorbRequest,
  parseRootAgentBootEnvelope,
  parseRootAgentKernelTurn,
  parseRootAgentLoopTurn,
  type RootAgentAbsorbResult,
  type RootAgentAbsorbRequest,
  type RootAgentKernelTurn,
  type RootAgentLoopTurn,
} from './envelope';

import {
  TawActEventSchema,
  TawIntentEventSchema,
  TawPlanEventSchema,
  TawResultEventSchema,
  type TawIntentEvent,
} from '../schema/taw';

import { absorb as defaultAbsorb } from './absorb';
import type { RootAgentAbsorber, RootAgentState, RootAgentTurnExecutor } from './types';

export class RootAgent {
  private state: RootAgentState;

  constructor(boot: unknown) {
    const parsedBoot = parseRootAgentBootEnvelope(boot);

    // Normalize intent into Task's Zod v3 schema/type.
    const intent: TawIntentEvent = TawIntentEventSchema.parse(parsedBoot.intent);

    this.state = {
      boot: parsedBoot,
      context: parsedBoot.context,
      intent,
      syscalls: parsedBoot.syscalls,
      planPromptText: parsedBoot.planPromptText,
    };
  }

  getState(): RootAgentState {
    return this.state;
  }

  /**
   * Executes one agent turn and validates the emitted turn artifact.
   */
  async turn(executor: RootAgentTurnExecutor): Promise<RootAgentLoopTurn | RootAgentKernelTurn> {
    const out = await executor({ state: this.state });

    const plan = out.plan ? TawPlanEventSchema.parse(out.plan) : undefined;
    const act = out.act ? TawActEventSchema.parse(out.act) : undefined;
    const result = out.result ? TawResultEventSchema.parse(out.result) : undefined;

    const candidate: RootAgentLoopTurn = {
      meta: out.meta,
      context: this.state.context,
      intent: this.state.intent,
      plan,
      act,
      result,
      traceDelta: out.traceDelta ?? [],
    };

    if (out.kernelTurn?.kernelResult) {
      return parseRootAgentKernelTurn({
        ...candidate,
        kernelResult: out.kernelTurn.kernelResult,
      });
    }

    return parseRootAgentLoopTurn(candidate);
  }

  /**
   * Runs a full OS step: turn -> absorb -> update internal context.
   */
  async step(
    executor: RootAgentTurnExecutor,
    opts?: {
      absorber?: RootAgentAbsorber;
      absorbRequest?: Omit<RootAgentAbsorbRequest, 'previous' | 'traceDelta'>;
    },
  ): Promise<{ turn: RootAgentLoopTurn | RootAgentKernelTurn; absorb: RootAgentAbsorbResult }> {
    const turn = await this.turn(executor);

    const absorbReq: RootAgentAbsorbRequest = parseRootAgentAbsorbRequest({
      previous: this.state.context,
      traceDelta: turn.traceDelta,
      ...opts?.absorbRequest,
    });

    const absorber = opts?.absorber ?? defaultAbsorb;
    const absorbResult = await absorber(absorbReq);

    // Update context for next turn.
    this.state = {
      ...this.state,
      context: absorbResult.next,
    };

    return { turn, absorb: absorbResult };
  }
}
