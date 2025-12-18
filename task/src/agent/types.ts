import type {
  ContextDocument,
  RootAgentAbsorbRequest,
  RootAgentAbsorbResult,
  RootAgentBootEnvelope,
  RootAgentKernelTurn,
  RootAgentLoopTurn,
  SyscallTable,
  TraceEvent,
} from '@organon/gdsl';

import type { TawActEvent, TawIntentEvent, TawPlanEvent, TawResultEvent } from '../schema/taw';

export type RootAgentState = {
  boot: RootAgentBootEnvelope;
  context: ContextDocument;
  intent: TawIntentEvent;
  syscalls?: SyscallTable;
  planPromptText?: string;
};

export type RootAgentTurnInput = {
  state: RootAgentState;
  meta?: RootAgentBootEnvelope['meta'];
};

export type RootAgentTurnOutput = {
  plan?: TawPlanEvent;
  act?: TawActEvent;
  result?: TawResultEvent;
  traceDelta?: TraceEvent[];
  kernelTurn?: Pick<RootAgentKernelTurn, 'kernelResult'>;
  meta?: RootAgentLoopTurn['meta'];
};

export type RootAgentTurnExecutor = (input: RootAgentTurnInput) => Promise<RootAgentTurnOutput> | RootAgentTurnOutput;

export type RootAgentAbsorber = (
  request: RootAgentAbsorbRequest,
) => Promise<RootAgentAbsorbResult> | RootAgentAbsorbResult;
