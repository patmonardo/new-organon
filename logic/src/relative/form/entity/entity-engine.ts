import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import type { Repository } from '@repository';
import {
  type Entity,
  EntitySchema,
  createEntity,
} from '@schema';
import * as active from '@schema';
import { FormEntity } from './entity-form';

type BaseState = Entity['shape']['state'];
type Signature = NonNullable<Entity['shape']['signature']>;
type Facets = Entity['shape']['facets'];

type EntityCreateCmd = {
  kind: 'entity.create';
  payload: Parameters<typeof createEntity>[0];
  meta?: Record<string, unknown>;
};
type EntityDeleteCmd = {
  kind: 'entity.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type EntityDescribeCmd = {
  kind: 'entity.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type EntitySetCoreCmd = {
  kind: 'entity.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type EntitySetStateCmd = {
  kind: 'entity.setState';
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type EntityPatchStateCmd = {
  kind: 'entity.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};
type EntitySetFacetsCmd = {
  kind: 'entity.setFacets';
  payload: { id: string; facets: Facets };
  meta?: Record<string, unknown>;
};
type EntityMergeFacetsCmd = {
  kind: 'entity.mergeFacets';
  payload: { id: string; patch: Record<string, unknown> };
  meta?: Record<string, unknown>;
};
type EntitySetSignatureCmd = {
  kind: 'entity.setSignature';
  payload: { id: string; signature?: Signature };
  meta?: Record<string, unknown>;
};
type EntityMergeSignatureCmd = {
  kind: 'entity.mergeSignature';
  payload: { id: string; patch: Record<string, unknown> };
  meta?: Record<string, unknown>;
};

export type EntityCommand =
  | EntityCreateCmd
  | EntityDeleteCmd
  | EntityDescribeCmd
  | EntitySetCoreCmd
  | EntitySetStateCmd
  | EntityPatchStateCmd
  | EntitySetFacetsCmd
  | EntityMergeFacetsCmd
  | EntitySetSignatureCmd
  | EntityMergeSignatureCmd;

export class EntityEngine {
  constructor(
    private readonly repo: Repository<Entity>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'entity',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getEntity(id: string): Promise<FormEntity | undefined> {
    const doc = await this.repo.get(id);
    return doc ? FormEntity.fromSchema(doc) : undefined;
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private async mustGet(id: string): Promise<FormEntity> {
    const e = await this.getEntity(id);
    if (!e) throw new Error(`Entity not found: ${id}`);
    return e;
  }

  private async persist(e: FormEntity) {
    const id = e.id;
    const doc = EntitySchema.parse(e.toSchema());
    const current = await this.repo.get(id);
    if (current) {
      await this.repo.update(id, doc as any);
    } else {
      await this.repo.create(doc);
    }
  }

  async handle(cmd: EntityCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'EntityEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'entity.create': {
        const { payload } = cmd as EntityCreateCmd;
        const created = EntitySchema.parse(createEntity(payload as any));
        const ent = FormEntity.from(created);
        await this.persist(ent);
        return [
          this.emit(base, 'entity.create', {
            id: ent.id,
            type: ent.type,
            name: ent.name ?? null,
          }),
        ];
      }

      case 'entity.delete': {
        const { id } = (cmd as EntityDeleteCmd).payload;
        const existed = await this.repo.get(id);
        if (existed) {
          await this.repo.delete(id);
        }
        return [this.emit(base, 'entity.delete', { id, ok: !!existed })];
      }

      case 'entity.setCore': {
        const { id, name, type } = (cmd as EntitySetCoreCmd).payload;
        const e = await this.mustGet(id);
        e.setCore({ name, type });
        await this.persist(e);
        return [
          this.emit(base, 'entity.setCore', {
            id,
            name: e.name ?? null,
            type: e.type,
          }),
        ];
      }

      case 'entity.setState': {
        const { id, state } = (cmd as EntitySetStateCmd).payload;
        const e = await this.mustGet(id);
        e.setState(state as any);
        await this.persist(e);
        return [this.emit(base, 'entity.setState', { id })];
      }

      case 'entity.patchState': {
        const { id, patch } = (cmd as EntityPatchStateCmd).payload;
        const e = await this.mustGet(id);
        e.patchState(patch as any);
        await this.persist(e);
        return [this.emit(base, 'entity.patchState', { id })];
      }

      case 'entity.setFacets': {
        const { id, facets } = (cmd as EntitySetFacetsCmd).payload;
        const e = await this.mustGet(id);
        e.setFacets(facets as any);
        await this.persist(e);
        return [this.emit(base, 'entity.setFacets', { id })];
      }

      case 'entity.mergeFacets': {
        const { id, patch } = (cmd as EntityMergeFacetsCmd).payload;
        const e = await this.mustGet(id);
        e.mergeFacets(patch as any);
        await this.persist(e);
        return [this.emit(base, 'entity.mergeFacets', { id })];
      }

      case 'entity.setSignature': {
        const { id, signature } = (cmd as EntitySetSignatureCmd).payload;
        const e = await this.mustGet(id);
        e.setSignature(signature as any);
        await this.persist(e);
        return [this.emit(base, 'entity.setSignature', { id })];
      }

      case 'entity.mergeSignature': {
        const { id, patch } = (cmd as EntityMergeSignatureCmd).payload;
        const e = await this.mustGet(id);
        e.mergeSignature(patch as any);
        await this.persist(e);
        return [this.emit(base, 'entity.mergeSignature', { id })];
      }

      case 'entity.describe': {
        const { id } = (cmd as EntityDescribeCmd).payload;

        // Use repo directly instead of mustGet to handle missing entities gracefully
        const doc = await this.repo.get(id);
        if (!doc) {
          return [this.emit(base, 'entity.describe', { id })];
        }

        const e = FormEntity.fromSchema(doc);
        return [
          this.emit(base, 'entity.describe', {
            id,
            type: e.type,
            name: e.name ?? null,
            state: doc.shape.state,
            signatureKeys: Object.keys(
              (doc.shape.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(doc.shape.facets ?? {}),
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  async process(
    entities: Array<active.ActiveEntity>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = active.parseActiveEntities(entities);
    const actions: any[] = [];
    for (const e of list) {
      if ((e as any).revoked === true) {
        if (e.id) actions.push({ type: 'entity.delete', id: e.id });
        continue;
      }
      actions.push({
        type: 'entity.upsert',
        id: e.id ?? childIdFromName((e as any).labels?.[0]),
        entityType: (e as any).entityType ?? 'system.Entity', // Changed from 'kind'
        labels: (e as any).labels, // Add this property explicitly
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'entity.delete') {
        const [evt] = await this.handle({
          kind: 'entity.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'entity.upsert') {
        const id = a.id as string;
        const existing = await this.getEntity(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'entity.create',
            payload: { id, type: a.entityType, name: a.name },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'entity.setCore',
            payload: { id, type: a.entityType, name: a.name },
          } as any);
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}

function childIdFromName(name?: string) {
  if (!name) return `entity:${Math.random().toString(36).slice(2, 10)}`;
  return `entity:${name.toLowerCase().replace(/\s+/g, '-')}`;
}
