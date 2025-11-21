import { InMemoryEventBus, type EventBus } from '@absolute';
import { makeInMemoryRepository } from '@repository';
import { AspectSchema, type Aspect } from '@schema';
import { AspectEngine } from './aspect-engine';
import type { Repository } from '@repository';
import type { Event } from '@absolute';

export type AspectId = string;

export interface AspectCreateInput {
  id?: string;
  type: string;
  name?: string;
  state?: Record<string, unknown>;
}

export interface AspectDescribeResult {
  id: string;
  type: string;
  name: string | null;
  state?: unknown;
  signatureKeys?: string[];
  facetsKeys?: string[];
}

export class AspectService {
  private readonly engine: AspectEngine;
  private readonly bus: EventBus;

  constructor(repo?: Repository<Aspect>, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    const defaultRepo =
      repo ??
      (makeInMemoryRepository(
        AspectSchema as any,
      ) as unknown as Repository<Aspect>);
    this.engine = new AspectEngine(defaultRepo, this.bus);
  }

  on(kind: string, handler: (event: Event) => void): void {
    const anyBus = this.bus as any;
    if (typeof anyBus.subscribe === 'function') {
      anyBus.subscribe(kind, handler);
    } else if (typeof anyBus.on === 'function') {
      anyBus.on(kind, handler);
    }
  }

  async send(cmd: {
    kind: string;
    payload?: any;
    meta?: Record<string, unknown>;
  }): Promise<Event[]> {
    return this.engine.handle(cmd as any);
  }

  private extractId(events: Event[], expectedKind: string): AspectId {
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

    return String(rawId) as AspectId;
  }

  async create(input: AspectCreateInput): Promise<AspectId> {
    if (!input.type) {
      throw new Error('AspectService.create: type is required');
    }

    const events = await this.engine.handle({
      kind: 'aspect.create',
      payload: {
        id: input.id,
        type: input.type,
        name: input.name,
        state: input.state ?? {},
      },
    } as any);

    return this.extractId(events, 'aspect.created');
  }

  async get(id: AspectId): Promise<Aspect | undefined> {
    const formAspect = await this.engine.getAspect(id);
    return formAspect?.toSchema();
  }

  async describe(id: AspectId): Promise<AspectDescribeResult> {
    const events = await this.engine.handle({
      kind: 'aspect.describe',
      payload: { id },
    } as any);

    const event = events.find((e) => e?.kind === 'aspect.describe');
    if (!event?.payload) {
      throw new Error(`aspect.describe failed for id: ${id}`);
    }

    return event.payload as AspectDescribeResult;
  }

  async setCore(
    id: AspectId,
    patch: { name?: string; type?: string },
  ): Promise<void> {
    const cleanPatch: { name?: string; type?: string } = {};
    if (patch.name !== undefined) cleanPatch.name = patch.name;
    if (patch.type !== undefined) cleanPatch.type = patch.type;

    await this.engine.handle({
      kind: 'aspect.setCore',
      payload: { id, ...cleanPatch },
    } as any);
  }

  async setState(id: AspectId, state: Record<string, unknown>): Promise<void> {
    await this.engine.handle({
      kind: 'aspect.setState',
      payload: { id, state },
    } as any);
  }

  async patchState(
    id: AspectId,
    patch: Record<string, unknown>,
  ): Promise<void> {
    await this.engine.handle({
      kind: 'aspect.patchState',
      payload: { id, patch },
    } as any);
  }

  async delete(id: AspectId): Promise<void> {
    await this.engine.handle({
      kind: 'aspect.delete',
      payload: { id },
    } as any);
  }

  async createAndDescribe(
    input: AspectCreateInput,
  ): Promise<{ id: AspectId; info: AspectDescribeResult }> {
    const id = await this.create(input);
    const info = await this.describe(id);
    return { id, info };
  }
}
