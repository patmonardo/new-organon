import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import { z } from 'zod';
import { FormAspect } from './aspect-form';
import { BaseState } from '@schema';
import * as active from '@schema';
import type { DialecticEvaluateCmd, DialecticCommand } from '@schema';
import { AspectSchema, type AspectShapeRepo } from '@schema/aspect';

type BaseStateShape = z.infer<typeof BaseState>;

type AspectCreateCmd = {
  kind: 'aspect.create';
  payload: Parameters<typeof FormAspect.create>[0];
  meta?: Record<string, unknown>;
};
type AspectDeleteCmd = {
  kind: 'aspect.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type AspectDescribeCmd = {
  kind: 'aspect.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type AspectSetCoreCmd = {
  kind: 'aspect.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type AspectSetStateCmd = {
  kind: 'aspect.setState';
  payload: { id: string; state: BaseStateShape };
  meta?: Record<string, unknown>;
};
type AspectPatchStateCmd = {
  kind: 'aspect.patchState';
  payload: { id: string; patch: Partial<BaseStateShape> };
  meta?: Record<string, unknown>;
};

export type AspectCommand =
  | AspectCreateCmd
  | AspectDeleteCmd
  | AspectDescribeCmd
  | AspectSetCoreCmd
  | AspectSetStateCmd
  | AspectPatchStateCmd
  | DialecticCommand;

export type AspectStore = {
  getAspectById(id: string): Promise<AspectShapeRepo | null>;
  saveAspect(data: Partial<AspectShapeRepo>): Promise<AspectShapeRepo>;
  deleteAspect?(id: string): Promise<boolean>;
};

