import {
  createEntity,
  updateEntity,
  type Entity,
} from '@schema';
import type { Repository } from '@repository';

type EventHandler = (evt: { kind: string; payload?: any; meta?: any }) => void;

export class EntityService {
  private readonly repo?: Repository<any>;
  private readonly store = new Map<string, Entity>();
  private readonly listeners = new Map<string, EventHandler[]>();

  constructor(repo?: Repository<any>) {
    this.repo = repo;
  }

  on(event: string, handler: EventHandler) {
    const list = this.listeners.get(event) ?? [];
    list.push(handler);
    this.listeners.set(event, list);
  }

  private emit(kind: string, payload?: any, meta?: any) {
    const handlers = this.listeners.get(kind) ?? [];
    const evt = { kind, payload, meta };
    for (const h of handlers) h(evt);
  }

  private async persist(doc: Entity) {
    if (!this.repo) return;
    const id = doc.shape.core.id;
    const existing = await this.repo.get(id);
    if (existing) {
      await this.repo.update(id, doc);
    } else {
      await this.repo.create(doc);
    }
  }

  async create(input: Parameters<typeof createEntity>[0]): Promise<string> {
    // Forward payload directly so shape.core.type is set by createEntity
    const doc = createEntity(input as any);
    const id = doc.shape.core.id;
    this.store.set(id, doc);
    await this.persist(doc);
    this.emit('entity.create', {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name,
    });
    return id;
  }

  async get(id: string): Promise<Entity | undefined> {
    if (this.repo) {
      const doc = await this.repo.get(id);
      return doc ?? undefined;
    }
    return this.store.get(id);
  }

  async describe(id: string) {
    const doc = await this.get(id);
    if (!doc) throw new Error(`entity not found: ${id}`);
    this.emit('entity.describe', { id });
    return {
      id,
      type: doc.shape.core.type,
      name: doc.shape.core.name,
      state: doc.shape.state,
      signatureKeys: Object.keys((doc.shape as any).signature ?? {}),
      facetsKeys: Object.keys((doc.shape as any).facets ?? {}),
    };
  }

  async setCore(id: string, core: { name?: string; type?: string }) {
    const doc = (await this.get(id)) as Entity | undefined;
    if (!doc) throw new Error(`entity not found: ${id}`);
    const next = updateEntity(doc, { core } as any);
    this.store.set(id, next);
    await this.persist(next);
    this.emit('entity.setCore', {
      id,
      name: next.shape.core.name,
      type: next.shape.core.type,
    });
  }

  async setState(id: string, state: any) {
    const doc = (await this.get(id)) as Entity | undefined;
    if (!doc) throw new Error(`entity not found: ${id}`);
    const next = updateEntity(doc, { state } as any);
    this.store.set(id, next);
    await this.persist(next);
    this.emit('entity.setState', { id });
  }

  async patchState(id: string, patch: any) {
    const doc = (await this.get(id)) as Entity | undefined;
    if (!doc) throw new Error(`entity not found: ${id}`);
    const next = updateEntity(doc, { state: patch } as any);
    this.store.set(id, next);
    await this.persist(next);
    this.emit('entity.patchState', { id });
  }

  async delete(id: string) {
    const existed = this.store.delete(id);
    if (this.repo) await this.repo.delete(id);
    this.emit('entity.delete', { id, ok: existed ? true : false });
  }
}
