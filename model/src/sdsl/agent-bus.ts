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
import type { FciBus, FciEnvelope } from './fci-bus';
import { InMemoryFciBus } from './fci-bus';

export type AgentBusEvent = FactTraceEvent;

export type AgentBusHandler = (event: AgentBusEvent) => void;

export interface AgentBus {
  publish(event: AgentBusEvent): void;
  subscribe(handler: AgentBusHandler): () => void;
}

export class InMemoryAgentBus implements AgentBus {
  private readonly bus: FciBus<'agent.fact', AgentBusEvent>;

  constructor(opts: { bus?: FciBus<'agent.fact', AgentBusEvent> } = {}) {
    this.bus = opts.bus ?? new InMemoryFciBus<'agent.fact', AgentBusEvent>();
  }

  publish(event: AgentBusEvent): void {
    this.bus.publish({ kind: 'agent.fact', payload: event });
  }

  subscribe(handler: AgentBusHandler): () => void {
    return this.bus.subscribe((envelope: FciEnvelope<'agent.fact', AgentBusEvent>) => {
      handler(envelope.payload);
    });
  }
}