export class AspectEngine {
  constructor(
    private readonly repo: AspectStore,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'aspect',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getAspect(id: string): Promise<FormAspect | undefined> {
    const doc = await this.repo.getAspectById(id);
    return doc ? FormAspect.fromRecord(AspectSchema.parse(doc)) : undefined;
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private async mustGet(id: string): Promise<FormAspect> {
    const a = await this.getAspect(id);
    if (!a) throw new Error(`Aspect not found: ${id}`);
    return a;
  }

  private async persist(a: FormAspect) {
    await this.repo.saveAspect(AspectSchema.parse(a.toSchema()));
  }

  async handle(cmd: AspectCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'AspectEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'aspect.create': {
        const { payload } = cmd as AspectCreateCmd;
        const a = FormAspect.create(payload as any);
        await this.persist(a);
        return [
          this.emit(base, 'aspect.created', {
            id: a.id,
            type: a.type,
            name: a.name ?? null,
          }),
        ];
      }

      case 'aspect.delete': {
        const { id } = (cmd as AspectDeleteCmd).payload;
        const existed = await this.repo.getAspectById(id);
        if (existed && this.repo.deleteAspect) {
          await this.repo.deleteAspect(id);
        }
        return [this.emit(base, 'aspect.deleted', { id, ok: !!existed })];
      }

      case 'aspect.setCore': {
        const { id, name, type } = (cmd as AspectSetCoreCmd).payload;
        const a = await this.mustGet(id);
        a.setCore({ name, type });
        await this.persist(a);
        return [
          this.emit(base, 'aspect.setCore', {
            id,
            name: a.name ?? null,
            type: a.type,
          }),
        ];
      }

      case 'aspect.setState': {
        const { id, state } = (cmd as AspectSetStateCmd).payload;
        const a = await this.mustGet(id);
        a.setState(state);
        await this.persist(a);
        return [this.emit(base, 'aspect.setState', { id })];
      }

      case 'aspect.patchState': {
        const { id, patch } = (cmd as AspectPatchStateCmd).payload;
        const a = await this.mustGet(id);
        a.patchState(patch);
        await this.persist(a);
        return [this.emit(base, 'aspect.patchState', { id })];
      }

      case 'aspect.describe': {
        const { id } = (cmd as AspectDescribeCmd).payload;
        const doc = await this.repo.getAspectById(id);
        if (!doc) {
          return [this.emit(base, 'aspect.describe', { id })];
        }

        const a = FormAspect.fromRecord(AspectSchema.parse(doc));
        return [
          this.emit(base, 'aspect.describe', {
            id,
            type: a.type,
            name: a.name ?? null,
            state: doc.state,
            signatureKeys: Object.keys(
              (doc.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(doc.facets ?? {}),
          }),
        ];
      }

      // --- Dialectic EVAL: Aspect as Essential Relation / Spectral ---
      // Aspect is Hegel's RELATION in Appearance - the essential connection.
      // "Spect" = to look, to appear (spectacle, spectrum, aspect).
      // Aspect is the SPECTRAL moment - how grounded facts step into existence.
      // It connects Entity <-> Property through the lens of Ground (Morph).
      // This is the appearing of relations - Fichtean Science's essential structure.
      case 'dialectic.evaluate': {
        const { dialecticState, context: evalContext } = (
          cmd as DialecticEvaluateCmd
        ).payload;

        // Create an Aspect to represent this dialectic relation
        const aspect = FormAspect.create({
          id: dialecticState.id,
          type: dialecticState.concept,
          name: dialecticState.title,
        });

        // Extract RELATIONAL STRUCTURE from moments
        // 'opposite', 'mediates', 'transforms', 'negates' show the essential relations
        const relationalMoments = dialecticState.moments.filter(
          (m) => m.relation !== undefined,
        );

        const relations = relationalMoments.map((m) => ({
          from: m.name,
          to: m.relatedTo,
          relation: m.relation,
          type: m.type,
        }));

        // Extract SPECTRAL POLES from polarities
        // Polarities are the "spectrum" - the range of appearing
        const polarities = dialecticState.moments.filter(
          (m) => m.type === 'polarity' || m.type === 'negation',
        );

        const spectrum = {
          poles: polarities.map((p) => ({
            name: p.name,
            definition: p.definition,
            oppositeTo: p.relatedTo,
          })),
          range: polarities.length,
          dialectical: polarities.some((p) => p.relation === 'opposite'),
        };

        // Extract APPEARING STRUCTURE from forces
        // Forces show how the relation "appears" - steps into existence
        const appearingForces = (dialecticState.forces ?? []).filter(
          (f) =>
            f.type === 'externality' ||
            f.type === 'reflection' ||
            f.type === 'passover',
        );

        const appearing = {
          mode:
            appearingForces.length > 0 ? appearingForces[0].type : 'immanent',
          triggers: appearingForces.map((f) => f.trigger),
          effects: appearingForces.map((f) => f.effect),
        };

        // The Essential Relation connects Entity (Thing) <-> Property (Law)
        // Through the Spectral lens of Ground (Morph)
        const essentialRelation = {
          // The spectrum of appearing
          spectrum,
          // The relational connections
          connections: relations,
          // How it appears
          appearing,
          // Grounded in (reference to Morph/Ground)
          groundedIn:
            evalContext?.groundId ?? dialecticState.previousStates?.[0],
        };

        // Store in signature: the relational moments
        const signature = dialecticState.moments.reduce((acc, m) => {
          acc[m.name] = {
            definition: m.definition,
            type: m.type,
            relation: m.relation,
            relatedTo: m.relatedTo,
            spectral: m.type === 'polarity' || m.type === 'negation',
          };
          return acc;
        }, {} as Record<string, any>);

        aspect.setSignature(signature);

        // Store in facets: the Spectral/Relational structure
        aspect.setFacets({
          dialecticState: dialecticState,
          phase: dialecticState.phase,
          // The Essential Relation structure
          essentialRelation,
          // Spectrum of poles
          spectrum,
          // Relational connections
          relations,
          // Appearing mode
          appearing,
          // Invariants as constraints on the relation
          constraints: dialecticState.invariants.map((inv) => ({
            id: inv.id,
            constraint: inv.constraint,
            predicate: inv.predicate,
          })),
          // Evaluation context
          context: evalContext,
        });

        // Set state to reflect the spectral aspect
        aspect.setState({
          status: 'active',
          meta: {
            isSpectral: true,
            poleCount: spectrum.poles.length,
            relationCount: relations.length,
            appearingMode: appearing.mode,
          },
        } as any);

        await this.persist(aspect);

        return [
          this.emit(base, 'dialectic.evaluated', {
            stateId: dialecticState.id,
            concept: dialecticState.concept,
            phase: dialecticState.phase,
            kind: 'aspect',
            poleCount: spectrum.poles.length,
            relationCount: relations.length,
            appearingMode: appearing.mode,
            isDialectical: spectrum.dialectical,
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-like interface to mirror other engines
  async process(
    aspects: Array<active.ActiveAspect>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = active.parseActiveAspects(aspects);
    const actions: any[] = [];
    for (const a of list) {
      if (a.revoked === true) {
        if (a.id) actions.push({ type: 'aspect.delete', id: a.id });
        continue;
      }
      actions.push({
        type: 'aspect.upsert',
        id: a.id ?? childIdFromName(a.name),
        name: a.name,
        aspectType: (a as any).kind ?? 'system.Aspect',
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'aspect.delete') {
        const [evt] = await this.handle({
          kind: 'aspect.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'aspect.upsert') {
        const id = a.id as string;
        const existing = await this.getAspect(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'aspect.create',
            payload: { id, type: a.aspectType, name: a.name },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'aspect.setCore',
            payload: { id, type: a.aspectType, name: a.name },
          } as any);
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
  return `aspect:${base}`;
}
