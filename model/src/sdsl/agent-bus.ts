/**
 * Agent Bus (Model Middleware)
 *
 * Minimal pub/sub for agent consumption.
 *
 * This is intentionally small and structural:
 * - Event type matches the FactTrace bridge (kind/payload/meta)
 * - No dependency on @organon/logic or @organon/task
 */

import type { FactTraceEvent } from './fact-trace';
import type { RealityPipe, RealityPipeEnvelope } from './reality-pipe';
import { InMemoryRealityPipe } from './reality-pipe';

export type AgentBusEvent = FactTraceEvent;

export type AgentBusHandler = (event: AgentBusEvent) => void;

export interface AgentBus {
  publish(event: AgentBusEvent): void;
  subscribe(handler: AgentBusHandler): () => void;
}

export class InMemoryAgentBus implements AgentBus {
  private readonly bus: RealityPipe<'agent.fact', AgentBusEvent>;

  constructor(opts: { bus?: RealityPipe<'agent.fact', AgentBusEvent> } = {}) {
    this.bus = opts.bus ?? new InMemoryRealityPipe<'agent.fact', AgentBusEvent>();
  }

  publish(event: AgentBusEvent): void {
    this.bus.publish({ kind: 'agent.fact', payload: event });
  }

  subscribe(handler: AgentBusHandler): () => void {
    return this.bus.subscribe((envelope: RealityPipeEnvelope<'agent.fact', AgentBusEvent>) => {
      handler(envelope.payload);
    });
  }
}

