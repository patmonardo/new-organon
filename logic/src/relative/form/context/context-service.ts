import type { Event, EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import type { Repository } from '@repository';
import { ContextSchema, type Context } from '@schema';
import { ContextEngine } from './context-engine';

export type ContextId = string;

export class ContextService {
  private readonly engine: ContextEngine;
  private readonly bus: EventBus;

  constructor(private readonly repo?: Repository<Context>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    this.engine = new ContextEngine(this.repo as any, this.bus);
  }

  // Event API (noun.verb only)
  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  // Low-level command API (pass-through)
  async send(cmd: {
    kind: string;
    payload?: any;
    meta?: Record<string, unknown>;
  }) {
    return this.engine.handle(cmd as any);
  }

  // Reads
  async get(id: ContextId): Promise<Context | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ? ContextSchema.parse(doc) : undefined;
    }
    const ctx = (this.engine as any).getContext
      ? (this.engine as any).getContext(id)
      : undefined;
    return ctx ? ContextSchema.parse((ctx as any).toSchema()) : undefined;
  }

  // SDK verbs (delegate to engine)

  async create(input: { type: string; name?: string }) {
    const [e] = await this.engine.handle({
      kind: 'context.create',
      payload: input,
    } as any);

    const payload = (e?.payload ?? {}) as any;
    const id =
      payload?.id ??
      payload?.shape?.core?.id ??
      payload?.shape?.id ??
      payload?.core?.id;
    if (!id) throw new Error('missing id in context.create event payload');
    return String(id) as ContextId;
  }

  async delete(id: ContextId) {
    await this.engine.handle({
      kind: 'context.delete',
      payload: { id },
    } as any);
  }

  async describe(id: ContextId) {
    const [e] = await this.engine.handle({
      kind: 'context.describe',
      payload: { id },
    } as any);
    return e.payload as {
      id: string;
      type: string;
      name: string | null;
      state?: unknown;
    };
  }

  async setCore(id: ContextId, core: { name?: string; type?: string }) {
    await this.engine.handle({
      kind: 'context.setCore',
      payload: { id, ...core },
    } as any);
  }

  async setState(id: ContextId, state: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'context.setState',
      payload: { id, state },
    } as any);
  }

  async patchState(id: ContextId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'context.patchState',
      payload: { id, patch },
    } as any);
  }

  // Convenience: create → describe
  async createAndDescribe(input: { type: string; name?: string }) {
    const id = await this.create(input);
    const info = await this.describe(id);
    return { id, info };
  }
}
