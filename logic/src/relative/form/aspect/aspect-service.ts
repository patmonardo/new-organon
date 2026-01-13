import { InMemoryEventBus, type EventBus } from '@absolute';
import type { Event } from '@absolute';
import { AspectEngine, type AspectStore } from './aspect-engine';
import { AspectSchema, type AspectShapeRepo } from '@schema/aspect';

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

class InMemoryAspectStore implements AspectStore {
  private store = new Map<string, AspectShapeRepo>();

  async getAspectById(id: string): Promise<AspectShapeRepo | null> {
    return this.store.get(id) ?? null;
  }

  async saveAspect(data: Partial<AspectShapeRepo>): Promise<AspectShapeRepo> {
    const existing = data.id ? this.store.get(data.id) : undefined;
    const now = Date.now();
    const next: AspectShapeRepo = {
      id: data.id ?? `aspect:${now}:${Math.random().toString(36).slice(2, 10)}`,
      type: data.type ?? existing?.type ?? 'aspect.unknown',
      name: data.name ?? existing?.name,
      state: data.state ?? existing?.state ?? {},
      signature: data.signature ?? existing?.signature,
      facets: data.facets ?? existing?.facets ?? {},
      status: data.status ?? existing?.status,
      tags: data.tags ?? existing?.tags,
      meta: data.meta ?? existing?.meta,
      createdAt: existing?.createdAt ?? data.createdAt ?? now,
      updatedAt: now,
    };
    this.store.set(next.id, next);
    return next;
  }

  async deleteAspect(id: string): Promise<boolean> {
    return this.store.delete(id);
  }
}

export class AspectService {
  private readonly engine: AspectEngine;
  private readonly bus: EventBus;

  constructor(repo?: AspectStore, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    const defaultRepo = repo ?? new InMemoryAspectStore();
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

  async get(id: AspectId): Promise<AspectShapeRepo | undefined> {
    const formAspect = await this.engine.getAspect(id);
    const doc = formAspect?.toSchema();
    return doc ? AspectSchema.parse(doc) : undefined;
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
