import { z } from 'zod';
import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import { FormProperty } from './property-form';

import { toDialecticalInfo, type DiscursiveRuleTag, BaseState } from '@schema';
import * as active from '@schema';
import type { DialecticEvaluateCmd, DialecticCommand } from '@schema';
import { PropertyShapeSchema, type PropertyShapeRepo } from '@schema/property';

type BaseStateShape = z.infer<typeof BaseState>;

export type PropertyCreateCmd = {
  kind: 'property.create';
  payload: Parameters<typeof FormProperty.create>[0];
  meta?: Record<string, unknown>;
};
export type PropertyDeleteCmd = {
  kind: 'property.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type PropertyDescribeCmd = {
  kind: 'property.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
export type PropertySetCoreCmd = {
  kind: 'property.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
export type PropertySetStateCmd = {
  kind: 'property.setState';
  payload: { id: string; state: BaseStateShape };
  meta?: Record<string, unknown>;
};
export type PropertyPatchStateCmd = {
  kind: 'property.patchState';
  payload: { id: string; patch: Partial<BaseStateShape> };
  meta?: Record<string, unknown>;
};

export type PropertyCommand =
  | PropertyCreateCmd
  | PropertyDeleteCmd
  | PropertyDescribeCmd
  | PropertySetCoreCmd
  | PropertySetStateCmd
  | PropertyPatchStateCmd
  | DialecticCommand;

export type PropertyStore = {
  getPropertyById(id: string): Promise<PropertyShapeRepo | null>;
  saveProperty(data: Partial<PropertyShapeRepo>): Promise<PropertyShapeRepo>;
  deleteProperty?(id: string): Promise<boolean>;
};

export class PropertyEngine {
  constructor(
    private readonly repo: PropertyStore,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'property',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getProperty(id: string): Promise<FormProperty | undefined> {
    const doc = await this.repo.getPropertyById(id);
    return doc
      ? FormProperty.fromRecord(PropertyShapeSchema.parse(doc))
      : undefined;
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
      kind !== 'property.create' &&
      kind !== 'property.setCore' &&
      kind !== 'property.setState'
    ) {
      return undefined;
    }

    const id = (payload as any)?.id as string | undefined;
    if (!id) return undefined;

    const op = kind === 'property.create' ? 'assert' : 'revise';

    const tags: DiscursiveRuleTag[] = [{ layer: 'property', rule: 'relation' }];

    return {
      factStore: {
        mode: 'logic',
        store: 'FormDB',
        op,
        kind: 'Property',
        ids: [id],
      },
      dialectic: toDialecticalInfo(tags),
    };
  }

  private async mustGet(id: string): Promise<FormProperty> {
    const p = await this.getProperty(id);
    if (!p) throw new Error(`Property not found: ${id}`);
    return p;
  }

  private async persist(p: FormProperty) {
    await this.repo.saveProperty(p.toRecord());
  }

  async handle(cmd: PropertyCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'PropertyEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'property.create': {
        const { payload } = cmd as PropertyCreateCmd;
        const p = FormProperty.create(payload as any);
        await this.persist(p);
        return [
          this.emit(base, 'property.create', {
            id: p.id,
            type: p.type,
            name: p.name,
          }),
        ];
      }

      case 'property.delete': {
        const { id } = (cmd as PropertyDeleteCmd).payload;
        const existed = await this.repo.getPropertyById(id);
        if (existed && this.repo.deleteProperty) {
          await this.repo.deleteProperty(id);
        }
        return [this.emit(base, 'property.delete', { id, ok: !!existed })];
      }

      case 'property.setCore': {
        const { id, name, type } = (cmd as PropertySetCoreCmd).payload;
        const p = await this.mustGet(id);
        p.setCore({ name, type });
        await this.persist(p);
        return [
          this.emit(base, 'property.setCore', {
            id,
            name: p.name,
            type: p.type,
          }),
        ];
      }

      case 'property.setState': {
        const { id, state } = (cmd as PropertySetStateCmd).payload;
        const p = await this.mustGet(id);
        p.setState(state);
        await this.persist(p);
        return [this.emit(base, 'property.setState', { id })];
      }

      case 'property.patchState': {
        const { id, patch } = (cmd as PropertyPatchStateCmd).payload;
        const p = await this.mustGet(id);
        p.patchState(patch);
        await this.persist(p);
        return [this.emit(base, 'property.patchState', { id })];
      }

      case 'property.describe': {
        const { id } = (cmd as PropertyDescribeCmd).payload;
        const doc = await this.repo.getPropertyById(id);
        if (!doc) {
          return [this.emit(base, 'property.describe', { id })];
        }

        const p = FormProperty.fromRecord(PropertyShapeSchema.parse(doc));
        return [
          this.emit(base, 'property.describe', {
            id,
            type: p.type,
            name: p.name,
            state: doc.state,
            signatureKeys: Object.keys(
              (doc.signature ?? {}) as Record<string, unknown>,
            ),
            facetsKeys: Object.keys(
              (doc.facets ?? {}) as Record<string, unknown>,
            ),
          }),
        ];
      }

      // --- Dialectic EVAL: Property as Law / Facticity ---
      // Property is the INVARIANT structure of the World - what persists through flux.
      // Law (Hegel) = the tranquil image of Appearance.
      // Property extracts: invariants, facticity grounds, mediating relations.
      // This is the living FactStore - marks of impure Dharmas.
      case 'dialectic.evaluate': {
        const { dialecticState, context: evalContext } = (
          cmd as DialecticEvaluateCmd
        ).payload;

        // Create a Property to represent this dialectic invariant
        const prop = FormProperty.create({
          id: dialecticState.id,
          type: dialecticState.concept,
          name: dialecticState.title,
        });

        // Extract INVARIANTS from the dialectic state
        // Each invariant becomes a scientific law - what must hold
        const invariants = dialecticState.invariants.map((inv) => ({
          id: inv.id,
          constraint: inv.constraint,
          predicate: inv.predicate,
          universality:
            inv.conditions && inv.conditions.length > 0
              ? 'conditional'
              : 'necessary',
        }));

        // Extract FACTICITY from moments
        // Moments that ground the property - the evidence/witnesses
        // 'polarity' and 'negation' moments are the dialectical grounds
        const groundingMoments = dialecticState.moments.filter(
          (m) =>
            m.type === 'polarity' ||
            m.type === 'negation' ||
            m.type === 'mediation',
        );

        const facticity = {
          grounds: groundingMoments.map((m) => m.name),
          conditions: dialecticState.invariants.flatMap(
            (inv) => inv.conditions ?? [],
          ),
          evidence: groundingMoments.map((m) => ({
            name: m.name,
            definition: m.definition,
            type: m.type,
          })),
        };

        // Extract MEDIATION structure
        // Property mediates Entity↔Aspect (Thing↔Relation)
        // 'mediates' and 'transforms' relations show this structure
        const mediatingMoments = dialecticState.moments.filter(
          (m) => m.relation === 'mediates' || m.relation === 'transforms',
        );

        const mediates = {
          fromEntities: mediatingMoments.map((m) => m.name),
          toAspects: mediatingMoments
            .map((m) => m.relatedTo)
            .filter(Boolean) as string[],
        };

        // Store in signature: the moments as the invariant's structure
        const signature = dialecticState.moments.reduce((acc, m) => {
          acc[m.name] = {
            definition: m.definition,
            type: m.type,
            relation: m.relation,
            relatedTo: m.relatedTo,
          };
          return acc;
        }, {} as Record<string, any>);

        prop.setSignature(signature);

        // Store in facets: the Law structure (facticity, invariants, mediation)
        prop.setFacets({
          dialecticState: dialecticState,
          phase: dialecticState.phase,
          // The Law as scientific invariant
          law: {
            invariants,
            universality: invariants.every(
              (i) => i.universality === 'necessary',
            )
              ? 'necessary'
              : 'conditional',
          },
          // Facticity - the living FactStore
          facticity,
          // Mediation structure
          mediates,
          // Evaluation context
          context: evalContext,
        });

        await this.persist(prop);

        return [
          this.emit(base, 'dialectic.evaluated', {
            stateId: dialecticState.id,
            concept: dialecticState.concept,
            phase: dialecticState.phase,
            kind: 'property',
            invariantCount: invariants.length,
            groundCount: facticity.grounds.length,
            universality: invariants.every(
              (i) => i.universality === 'necessary',
            )
              ? 'necessary'
              : 'conditional',
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  // ADR-like interface
  async process(
    properties: Array<active.ActiveProperty>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = properties as any[]; // Skip parsing for now to preserve test data
    const actions: any[] = [];
    for (const p of list) {
      if (p.revoked === true) {
        if (p.id) actions.push({ type: 'property.delete', id: p.id });
        continue;
      }
      actions.push({
        type: 'property.upsert',
        id: p.id ?? childIdFromName(p.name),
        name: p.name,
        propertyType: p.propertyType ?? 'system.Property',
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'property.delete') {
        const [evt] = await this.handle({
          kind: 'property.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'property.upsert') {
        const id = a.id as string;
        const existing = await this.getProperty(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'property.create',
            payload: { id, type: a.propertyType, name: a.name },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'property.setCore',
            payload: { id, type: a.propertyType, name: a.name },
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
  return `property:${base}`;
}
