import { EventEmitter } from 'events';
import type { EmpowermentLike, EmpowermentProvider } from '../empowerment-core';
import { AgentRunner } from './agent-runner';
import type { ProtoTask } from './agent-runner';

// narrow runner outcomes safely
function isSuccessOutcome(o: unknown): o is { ok: true; audit: any } {
  return (
    !!o &&
    typeof o === 'object' &&
    (o as any).ok === true &&
    'audit' in (o as any)
  );
}
function isFailureOutcome(
  o: unknown,
): o is { ok: false; reason?: string; result?: any } {
  return !!o && typeof o === 'object' && (o as any).ok === false;
}

export class AgentProcessor extends EventEmitter {
  private readonly runner: AgentRunner;
  private readonly logger?: (msg: string, ...args: any[]) => void;
  private queue: ProtoTask[] = [];
  // avoid depending on NodeJS type globals: use ReturnType<typeof setInterval>
  private timer?: ReturnType<typeof setInterval>;
  private polling = false;

  constructor(
    providerEngines: EmpowermentProvider[],
    opts?: {
      logger?: (msg: string, ...args: any[]) => void;
      initiation?: EmpowermentLike;
    },
  ) {
    super();
    this.logger = opts?.logger;
    this.runner = new AgentRunner(providerEngines, {
      logger: this.logger,
      initiation: opts?.initiation,
    });
  }

  submitTask(task: ProtoTask) {
    this.queue.push(task);
    this.logger?.(`[agent-processor] queued task=${task.id}`);
    this.emit('queued', task);
    return task.id;
  }

  // attempt one dequeue + run cycle; returns outcome or undefined when nothing to do
  async pollAndRunOnce(opts?: { privilegeBoost?: number }) {
    const task = this.queue.shift();
    if (!task) return undefined;
    this.logger?.(`[agent-processor] running task=${task.id}`);

    const outcome: unknown = await this.runner.runTask(task, {
      privilegeBoost: opts?.privilegeBoost,
    });

    // emit audit or denial event using type guards
    if (isSuccessOutcome(outcome)) {
      this.emit('audit', outcome.audit);
    } else if (isFailureOutcome(outcome)) {
      this.emit('denied', {
        task: task.id,
        reason: outcome.reason,
        result: outcome.result,
      });
    } else {
      // fallback for unexpected shape
      this.emit('denied', {
        task: task.id,
        reason: 'invalid-outcome',
        result: outcome,
      });
    }
    return outcome;
  }

  // start polling loop with interval ms; safe-guards to avoid concurrent runs
  startPolling(intervalMs = 500) {
    if (this.polling) return;
    this.polling = true;
    this.timer = setInterval(async () => {
      try {
        if (this.queue.length === 0) return;
        await this.pollAndRunOnce();
      } catch (err) {
        this.logger?.(`[agent-processor] poll error: ${String(err)}`);
      }
    }, intervalMs);
    this.logger?.('[agent-processor] started polling');
  }

  stopPolling() {
    if (this.timer) {
      clearInterval(this.timer as unknown as number);
      this.timer = undefined;
    }
    this.polling = false;
    this.logger?.('[agent-processor] stopped polling');
  }

  // expose queue length for simple monitoring
  queueLength() {
    return this.queue.length;
  }
}

export default AgentProcessor;
