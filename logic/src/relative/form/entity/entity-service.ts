import type { Event, EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { EntityEngine } from './entity-engine';
import type { EntityShapeRepo } from '@schema/entity';

export type EntityId = string;

export class EntityService {
  private readonly engine: EntityEngine;
  private readonly bus: EventBus;

  constructor(private readonly repo?: any, bus?: EventBus) {
    this.bus = bus ?? new InMemoryEventBus();
    this.engine = new EntityEngine(this.repo, this.bus);
  }

  on(kind: string, handler: (e: Event) => void) {
    return this.bus.subscribe(kind, handler);
  }

  async send(cmd: {
    kind: string;
    payload?: any;
    meta?: Record<string, unknown>;
  }) {
    return this.engine.handle(cmd as any);
  }

  async get(id: EntityId): Promise<EntityShapeRepo | undefined> {
    if (this.repo) {
      if (this.repo.getEntityById) {
        return await this.repo.getEntityById(id);
      }
      const doc = await this.repo.get(id);
      return doc ? doc : undefined;
    }
    const entity = await this.engine.getEntity(id);
    return entity ? entity.toSchema() : undefined;
  }

  async create(input: {
    type: string;
    formId: string;
    name?: string;
    description?: string;
  }) {
    const [e] = await this.engine.handle({
      kind: 'entity.create',
      payload: input,
    } as any);

    const payload = (e?.payload ?? {}) as any;
    const id = payload?.id;
    if (!id) throw new Error('missing id in entity.create event payload');
    return String(id);
  }

  async delete(id: EntityId) {
    await this.engine.handle({ kind: 'entity.delete', payload: { id } } as any);
  }

  async describe(id: EntityId) {
    const [e] = await this.engine.handle({
      kind: 'entity.describe',
      payload: { id },
    } as any);
    return e?.payload as {
      id: string;
      type: string;
      name: string | null;
      state?: unknown;
      signatureKeys?: string[];
      facetsKeys?: string[];
    };
  }

  async setCore(id: EntityId, core: { name?: string; type?: string }) {
    await this.engine.handle({
      kind: 'entity.setCore',
      payload: { id, ...core },
    } as any);
  }

  async setState(id: EntityId, state: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'entity.setState',
      payload: { id, state },
    } as any);
  }

  async patchState(id: EntityId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'entity.patchState',
      payload: { id, patch },
    } as any);
  }

  async setFacets(id: EntityId, facets: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'entity.setFacets',
      payload: { id, facets },
    } as any);
  }

  async mergeFacets(id: EntityId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'entity.mergeFacets',
      payload: { id, patch },
    } as any);
  }

  async setSignature(id: EntityId, signature?: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'entity.setSignature',
      payload: { id, signature },
    } as any);
  }

  async mergeSignature(id: EntityId, patch: Record<string, unknown>) {
    await this.engine.handle({
      kind: 'entity.mergeSignature',
      payload: { id, patch },
    } as any);
  }

  async createAndDescribe(input: {
    type: string;
    formId: string;
    name?: string;
    description?: string;
  }) {
    const id = await this.create(input);
    const info = await this.describe(id);
    return { id, info };
  }
}
