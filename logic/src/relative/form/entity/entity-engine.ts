import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import { InMemoryRepository } from '@repository';
import { EntityShapeRepository } from '@repository';
import { toDialecticalInfo, type DiscursiveRuleTag } from '@schema';
import { EntityShapeSchema, type EntityShapeRepo } from '@schema/entity';
import * as active from '@schema';
import { FormEntity } from './entity-form';
import type {
  DialecticEvaluateCmd,
  DialecticInvariantCheckCmd,
  DialecticCommand,
} from '@schema';

type BaseState = {
  status?: string;
  tags?: string[];
  meta?: Record<string, unknown>;
};
type Signature = Record<string, unknown>;
type Facets = Record<string, unknown>;

type EntityCreateCmd = {
  kind: 'entity.create';
  payload: Parameters<typeof FormEntity.create>[0];
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
  | EntityMergeSignatureCmd
  | DialecticCommand;

// Direct wiring: accept EntityShapeRepository (Neo4j) or InMemoryRepository<EntityShape> (testing)
type EntityStore =
  | EntityShapeRepository
  | InMemoryRepository<typeof EntityShapeSchema>;

export class EntityEngine {
  private readonly entities = new Map<string, FormEntity>();

  constructor(
    private readonly repo?: EntityStore,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'entity',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getEntity(id: string): Promise<FormEntity | undefined> {
    const cached = this.entities.get(id);
    if (cached) return cached;
    if (!this.repo) return undefined;

    if (this.repo instanceof EntityShapeRepository) {
      const entityShape = await this.repo.getEntityById(id);
      if (!entityShape) return undefined;
      const ent = this.repoRecordToFormEntity(entityShape as EntityShapeRepo);
      this.entities.set(ent.id, ent);
      return ent;
    }

    if (this.repo instanceof InMemoryRepository) {
      const doc = await this.repo.get(id);
      if (!doc) return undefined;
      const ent = this.repoRecordToFormEntity(doc as EntityShapeRepo);
      this.entities.set(ent.id, ent);
      return ent;
    }

    return undefined;
  }

  private repoRecordToFormEntity(repoEntity: EntityShapeRepo): FormEntity {
    return FormEntity.fromRecord(EntityShapeSchema.parse(repoEntity));
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = {
      ...childSpan(base, { action: kind, scope: this.scope }),
      ...(this.metaFor(kind, payload) ?? {}),
    };
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private metaFor(kind: Event['kind'], payload: Event['payload']) {
    if (
      kind !== 'entity.create' &&
      kind !== 'entity.setCore' &&
      kind !== 'entity.setState'
    ) {
      return undefined;
    }

    const id = (payload as any)?.id as string | undefined;
    if (!id) return undefined;

    const op = kind === 'entity.create' ? 'assert' : 'revise';
    const tags: DiscursiveRuleTag[] = [{ layer: 'entity', rule: 'thing' }];

    return {
      factStore: {
        mode: 'logic',
        store: 'FormDB',
        op,
        kind: 'Entity',
        ids: [id],
      },
      dialectic: toDialecticalInfo(tags),
    };
  }

  private readPath(obj: unknown, path: string): unknown {
    if (!path) return undefined;
    const parts = path.split('.').filter(Boolean);
    let cur: any = obj;
    for (const p of parts) {
      if (cur === null || cur === undefined) return undefined;
      cur = cur[p];
    }
    return cur;
  }

  private evalInvariantPredicate(
    entity: FormEntity,
    predicate: string | undefined,
  ): { ok: boolean; reason?: string } {
    if (!predicate) {
      // Seed behavior: if no formal predicate, treat as satisfied (caller can still log constraint text).
      return { ok: true };
    }

    // Mini predicate DSL (seed):
    // - exists:<path>
    // - eq:<path>:<json-or-string>
    const trimmed = predicate.trim();

    if (trimmed.startsWith('exists:')) {
      const path = trimmed.slice('exists:'.length).trim();
      const v = this.readPath(entity.toRecord(), path);
      const ok = v !== undefined && v !== null;
      return ok ? { ok } : { ok, reason: `missing value at ${path}` };
    }

    if (trimmed.startsWith('eq:')) {
      const rest = trimmed.slice('eq:'.length);
      const idx = rest.indexOf(':');
      if (idx <= 0)
        return {
          ok: false,
          reason: 'invalid eq predicate (expected eq:<path>:<value>)',
        };
      const path = rest.slice(0, idx).trim();
      const raw = rest.slice(idx + 1).trim();
      let expected: any = raw;
      try {
        expected = JSON.parse(raw);
      } catch {
        // keep as string
      }
      const actual = this.readPath(entity.toRecord(), path);
      const ok = actual === expected;
      return ok
        ? { ok }
        : {
            ok,
            reason: `expected ${path} === ${JSON.stringify(
              expected,
            )}; got ${JSON.stringify(actual)}`,
          };
    }

    // Unknown predicate form: seed treats as not evaluable => violated.
    return { ok: false, reason: `unknown predicate: ${trimmed}` };
  }

  private async mustGet(id: string): Promise<FormEntity> {
    const e = await this.getEntity(id);
    if (!e) throw new Error(`Entity not found: ${id}`);
    return e;
  }

  private async persist(e: FormEntity) {
    if (!this.repo) return;

    const entityShape = this.formEntityToRepoRecord(e);

    // Check if it's EntityShapeRepository (Neo4j)
    if (this.repo instanceof EntityShapeRepository) {
      await this.repo.saveEntity(entityShape);
      return;
    }

    // Otherwise it's InMemoryRepository
    if (this.repo instanceof InMemoryRepository) {
      const id = e.id;
      const current = await this.repo.get(id);
      if (current) {
        await this.repo.update(id, entityShape as any);
      } else {
        await this.repo.create(entityShape as any);
      }
    }
  }

  private formEntityToRepoRecord(formEntity: FormEntity): EntityShapeRepo {
    return formEntity.toRecord();
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
        const ent = FormEntity.create(payload as any);
        this.entities.set(ent.id, ent);
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
        let existed = false;
        if (this.repo) {
          if (this.repo instanceof EntityShapeRepository) {
            existed = await this.repo.deleteEntity(id);
          } else if (this.repo instanceof InMemoryRepository) {
            existed = await this.repo.delete(id);
          }
        }
        return [this.emit(base, 'entity.delete', { id, ok: existed })];
      }

      case 'entity.setCore': {
        const { id, name, type } = (cmd as EntitySetCoreCmd).payload;
        const e = await this.mustGet(id);
        e.setCore({ name, type });
        this.entities.set(e.id, e);
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
        this.entities.set(e.id, e);
        await this.persist(e);
        return [this.emit(base, 'entity.setState', { id })];
      }

      case 'entity.patchState': {
        const { id, patch } = (cmd as EntityPatchStateCmd).payload;
        const e = await this.mustGet(id);
        e.patchState(patch as any);
        this.entities.set(e.id, e);
        await this.persist(e);
        return [this.emit(base, 'entity.patchState', { id })];
      }

      case 'entity.setFacets': {
        const { id, facets } = (cmd as EntitySetFacetsCmd).payload;
        const e = await this.mustGet(id);
        e.setFacets(facets as any);
        this.entities.set(e.id, e);
        await this.persist(e);
        return [this.emit(base, 'entity.setFacets', { id })];
      }

      case 'entity.mergeFacets': {
        const { id, patch } = (cmd as EntityMergeFacetsCmd).payload;
        const e = await this.mustGet(id);
        e.mergeFacets(patch as any);
        this.entities.set(e.id, e);
        await this.persist(e);
        return [this.emit(base, 'entity.mergeFacets', { id })];
      }

      case 'entity.setSignature': {
        const { id, signature } = (cmd as EntitySetSignatureCmd).payload;
        const e = await this.mustGet(id);
        e.setSignature(signature as any);
        this.entities.set(e.id, e);
        await this.persist(e);
        return [this.emit(base, 'entity.setSignature', { id })];
      }

      case 'entity.mergeSignature': {
        const { id, patch } = (cmd as EntityMergeSignatureCmd).payload;
        const e = await this.mustGet(id);
        e.mergeSignature(patch as any);
        this.entities.set(e.id, e);
        await this.persist(e);
        return [this.emit(base, 'entity.mergeSignature', { id })];
      }

      case 'entity.describe': {
        const { id } = (cmd as EntityDescribeCmd).payload;

        // Load from repository
        const e = await this.getEntity(id);
        if (!e) {
          return [this.emit(base, 'entity.describe', { id })];
        }

        return [
          this.emit(base, 'entity.describe', {
            id,
            type: e.type,
            name: e.name ?? null,
            state: {
              status: e.status,
              tags: e.tags,
              meta: e.meta,
            },
            signatureKeys: Object.keys(
              (e.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(e.facets ?? {}),
          }),
        ];
      }

      case 'dialectic.invariant.check': {
        const { stateId, invariants } = (cmd as DialecticInvariantCheckCmd)
          .payload;
        const e = await this.mustGet(stateId);

        const violations: Event[] = [];
        for (const inv of invariants) {
          const res = this.evalInvariantPredicate(e, (inv as any)?.predicate);
          if (!res.ok) {
            violations.push(
              this.emit(base, 'dialectic.invariant.violated', {
                stateId,
                invariant: (inv as any)?.id ?? 'invariant.unknown',
                reason: res.reason ?? 'Constraint not satisfied',
              }),
            );
          }
        }

        if (violations.length > 0) return violations;

        return [
          this.emit(base, 'dialectic.invariant.satisfied', {
            stateId,
            count: invariants.length,
          }),
        ];
      }

      case 'dialectic.evaluate': {
        const { dialecticState, context } = (cmd as DialecticEvaluateCmd)
          .payload;

        // Create entity from dialectic state
        // We use the dialectic state ID as the entity ID, or generate a new one?
        // The plan says "id: dialecticState.id", which implies 1:1 mapping.
        // But Entities are instances. Maybe we should allow ID override or gen new one.
        // For now, let's follow the plan but maybe suffix it if it's an instance?
        // Actually, if we are "evaluating" a state to produce an entity, it's like "instantiating" it.
        // Let's use the state ID for now as the "Concept Entity".

        // Note: formId must be provided when creating entity
        // For dialectic evaluation, we need a formId - this should come from context or be provided
        // For now, throw if not provided in context
        const formId = (context as any)?.formId;
        if (!formId) {
          throw new Error(
            'Entity creation from dialectic state requires formId in context',
          );
        }

        const entity = FormEntity.create({
          id: dialecticState.id,
          type: dialecticState.concept,
          name: dialecticState.title,
          description: dialecticState.description,
          formId: formId,
        });

        // Store dialectic state in facets
        entity.setFacets({
          dialecticState: dialecticState,
          phase: dialecticState.phase,
          moments: dialecticState.moments,
          invariants: dialecticState.invariants,
          context: context,
        });

        // Store moments as signature
        const momentsSignature = dialecticState.moments.reduce((acc, m) => {
          acc[m.name] = {
            definition: m.definition,
            type: m.type,
            value: null, // Runtime value to be set
          };
          return acc;
        }, {} as Record<string, any>);

        entity.setSignature(momentsSignature);

        await this.persist(entity);

        return [
          this.emit(base, 'dialectic.evaluated', {
            stateId: entity.id, // Using entity ID as state ID here
            concept: dialecticState.concept,
            phase: dialecticState.phase,
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
