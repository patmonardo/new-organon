import type { Event, EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import type { FormShapeRepo } from '@schema/form';
import { ShapeEngine } from './shape-engine';

export type ShapeId = string;

export class ShapeService {
  private readonly engine: ShapeEngine;
  private readonly bus: EventBus;

  constructor(private readonly repo?: any, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    this.engine = new ShapeEngine(this.repo, this.bus);
  }

  // Event API
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
  async get(id: ShapeId): Promise<FormShapeRepo | undefined> {
    // Prefer repo (Sthiti), fall back to in-memory engine (session)
    if (this.repo) {
      // If it's FormShapeRepository, use it directly
      if (this.repo.getFormById) {
        return await this.repo.getFormById(id);
      }
      // Otherwise it's InMemoryRepository
      const doc = await this.repo.get(id);
      return doc ? doc : undefined;
    }
    const fs = await this.engine.getShape(id);
    return fs ? fs.toSchema() : undefined;
  }

  // SDK conveniences (unified verbs)

  async create(input: { type: string; name?: string }) {
    const [e] = await this.engine.handle({
      kind: 'shape.create',
      payload: input,
    } as any);

    // Engine now emits a flat payload record: { id, type, name }
    // Keep backward compatibility with older nested shapes if encountered.
    const payload = (e?.payload ?? {}) as any;
    const id =
      payload?.id ??
      payload?.shape?.core?.id ??
      payload?.shape?.id ??
      payload?.core?.id;
    if (!id) throw new Error('missing id in shape.create event payload');
    return String(id);
  }

  async delete(id: ShapeId) {
    await this.engine.handle({ kind: 'shape.delete', payload: { id } } as any);
  }

  async describe(id: ShapeId) {
    const [e] = await this.engine.handle({
      kind: 'shape.describe',
      payload: { id },
    } as any);
    return e.payload as {
      id: string;
      type: string;
      name: string | null;
      state?: unknown;
      signatureKeys?: string[];
      facetsKeys?: string[];
    };
  }

  async setCore(id: ShapeId, core: { name?: string; type?: string }) {
    await this.engine.handle({
      kind: 'shape.setCore',
      payload: { id, ...core },
    } as any);
  }

  async setState(id: ShapeId, state: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'shape.setState',
      payload: { id, state },
    } as any);
  }

  async patchState(id: ShapeId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'shape.patchState',
      payload: { id, patch },
    } as any);
  }

  async setFacets(id: ShapeId, facets: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'shape.setFacets',
      payload: { id, facets },
    } as any);
  }

  async mergeFacets(id: ShapeId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'shape.mergeFacets',
      payload: { id, patch },
    } as any);
  }

  async setSignature(id: ShapeId, signature?: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'shape.setSignature',
      payload: { id, signature },
    } as any);
  }

  async mergeSignature(id: ShapeId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'shape.mergeSignature',
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
