/**
 * EssenceAgentRuntime (Model Middleware)
 *
 * The Model package as middle term:
 * - receives Essence as an event stream
 * - maintains a trace window
 * - produces the Agent's View (ContextDocument)
 * - fans out into plural AgentAdapter outputs
 */

import type { AgentBus, AgentBusEvent } from './agent-bus';
import { InMemoryAgentBus } from './agent-bus';
import type { ContextDocument } from './agent-view';
import type { FactTraceEvent, TraceGoal, TraceSchema } from './fact-trace';
import type { AgentOutputMap } from './essence-to-agent';
import type { AgentOutputFormat } from './agent-adapter';
import {
  createRuntimeState,
  recordEvent,
  snapshotTrace,
  toAgentOutputsFromState,
  toContextFromState,
  type RuntimeState,
} from './runtime-algorithms';

export type RuntimeOptions = {
  maxEvents?: number;
};

export type ContextOptions = {
  schema?: TraceSchema;
  goal?: TraceGoal;
  maxFacts?: number;
};

export class EssenceAgentRuntime {
  private readonly bus: AgentBus;
  private readonly maxEvents: number;
  private state: RuntimeState = createRuntimeState();

  constructor(opts: RuntimeOptions & { bus?: AgentBus } = {}) {
    this.bus = opts.bus ?? new InMemoryAgentBus();
    this.maxEvents = opts.maxEvents ?? 500;

    // Default behavior: the runtime records whatever is published.
    this.bus.subscribe((e) => this.record(e));
  }

  get agentBus(): AgentBus {
    return this.bus;
  }

  publish(event: AgentBusEvent): void {
    this.bus.publish(event);
  }

  record(event: FactTraceEvent): void {
    this.state = recordEvent(this.state, event, { maxEvents: this.maxEvents });
  }

  snapshot(): FactTraceEvent[] {
    return snapshotTrace(this.state);
  }

  toContext(opts?: ContextOptions): ContextDocument {
    return toContextFromState(this.state, {
      schema: opts?.schema,
      goal: opts?.goal,
      maxFacts: opts?.maxFacts,
    });
  }

  toAgentOutputs(opts?: ContextOptions & {
    formats?: AgentOutputFormat[];
    functionName?: string;
    operationType?: 'query' | 'mutation';
  }): AgentOutputMap {
    return toAgentOutputsFromState(this.state, {
      formats: opts?.formats,
      functionName: opts?.functionName,
      operationType: opts?.operationType,
      schema: opts?.schema,
      goal: opts?.goal,
      maxFacts: opts?.maxFacts,
    });
  }
}

