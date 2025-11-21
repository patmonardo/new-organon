import BaseDriver from '../core/driver';
import type { EventBus } from '../core/bus';

import type { Repository } from '../../repository/repo';
import { makeInMemoryRepository } from '../../repository/memory';

import {
  createEntity,
  updateEntity,
  EntitySchema,
  type Entity,
} from '../../schema/entity';

export type RawEntity = Record<string, any>;
export type EntityInput = unknown;

type ActiveEntity = {
  id?: string;
  entityId?: string;
  entityType?: string;
  name?: string;
  revoked?: boolean;
  active?: boolean;
  state?: Record<string, unknown>;
  signature?: Record<string, unknown>;
  facets?: Record<string, unknown>;
  version?: string;
  ext?: Record<string, unknown>;
  shape?: any;
};

function toActiveEntity(input: EntityInput): ActiveEntity {
  const i = (input ?? {}) as any;
  return {
    id: typeof i.id === 'string' ? i.id : undefined,
    entityId: typeof i.entityId === 'string' ? i.entityId : undefined,
    entityType: typeof i.entityType === 'string' ? i.entityType : undefined,
    name: typeof i.name === 'string' ? i.name : undefined,
    revoked: !!i.revoked,
    active: !!i.active,
    state: i.state,
    signature: i.signature,
    facets: i.facets,
    version: i.version,
    ext: i.ext,
    shape: i.shape,
  };
}

export class ThingDriver extends BaseDriver {
  constructor() {
    super('ThingDriver');
  }

  toActive(input: unknown): ActiveEntity {
    return toActiveEntity(input as EntityInput);
  }
  fromActive(input: ActiveEntity) {
    return input;
  }

  private useRepo(repo?: Repository<Entity>): Repository<Entity> {
    return (
      repo ??
      (makeInMemoryRepository(
        EntitySchema as any,
      ) as unknown as Repository<Entity>)
    );
  }

  private toActiveBatch(inputs: unknown[] = []): ActiveEntity[] {
    return (inputs ?? []).map((i) => this.toActive(i));
  }

  async processEntities(
    entities: EntityInput[],
    _opts?: {
      repo?: Repository<Entity>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const actives = this.toActiveBatch(entities);
    const actions = actives.map((e) => {
      const id = String(e.id ?? e.entityId ?? '');
      if (e.revoked) return { type: 'entity.delete', id };
      return {
        type: 'entity.upsert',
        id,
        entityType: e.entityType ?? 'system.Entity',
        name: e.name,
        state: e.state,
        signature: e.signature,
        facets: e.facets,
        version: e.version,
        ext: e.ext,
      };
    });
    return { actions, snapshot: { count: actions.length } } as const;
  }

  async commitEntities(
    entities: EntityInput[],
    opts?: {
      repo?: Repository<Entity>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const repo = this.useRepo(opts?.repo);
    const bus = opts?.bus;
    const { actions, snapshot } = await this.processEntities(entities, opts);

    const errors: any[] = [];
    const events: any[] = [];
    const processedIds = new Set<string>(); // Track what we've created in this batch

    for (const a of actions) {
      try {
        if (a.type === 'entity.delete') {
          await repo.delete(a.id);
          const evt = { kind: 'entity.delete', payload: { id: a.id } };
          publish(bus, 'entity.delete', evt);
          events.push(evt);
          continue;
        }

        const id = a.id;
        // fetch once, treat as possibly undefined
        const existingDoc = await repo.get(id);
        const existsInBatch = processedIds.has(id);

        if (!existingDoc && !existsInBatch) {
          // create new entity
          const doc = createEntity({
            id,
            type: a.entityType!,
            name: a.name,
            state: a.state ?? {},
            signature: a.signature,
            facets: a.facets,
            version: a.version,
            ext: a.ext,
          });
          await repo.create(doc);
          processedIds.add(id); // Mark as created in this batch
          const evt = { kind: 'entity.create', payload: doc };
          publish(bus, 'entity.create', evt);
          events.push(evt);
        } else {
          // update existing entity (either from repo or created in this batch)
          // use existingDoc when available, otherwise try to re-read (defensive)
          const current = existingDoc ?? (await repo.get(id));
          if (!current) {
            // defensive fallback: if still missing, create instead of throwing
            const doc = createEntity({
              id,
              type: a.entityType ?? 'system.Entity',
              name: a.name,
              state: a.state ?? {},
              signature: a.signature,
              facets: a.facets,
              version: a.version,
              ext: a.ext,
            });
            await repo.create(doc);
            processedIds.add(id);
            const evt = { kind: 'entity.create', payload: doc };
            publish(bus, 'entity.create', evt);
            events.push(evt);
            continue;
          }

          const next = updateEntity(current, {
            core: {
              ...(a.name !== undefined ? { name: a.name } : {}),
              ...(a.entityType !== undefined ? { type: a.entityType } : {}),
            },
          } as any);
          await repo.update(id, next as any);
          const evt = { kind: 'entity.setCore', payload: next };
          publish(bus, 'entity.setCore', evt);
          events.push(evt);
        }
      } catch (err) {
        errors.push(err);
      }
    }

    return {
      actions,
      snapshot,
      commitResult: {
        success: errors.length === 0,
        errors: errors.length ? errors : undefined,
        events,
      },
    } as const;
  }
}

export const DefaultThingDriver = new ThingDriver();

export async function processEntities(
  entities: EntityInput[],
  opts?: {
    repo?: Repository<Entity>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultThingDriver.processEntities(entities, opts);
}

export async function commitEntities(
  entities: EntityInput[],
  opts?: {
    repo?: Repository<Entity>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultThingDriver.commitEntities(entities, opts);
}

// Fix the publish function - it's not emitting events correctly:
function publish(bus: EventBus | undefined, kind: string, evt: any) {
  if (!kind) return;
  if (!bus) return;
  const anyBus = bus as any;
  // The bus expects just the event object, not nested in another wrapper
  if (typeof anyBus.publish === 'function') anyBus.publish(evt);
  else if (typeof anyBus.emit === 'function') anyBus.emit(evt);
}
