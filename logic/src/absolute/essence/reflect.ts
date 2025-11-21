import { InMemoryEventBus, type EventBus } from '../core/bus';
import { makeInMemoryRepository } from '../../repository/memory';
import { ContextSchema, type Context } from '../../schema/context';
import { ContextEngine } from '../../relative/form/context/context-engine';
import type { Repository } from '../../repository/repo';
import type { Event } from '../core/message';
import BaseDriver from '../core/driver';
import { ActiveContext } from '../../schema/active';

export class ReflectDriver extends BaseDriver {
  private readonly contextEngine: ContextEngine;
  private readonly repo: Repository<Context>;
  private readonly eventBus: EventBus;

  constructor(repo?: Repository<Context>, bus?: EventBus) {
    super('ReflectDriver');

    this.eventBus = bus ?? new InMemoryEventBus();
    this.repo =
      repo ??
      (makeInMemoryRepository(
        ContextSchema as any,
      ) as unknown as Repository<Context>);
    this.contextEngine = new ContextEngine(this.repo, this.eventBus);
  }

  // Safe ID extraction helper - fixed to use Event[] type
  private extractId(events: Event[], expectedKind: string): string {
    const event = events.find((e) => e?.kind === expectedKind);
    if (!event) {
      throw new Error(`No ${expectedKind} event found in response`);
    }

    if (!event.payload) {
      throw new Error(`${expectedKind} event has no payload`);
    }

    const rawId = (event.payload as any)?.id;
    if (typeof rawId !== 'string' && typeof rawId !== 'number') {
      throw new Error(
        `${expectedKind} event payload.id must be string or number. Got: ${typeof rawId}`,
      );
    }

    return String(rawId);
  }

  // Process contexts using ActiveContext schema
  async processContexts(
    contexts: Array<ActiveContext>,
    particulars: any[] = [],
    context?: any,
  ) {
    return this.contextEngine.process(contexts, particulars, context);
  }

  // Commit processed actions
  async commitContexts(actions: any[], snapshot: { count: number }) {
    return this.contextEngine.commit(actions, snapshot);
  }

  // Process and commit in one step
  async processAndCommitContexts(
    contexts: Array<ActiveContext>,
    particulars: any[] = [],
    context?: any,
  ) {
    const { actions, snapshot } = await this.processContexts(
      contexts,
      particulars,
      context,
    );
    return this.commitContexts(actions, snapshot);
  }

  // Direct context access via engine
  async getContext(id: string) {
    return this.contextEngine.getContext(id);
  }

  // Create context via engine - fixed to use proper Event[] type
  async createContext(input: {
    id?: string;
    type: string;
    name: string;
    state?: any;
  }): Promise<string> {
    const events = await this.contextEngine.handle({
      kind: 'context.create',
      payload: input,
    } as any);

    return this.extractId(events, 'context.create');
  }

  // Delete context via engine
  async deleteContext(id: string) {
    await this.contextEngine.handle({
      kind: 'context.delete',
      payload: { id },
    } as any);
  }
}

// Functional interfaces
export async function processContexts(
  contexts: Array<ActiveContext>,
  driver: ReflectDriver = DefaultReflectDriver,
  particulars: any[] = [],
  context?: any,
) {
  return driver.processContexts(contexts, particulars, context);
}

export async function commitContexts(
  actions: any[],
  snapshot: { count: number },
  driver: ReflectDriver = DefaultReflectDriver,
) {
  return driver.commitContexts(actions, snapshot);
}

export const DefaultReflectDriver = new ReflectDriver();
