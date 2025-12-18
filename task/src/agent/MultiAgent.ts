import type { RootAgentTurnExecutor, RootAgentTurnInput, RootAgentTurnOutput } from './types';

export class MultiAgent {
  private readonly agents: RootAgentTurnExecutor[];

  constructor(agents: readonly RootAgentTurnExecutor[]) {
    this.agents = [...agents];
  }

  async decide(input: RootAgentTurnInput): Promise<RootAgentTurnOutput> {
    const merged: RootAgentTurnOutput = {
      traceDelta: [],
    };

    for (const agent of this.agents) {
      const out = await agent(input);

      if (out.meta) merged.meta = out.meta;
      if (out.plan) merged.plan = out.plan;
      if (out.act) merged.act = out.act;
      if (out.result) merged.result = out.result;
      if (out.kernelTurn) merged.kernelTurn = out.kernelTurn;

      if (out.traceDelta?.length) {
        merged.traceDelta = [...(merged.traceDelta ?? []), ...out.traceDelta];
      }

      // Stop once a concrete action/result has been produced.
      if (out.act || out.result || out.kernelTurn) break;
    }

    return merged;
  }
}
