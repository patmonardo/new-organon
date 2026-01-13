import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import { InMemoryRepository } from '@repository';
import { FormShapeRepository } from '@repository';
import type { FormShapeRepo } from '@schema/form';
import { FormShapeSchema } from '@schema/form';
import { FormShape } from './shape-form';
import { ActiveShape } from '@schema';
import { parseActiveShapes } from '@schema';
import {
  inferReflectionRulesFromCrudFormShape,
  toDialecticalInfo,
  type DiscursiveRuleTag,
} from '@schema';
import type {
  DialecticStateTransitionCmd,
  DialecticMomentActivateCmd,
  DialecticForceApplyCmd,
  DialecticInvariantCheckCmd,
  DialecticEvaluateCmd,
  DialecticCommand,
} from '@schema';

type BaseState = Record<string, unknown>;
type Signature = Record<string, unknown>;
type Facets = Record<string, unknown>;

type ShapeCreateCmd = {
  kind: 'shape.create';
  payload: Parameters<typeof FormShape.create>[0];
  meta?: Record<string, unknown>;
};
type ShapeDeleteCmd = {
  kind: 'shape.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type ShapeSetCoreCmd = {
  kind: 'shape.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type ShapeSetSignatureCmd = {
  kind: 'shape.setSignature';
  payload: { id: string; signature?: Signature };
  meta?: Record<string, unknown>;
};
type ShapeMergeSignatureCmd = {
  kind: 'shape.mergeSignature';
  payload: { id: string; patch: Record<string, unknown> };
  meta?: Record<string, unknown>;
};
type ShapeSetFacetsCmd = {
  kind: 'shape.setFacets';
  payload: { id: string; facets: Facets };
  meta?: Record<string, unknown>;
};
type ShapeMergeFacetsCmd = {
  kind: 'shape.mergeFacets';
  payload: { id: string; patch: Record<string, unknown> };
  meta?: Record<string, unknown>;
};
type ShapeSetStateCmd = {
  kind: 'shape.setState';
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type ShapePatchStateCmd = {
  kind: 'shape.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};
type ShapeDescribeCmd = {
  kind: 'shape.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ShapeCommand =
  | ShapeCreateCmd
  | ShapeDeleteCmd
  | ShapeSetCoreCmd
  | ShapeSetSignatureCmd
  | ShapeMergeSignatureCmd
  | ShapeSetFacetsCmd
  | ShapeMergeFacetsCmd
  | ShapeSetStateCmd
  | ShapePatchStateCmd
  | ShapeDescribeCmd
  | DialecticCommand;

// Direct wiring: accept FormShapeRepository (Neo4j) or InMemoryRepository<FormShape> (testing)
type FormShapeStore =
  | FormShapeRepository
  | InMemoryRepository<typeof FormShapeSchema>;

export class ShapeEngine {
  private readonly shapes = new Map<string, FormShape>();

  constructor(
    private readonly repo?: FormShapeStore,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'shape',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getShape(id: string): Promise<FormShape | undefined> {
    // Check in-memory cache first
    const cached = this.shapes.get(id);
    if (cached) return cached;

    // Load from repository if available
    if (this.repo) {
      const loaded = await this.loadFromRepo(id);
      if (loaded) {
        this.shapes.set(id, loaded);
        return loaded;
      }
    }

    return undefined;
  }

  private async loadFromRepo(id: string): Promise<FormShape | undefined> {
    if (!this.repo) return undefined;

    // Check if it's FormShapeRepository (Neo4j)
    if (this.repo instanceof FormShapeRepository) {
      const repoShape = await this.repo.getFormById(id);
      if (!repoShape) return undefined;
      // Convert repository FormShape to dialectical FormShape class
      return this.repoRecordToFormShape(repoShape as FormShapeRepo);
    }

    // Otherwise it's InMemoryRepository
    if (this.repo instanceof InMemoryRepository) {
      const doc = await this.repo.get(id);
      if (!doc) return undefined;
      // InMemoryRepository stores FormShape schema type
      return this.repoRecordToFormShape(doc as FormShapeRepo);
    }

    return undefined;
  }

  private repoRecordToFormShape(repoShape: FormShapeRepo): FormShape {
    // Convert repository FormShape to dialectical FormShape class
    // FormShape class now works directly with FormShapeSchema
    return FormShape.fromRecord(repoShape);
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
      kind !== 'shape.create' &&
      kind !== 'shape.setCore' &&
      kind !== 'shape.setState'
    ) {
      return undefined;
    }

    const id = (payload as any)?.id as string | undefined;
    if (!id) return undefined;

    const op = kind === 'shape.create' ? 'assert' : 'revise';

    const tags: DiscursiveRuleTag[] = [];
    const add = (t: DiscursiveRuleTag) => {
      if (!tags.some((x) => x.layer === t.layer && x.rule === t.rule))
        tags.push(t);
    };

    // Minimal explicit mapping by engine verb
    if (kind === 'shape.create') add({ layer: 'shape', rule: 'posting' });
    if (kind === 'shape.setCore') add({ layer: 'shape', rule: 'external' });
    if (kind === 'shape.setState') add({ layer: 'shape', rule: 'determining' });

    // Heuristic lift from the underlying CRUD-ish shape structure (fields/layout/name)
    const s = this.shapes.get(id);
    const inferred = inferReflectionRulesFromCrudFormShape(s?.toRecord());
    for (const r of inferred) add({ layer: 'shape', rule: r });

    return {
      factStore: {
        mode: 'reflection',
        store: 'FormDB',
        op,
        kind: 'FormShape',
        ids: [id],
      },
      dialectic: toDialecticalInfo(tags),
    };
  }

  private async mustGet(id: string): Promise<FormShape> {
    const s = await this.getShape(id);
    if (!s) throw new Error(`Shape not found: ${id}`);
    return s;
  }

  private async persist(s: FormShape) {
    if (!this.repo) return;

    // Convert dialectical FormShape to repository FormShape
    const repoShape = this.formShapeToRepoRecord(s);

    // Check if it's FormShapeRepository (Neo4j)
    if (this.repo instanceof FormShapeRepository) {
      await this.repo.saveForm(repoShape);
      return;
    }

    // Otherwise it's InMemoryRepository
    if (this.repo instanceof InMemoryRepository) {
      const id = s.id;
      const current = await this.repo.get(id);
      if (current) {
        await this.repo.update(id, repoShape as any);
      } else {
        await this.repo.create(repoShape as any);
      }
    }
  }

  private formShapeToRepoRecord(formShape: FormShape): FormShapeRepo {
    // Convert dialectical FormShape class to repository FormShape type
    // FormShape class now works directly with FormShapeSchema, so just return it
    return formShape.toRecord();
  }

  async handle(cmd: ShapeCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'ShapeEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'shape.create': {
        const { payload } = cmd as ShapeCreateCmd;
        const shape = FormShape.create(payload);
        this.shapes.set(shape.id, shape);
        await this.persist(shape);
        return [
          this.emit(base, 'shape.create', {
            id: shape.id,
            type: shape.type,
            name: shape.name ?? null,
          }),
        ];
      }

      case 'shape.delete': {
        const { id } = (cmd as ShapeDeleteCmd).payload;
        const existed = this.shapes.delete(id);
        if (this.repo) {
          if (this.repo instanceof FormShapeRepository) {
            await this.repo.deleteForm(id);
          } else if (this.repo instanceof InMemoryRepository) {
            await this.repo.delete(id);
          }
        }
        return [this.emit(base, 'shape.delete', { id, ok: existed })];
      }

      case 'shape.setCore': {
        const { id, name, type } = (cmd as ShapeSetCoreCmd).payload;
        const s = await this.mustGet(id);
        if (name !== undefined) s.setName(name);
        if (type !== undefined) s.setType(type);
        await this.persist(s);
        return [
          this.emit(base, 'shape.setCore', {
            id,
            name: s.name ?? null,
            type: s.type,
          }),
        ];
      }

      case 'shape.setSignature': {
        const { id, signature } = (cmd as ShapeSetSignatureCmd).payload;
        const s = await this.mustGet(id);
        s.setSignature(signature);
        await this.persist(s);
        return [this.emit(base, 'shape.setSignature', { id })];
      }

      case 'shape.mergeSignature': {
        const { id, patch } = (cmd as ShapeMergeSignatureCmd).payload;
        const s = await this.mustGet(id);
        s.patchSignature(patch);
        await this.persist(s);
        return [this.emit(base, 'shape.mergeSignature', { id })];
      }

      case 'shape.setFacets': {
        const { id, facets } = (cmd as ShapeSetFacetsCmd).payload;
        const s = await this.mustGet(id);
        s.setFacets(facets);
        await this.persist(s);
        return [this.emit(base, 'shape.setFacets', { id })];
      }

      case 'shape.mergeFacets': {
        const { id, patch } = (cmd as ShapeMergeFacetsCmd).payload;
        const s = await this.mustGet(id);
        s.mergeFacets(patch);
        await this.persist(s);
        return [this.emit(base, 'shape.mergeFacets', { id })];
      }

      case 'shape.setState': {
        const { id, state } = (cmd as ShapeSetStateCmd).payload;
        const s = await this.mustGet(id);
        s.setState(state);
        await this.persist(s);
        return [
          this.emit(base, 'shape.setState', { id, status: s.state.status }),
        ];
      }

      case 'shape.patchState': {
        const { id, patch } = (cmd as ShapePatchStateCmd).payload;
        const s = await this.mustGet(id);
        s.patchState(patch);
        await this.persist(s);
        return [
          this.emit(base, 'shape.patchState', { id, status: s.state.status }),
        ];
      }

      case 'shape.describe': {
        const { id } = (cmd as ShapeDescribeCmd).payload;
        const s = await this.mustGet(id);
        return [
          this.emit(base, 'shape.describe', {
            id,
            type: s.type,
            name: s.name ?? null,
            state: s.state,
            signatureKeys: Object.keys(
              (s.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(s.facets ?? {}),
          }),
        ];
      }

      // --- Dialectic IR Handlers ---

      case 'dialectic.state.transition': {
        const { fromStateId, toStateId, dialecticState } = (
          cmd as DialecticStateTransitionCmd
        ).payload;

        // Get the current shape (represents fromState)
        const fromShape = await this.mustGet(fromStateId);
        const fromDialecticState = fromShape.getDialecticState();

        if (!fromDialecticState) {
          throw new Error(`Source shape ${fromStateId} has no dialectic state`);
        }

        // Apply transition logic
        const transition = fromDialecticState.transitions?.find(
          (t) => t.to === toStateId,
        );
        if (!transition) {
          throw new Error(`No transition from ${fromStateId} to ${toStateId}`);
        }

        // Store NEW dialecticState in facets of TO shape (not from shape)
        // Note: The original code merged it into fromShape, which might be wrong if dialecticState is the target.
        // But wait, the original code said:
        // fromShape.mergeFacets({ dialecticState: dialecticState ... })
        // If dialecticState is the TARGET state, we shouldn't merge it into FROM shape.

        // Let's fix that too. We should only update TO shape with the new state.

        // Create or update target shape
        let toShape = await this.getShape(toStateId);
        if (!toShape) {
          toShape = FormShape.create({
            id: toStateId,
            type: dialecticState.concept,
            name: dialecticState.title,
          });
          this.shapes.set(toStateId, toShape);
        }

        // Store dialecticState in facets of target shape
        toShape.mergeFacets({
          dialecticState: dialecticState,
          currentPhase: dialecticState.phase,
        });

        // Store moments in signature
        const momentsSignature = dialecticState.moments.reduce((acc, m) => {
          acc[m.name] = {
            definition: m.definition,
            type: m.type,
            relation: m.relation,
            relatedTo: m.relatedTo,
          };
          return acc;
        }, {} as Record<string, any>);

        toShape.setSignature(momentsSignature);

        await this.persist(fromShape);
        await this.persist(toShape);

        return [
          this.emit(base, 'dialectic.state.transitioned', {
            fromState: fromStateId,
            toState: toStateId,
            mechanism: transition.mechanism,
          }),
        ];
      }

      case 'dialectic.moment.activate': {
        const { stateId, moment } = (cmd as DialecticMomentActivateCmd).payload;
        const shape = await this.mustGet(stateId);

        // Store moment in signature
        shape.patchSignature({
          [moment.name]: {
            definition: moment.definition,
            type: moment.type,
            active: true,
          },
        });

        await this.persist(shape);

        return [
          this.emit(base, 'dialectic.moment.activated', {
            stateId,
            moment: moment.name,
          }),
        ];
      }

      case 'dialectic.force.apply': {
        const { stateId, force } = (cmd as DialecticForceApplyCmd).payload;
        const shape = await this.mustGet(stateId);

        // Check trigger condition (simplified - would need proper evaluation)
        const triggerMet = true; // TODO: Evaluate force.trigger

        if (triggerMet) {
          // Apply effect
          shape.patchState({
            meta: {
              ...(shape.state.meta as object),
              lastForceApplied: force.id,
              forceEffect: force.effect,
            },
          });

          await this.persist(shape);

          return [
            this.emit(base, 'dialectic.force.applied', {
              stateId,
              force: force.id,
              effect: force.effect,
              targetState: force.targetState,
            }),
          ];
        }

        return [];
      }

      case 'dialectic.invariant.check': {
        const { stateId, invariants } = (cmd as DialecticInvariantCheckCmd)
          .payload;
        const shape = await this.mustGet(stateId);

        const violations: Event[] = [];

        for (const inv of invariants) {
          // TODO: Proper predicate evaluation
          // For now, just store in facets
          const satisfied = true; // Placeholder

          if (!satisfied) {
            violations.push(
              this.emit(base, 'dialectic.invariant.violated', {
                stateId,
                invariant: inv.id,
                reason: 'Constraint not satisfied',
              }),
            );
          }
        }

        if (violations.length > 0) {
          return violations;
        }

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

        // Create a shape to represent this dialectic state
        const shape = FormShape.create({
          id: dialecticState.id,
          type: dialecticState.concept,
          name: dialecticState.title,
        });

        // Store full dialectic state in facets
        shape.setFacets({
          dialecticState: dialecticState,
          phase: dialecticState.phase,
          context: context,
        });

        // Store moments in signature
        const momentsSignature = dialecticState.moments.reduce((acc, m) => {
          acc[m.name] = {
            definition: m.definition,
            type: m.type,
          };
          return acc;
        }, {} as Record<string, any>);

        shape.setSignature(momentsSignature);

        this.shapes.set(shape.id, shape);
        await this.persist(shape);

        return [
          this.emit(base, 'dialectic.evaluated', {
            stateId: dialecticState.id,
            concept: dialecticState.concept,
            phase: dialecticState.phase,
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR 0002 ShapeEngine interface: process/commit
  async process(
    shapes: Array<ActiveShape>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    // Validate/normalize via Zod; this clamps confidence and checks ids
    const list = parseActiveShapes(shapes);
    // For now, produce deterministic actions that map to our command space
    const actions: any[] = [];
    for (const s of list) {
      // Upsert behavior based on revoked/active
      if (s.revoked === true) {
        if (s.id)
          actions.push({ type: 'shape.delete', id: s.id, sourceShapeId: s.id });
        continue;
      }
      actions.push({
        type: 'shape.upsert',
        id: s.id ?? childIdFromName(s.name),
        name: s.name,
        kind: s.kind ?? 'shape',
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'shape.delete') {
        const [evt] = await this.handle({
          kind: 'shape.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'shape.upsert') {
        // Upsert → create or set core
        const id = a.id as string;
        if (!(await this.getShape(id))) {
          const [evt] = await this.handle({
            kind: 'shape.create',
            payload: { id, type: a.kind, name: a.name },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'shape.setCore',
            payload: { id, type: a.kind, name: a.name },
          } as any);
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}

function childIdFromName(name?: string) {
  if (!name) return `shape:${Math.random().toString(36).slice(2, 10)}`;
  return `shape:${name.toLowerCase().replace(/\s+/g, '-')}`;
}
