import { InMemoryEventBus, type EventBus } from '@absolute';
import type { Event } from '@absolute';
import { MorphEngine, type MorphStore } from './morph-engine';
import { MorphSchema, type MorphShapeRepo } from '@schema/morph';

export type MorphId = string;

export interface MorphCreateInput {
  id?: string;
  type: string; // Required - this is the canonical morph type
  name?: string;
  state?: Record<string, unknown>;
}

export interface MorphDescribeResult {
  id: string;
  type: string;
  name: string | null;
  state?: unknown;
  signatureKeys?: string[];
  facetsKeys?: string[];
}

class InMemoryMorphStore implements MorphStore {
  private store = new Map<string, MorphShapeRepo>();

  async getMorphById(id: string): Promise<MorphShapeRepo | null> {
    return this.store.get(id) ?? null;
  }

  async saveMorph(data: Partial<MorphShapeRepo>): Promise<MorphShapeRepo> {
    const existing = data.id ? this.store.get(data.id) : undefined;
    const now = Date.now();
    const next = MorphSchema.parse({
      id:
        data.id ??
        `morph:${now.toString(36)}:${Math.random().toString(36).slice(2, 8)}`,
      type: data.type ?? existing?.type ?? 'morph.unknown',
      name: data.name ?? existing?.name,
      description: data.description ?? existing?.description,
      inputType: data.inputType ?? existing?.inputType ?? 'FormShape',
      outputType: data.outputType ?? existing?.outputType ?? 'FormShape',
      transformFn: data.transformFn ?? existing?.transformFn,
      state: data.state ?? existing?.state ?? {},
      signature: data.signature ?? existing?.signature,
      facets: data.facets ?? existing?.facets ?? {},
      composition: data.composition ??
        existing?.composition ?? { kind: 'single', steps: [] },
      config: data.config ?? existing?.config ?? {},
      status: data.status ?? existing?.status,
      tags: data.tags ?? existing?.tags,
      meta: data.meta ?? existing?.meta,
      createdAt: existing?.createdAt ?? data.createdAt ?? now,
      updatedAt: now,
    });
    this.store.set(next.id, next);
    return next;
  }

  async deleteMorph(id: string): Promise<boolean> {
    return this.store.delete(id);
  }
}

export class MorphService {
  private readonly engine: MorphEngine;
  private readonly bus: EventBus;

  constructor(repo?: MorphStore, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    const defaultRepo = repo ?? new InMemoryMorphStore();
    this.engine = new MorphEngine(defaultRepo, this.bus);
  }

  // Event subscription
  on(kind: string, handler: (event: Event) => void): void {
    const anyBus = this.bus as any;
    if (typeof anyBus.subscribe === 'function') {
      anyBus.subscribe(kind, handler);
    } else if (typeof anyBus.on === 'function') {
      anyBus.on(kind, handler);
    }
  }

  // Direct engine access
  async send(cmd: {
    kind: string;
    payload?: any;
    meta?: Record<string, unknown>;
  }): Promise<Event[]> {
    return this.engine.handle(cmd as any);
  }

  // Safe ID extraction helper
  private extractId(events: Event[], expectedKind: string): MorphId {
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

    return String(rawId) as MorphId;
  }

  // CRUD operations - validate input before passing to engine
  async create(input: MorphCreateInput): Promise<MorphId> {
    // Validate required fields at service boundary
    if (!input.type) {
      throw new Error('MorphService.create: type is required');
    }

    const events = await this.engine.handle({
      kind: 'morph.create',
      payload: {
        id: input.id,
        type: input.type,
        name: input.name ?? null, // Explicit null for optional name
        state: input.state ?? {},
      },
    } as any);

    return this.extractId(events, 'morph.create');
  }

  async get(id: MorphId): Promise<MorphShapeRepo | undefined> {
    const formMorph = await this.engine.getMorph(id);
    return formMorph?.toSchema();
  }

  async describe(id: MorphId): Promise<MorphDescribeResult> {
    const events = await this.engine.handle({
      kind: 'morph.describe',
      payload: { id },
    } as any);

    const event = events.find((e) => e?.kind === 'morph.describe');
    if (!event?.payload) {
      throw new Error(`morph.describe failed for id: ${id}`);
    }

    return event.payload as MorphDescribeResult;
  }

  async setCore(
    id: MorphId,
    patch: { name?: string; type?: string },
  ): Promise<void> {
    await this.engine.handle({
      kind: 'morph.setCore',
      payload: { id, ...patch },
    } as any);
  }

  async setState(id: MorphId, state: Record<string, unknown>): Promise<void> {
    await this.engine.handle({
      kind: 'morph.setState',
      payload: { id, state },
    } as any);
  }

  async patchState(id: MorphId, patch: Record<string, unknown>): Promise<void> {
    await this.engine.handle({
      kind: 'morph.patchState',
      payload: { id, patch },
    } as any);
  }

  async delete(id: MorphId): Promise<void> {
    await this.engine.handle({
      kind: 'morph.delete',
      payload: { id },
    } as any);
  }

  // Convenience method
  async createAndDescribe(
    input: MorphCreateInput,
  ): Promise<{ id: MorphId; info: MorphDescribeResult }> {
    const id = await this.create(input);
    const info = await this.describe(id);
    return { id, info };
  }
}
