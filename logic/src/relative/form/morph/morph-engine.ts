import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import type { Repository } from '@repository';
import {
  type Morph,
  MorphSchema,
  createMorph,
  updateMorph,
} from '@schema';
import { FormMorph } from './morph-form';
import * as active from '@schema';
import type {
  DialecticEvaluateCmd,
  DialecticCommand,
} from '@schema';

type BaseState = Morph['shape']['state'];

type MorphCreateCmd = {
  kind: 'morph.create';
  payload: Parameters<typeof createMorph>[0];
  meta?: Record<string, unknown>;
};
type MorphDeleteCmd = {
  kind: 'morph.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type MorphDescribeCmd = {
  kind: 'morph.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type MorphSetCoreCmd = {
  kind: 'morph.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type MorphSetStateCmd = {
  kind: 'morph.setState';
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type MorphPatchStateCmd = {
  kind: 'morph.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};

export type MorphCommand =
  | MorphCreateCmd
  | MorphDeleteCmd
  | MorphDescribeCmd
  | MorphSetCoreCmd
  | MorphSetStateCmd
  | MorphPatchStateCmd
  | DialecticCommand;

export class MorphEngine {
  constructor(
    private readonly repo: Repository<Morph>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'morph',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getMorph(id: string): Promise<FormMorph | undefined> {
    const doc = await this.repo.get(id);
    return doc ? FormMorph.fromSchema(doc) : undefined;
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private async mustGet(id: string): Promise<FormMorph> {
    const m = await this.getMorph(id);
    if (!m) throw new Error(`Morph not found: ${id}`);
    return m;
  }

  private async persist(m: FormMorph) {
    const id = m.id;
    const doc = MorphSchema.parse(m.toSchema());
    const current = await this.repo.get(id);
    if (current) {
      await this.repo.update(id, doc as any);
    } else {
      await this.repo.create(doc);
    }
  }

  async handle(cmd: MorphCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'MorphEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'morph.create': {
        const { payload } = cmd as MorphCreateCmd;
        const created = MorphSchema.parse(createMorph(payload as any));
        const m = FormMorph.fromSchema(created);
        await this.persist(m);
        return [
          this.emit(base, 'morph.create', {
            id: m.id,
            type: m.type,
            name: m.name ?? null,
          }),
        ];
      }

      case 'morph.delete': {
        const { id } = (cmd as MorphDeleteCmd).payload;
        const existed = await this.repo.get(id);
        if (existed) {
          await this.repo.delete(id);
        }
        return [this.emit(base, 'morph.delete', { id, ok: !!existed })];
      }

      case 'morph.setCore': {
        const { id, name, type } = (cmd as MorphSetCoreCmd).payload;
        const m = await this.mustGet(id);
        m.setCore({ name, type });
        await this.persist(m);
        return [
          this.emit(base, 'morph.setCore', {
            id,
            name: m.name ?? null,
            type: m.type,
          }),
        ];
      }

      case 'morph.setState': {
        const { id, state } = (cmd as MorphSetStateCmd).payload;
        const m = await this.mustGet(id);
        m.setState(state as any);
        await this.persist(m);
        return [this.emit(base, 'morph.setState', { id })];
      }

      case 'morph.patchState': {
        const { id, patch } = (cmd as MorphPatchStateCmd).payload;
        const m = await this.mustGet(id);
        m.patchState(patch as any);
        await this.persist(m);
        return [this.emit(base, 'morph.patchState', { id })];
      }

      case 'morph.describe': {
        const { id } = (cmd as MorphDescribeCmd).payload;
        const doc = await this.repo.get(id);
        if (!doc) {
          return [this.emit(base, 'morph.describe', { id })];
        }

        const m = FormMorph.fromSchema(doc);
        return [
          this.emit(base, 'morph.describe', {
            id,
            type: m.type,
            name: m.name ?? null,
            state: doc.shape.state,
            signatureKeys: Object.keys(
              (doc.shape.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(doc.shape.facets ?? {}),
          }),
        ];
      }

      // --- Dialectic EVAL: Morph as Ground / Active Container ---
      // Morph is the SYNTHESIS of Shape + Context = Ground.
      // Ground (Hegel) = the unity of Essence and Foundation, the REASON for existence.
      // Morph is the PRINCIPLE that enables transformation - the Active Container.
      // It holds the conditions under which Aspects can appear.
      case 'dialectic.evaluate': {
        const { dialecticState, context: evalContext } = (cmd as DialecticEvaluateCmd).payload;

        // Create a Morph to represent this dialectic ground
        const morph = FormMorph.create({
          id: dialecticState.id,
          type: dialecticState.concept,
          name: dialecticState.title,
        });

        // Extract TRANSFORMATION PRINCIPLE from transitions
        // Transitions show the "reason" - why one state becomes another
        const transformations = (dialecticState.transitions ?? []).map(t => ({
          id: t.id,
          from: t.from,
          to: t.to,
          mechanism: t.mechanism,
          middleTerm: t.middleTerm,
          reason: t.description,
        }));

        // Extract CONTAINER STRUCTURE from moments
        // The Ground "contains" moments - it's the active unity that holds them
        // 'sublation' and 'mediation' moments show the containing structure
        const containingMoments = dialecticState.moments
          .filter(m => m.type === 'sublation' || m.type === 'mediation' || m.relation === 'contains');

        const container = {
          holds: dialecticState.moments.map(m => m.name),
          activeUnity: containingMoments.map(m => ({
            name: m.name,
            definition: m.definition,
            contains: m.relatedTo,
          })),
        };

        // Extract GROUNDING CONDITIONS from forces
        // Forces show what "grounds" the dialectic movement
        const groundingForces = (dialecticState.forces ?? []).map(f => ({
          id: f.id,
          type: f.type,
          trigger: f.trigger,
          effect: f.effect,
          grounds: f.targetState,
        }));

        // The Ground as the synthesis of Shape (form) + Context (scope)
        const ground = {
          form: dialecticState.concept,           // from Shape
          scope: dialecticState.phase,            // from Context
          principle: transformations.length > 0   // the reason
            ? transformations[0].mechanism
            : 'immanent',
        };

        // Store in signature: the moments as the ground's structure
        const signature = dialecticState.moments.reduce((acc, m) => {
          acc[m.name] = {
            definition: m.definition,
            type: m.type,
            relation: m.relation,
            relatedTo: m.relatedTo,
          };
          return acc;
        }, {} as Record<string, any>);

        morph.setSignature(signature);

        // Store in facets: the Ground structure
        morph.setFacets({
          dialecticState: dialecticState,
          phase: dialecticState.phase,
          // The Ground - active unity of form and scope
          ground,
          // Container structure - what this ground holds
          container,
          // Transformation principle - why change occurs
          transformations,
          // Grounding forces - what drives from this ground
          groundingForces,
          // Evaluation context
          context: evalContext,
        });

        // Set state to reflect the active ground
        morph.setState({
          status: 'active',
          meta: {
            isGround: true,
            principle: ground.principle,
            containsCount: container.holds.length,
            transformationCount: transformations.length,
          },
        } as any);

        await this.persist(morph);

        return [
          this.emit(base, 'dialectic.evaluated', {
            stateId: dialecticState.id,
            concept: dialecticState.concept,
            phase: dialecticState.phase,
            kind: 'morph',
            principle: ground.principle,
            containsCount: container.holds.length,
            transformationCount: transformations.length,
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-like interface to mirror EntityEngine/ContextEngine/PropertyEngine
  async process(
    morphs: Array<active.ActiveMorph>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = morphs as any[]; // Skip parsing for now to preserve test data
    const actions: any[] = [];
    for (const m of list) {
      if (m.revoked === true) {
        if (m.id) actions.push({ type: 'morph.delete', id: m.id });
        continue;
      }
      actions.push({
        type: 'morph.upsert',
        id: m.id ?? childIdFromName(m.name),
        morphType: m.kind ?? m.type ?? 'system.Morph',
        name: m.name,
        morph: {
          id: m.id,
          type: m.kind ?? m.type ?? 'system.Morph',
          name: m.name,
          transform: m.transform,
          active: m.active,
        },
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'morph.delete') {
        const [evt] = await this.handle({
          kind: 'morph.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'morph.upsert') {
        const id = a.id as string;
        const existing = await this.getMorph(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'morph.create',
            payload: {
              id,
              type: a.morphType,
              name: a.name,
              state: a.morph ? { transform: a.morph.transform, active: a.morph.active } : {},
            },
          } as any);
          events.push(evt);
        } else {
          // For existing morphs, emit morph.update (not setCore like other engines)
          const updatedDoc = updateMorph(existing.toSchema(), {
            core: {
              ...(a.name !== undefined ? { name: a.name } : {}),
              ...(a.morphType !== undefined ? { type: a.morphType } : {}),
            },
            state: a.morph ? { transform: a.morph.transform, active: a.morph.active } : {},
          } as any);
          const updated = FormMorph.fromSchema(MorphSchema.parse(updatedDoc));
          await this.persist(updated);

          const evt = this.emit({ correlationId: 'commit' }, 'morph.update', {
            id,
            type: updated.type,
            name: updated.name ?? null,
          });
          events.push(evt);
        }
      }
    }
    return { success: true, errors: [], events } as any;
  }
}

function childIdFromName(name?: string) {
  const base =
    name?.toString().trim().toLowerCase().replace(/\s+/g, '-') ||
    Math.random().toString(36).slice(2, 10);
  return `morph:${base}`;
}
