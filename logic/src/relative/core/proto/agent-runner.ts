import type { EmpowermentLike } from '../empowerment-core';
import type { EmpowermentProvider } from '../empowerment-core';
import { PrototypeProcessor } from './proto-processor';

export type ProtoTask = {
  id: string;
  actor: string; // subject executing the task
  action: string;
  resource?: string;
  payload?: any;
  meta?: Record<string, any>;
};

/**
 * AgentRunner - thin orchestrator that asks the Processor whether an actor is allowed
 * to perform an action on a resource and then performs a simulated execution (audit).
 * Keeps everything in-proto (no external task package).
 */
export class AgentRunner {
  private readonly processor: PrototypeProcessor;
  private readonly logger?: (msg: string, ...args: any[]) => void;

  constructor(
    providerEngines: EmpowermentProvider[],
    opts?: {
      logger?: (msg: string, ...args: any[]) => void;
      initiation?: EmpowermentLike;
    },
  ) {
    this.processor = new PrototypeProcessor(providerEngines, {
      logger: opts?.logger,
      initiation: opts?.initiation,
    });
    this.logger = opts?.logger;
  }

  // Attempt to run a ProtoTask. Returns an execution result object.
  async runTask(task: ProtoTask, opts?: { privilegeBoost?: number }) {
    this.logger?.(
      `[agent-runner] request task=${task.id} actor=${task.actor} action=${task.action} resource=${task.resource}`,
    );
    // authorize via the prototype processor (pure witness)
    const result = await this.processor.run(
      task.actor,
      task.action,
      task.resource,
      { privilegeBoost: opts?.privilegeBoost ?? 1 },
    );

    const allowed = result.total > 0;
    if (!allowed) {
      this.logger?.(
        `[agent-runner] denied task=${task.id} actor=${task.actor} total=${result.total}`,
      );
      return { ok: false, reason: 'unauthorized', result };
    }

    // Simulated execution (in real system this would dispatch to worker / side-effect)
    this.logger?.(
      `[agent-runner] executing task=${task.id} actor=${task.actor} action=${task.action} resource=${task.resource} score=${result.total}`,
    );
    // produce an audit event (simple object)
    const audit = {
      taskId: task.id,
      actor: task.actor,
      action: task.action,
      resource: task.resource,
      timestamp: new Date().toISOString(),
      score: result.total,
      evidence: result.scores,
    };

    // return simulated execution outcome + audit
    const outcome = {
      ok: true,
      taskId: task.id,
      audit,
      output: { message: `Executed ${task.action}` },
    };
    this.logger?.(`[agent-runner] completed task=${task.id} ok=${outcome.ok}`);
    return outcome;
  }
}
