import type { RootAgentTurnExecutor, RootAgentTurnInput, RootAgentTurnOutput } from './types';

export type SingleAgent = RootAgentTurnExecutor;

export function singleAgent(fn: (input: RootAgentTurnInput) => Promise<RootAgentTurnOutput> | RootAgentTurnOutput): SingleAgent {
  return fn;
}
