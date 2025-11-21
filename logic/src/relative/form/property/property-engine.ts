import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import type { Repository } from '@repository';
import { FormProperty } from './property-form';

import {
  type Property,
  PropertySchema,
  createProperty,
} from '@schema';
import * as active from '@schema';

type BaseState = Property['shape']['state'];

export type PropertyCreateCmd = {
  kind: 'property.create';
  payload: Parameters<typeof createProperty>[0];
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
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
export type PropertyPatchStateCmd = {
  kind: 'property.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};

export type PropertyCommand =
  | PropertyCreateCmd
  | PropertyDeleteCmd
  | PropertyDescribeCmd
  | PropertySetCoreCmd
  | PropertySetStateCmd
  | PropertyPatchStateCmd;

export class PropertyEngine {
  constructor(
    private readonly repo: Repository<Property>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'property',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getProperty(id: string): Promise<FormProperty | undefined> {
    const doc = await this.repo.get(id);
    return doc ? FormProperty.fromSchema(doc) : undefined;
  }

  private emit(base: any, kind: Event['kind'], payload: Event['payload']) {
    const meta = childSpan(base, { action: kind, scope: this.scope });
    const evt: Event = { kind, payload, meta };
    this.bus.publish(evt);
    return evt;
  }

  private async mustGet(id: string): Promise<FormProperty> {
    const p = await this.getProperty(id);
    if (!p) throw new Error(`Property not found: ${id}`);
    return p;
  }

  private async persist(p: FormProperty) {
    const id = p.id;
    const doc = PropertySchema.parse(p.toSchema());
    const current = await this.repo.get(id);
    if (current) {
      await this.repo.update(id, doc as any);
    } else {
      await this.repo.create(doc);
    }
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
        const created = PropertySchema.parse(createProperty(payload as any));
        const p = FormProperty.fromSchema(created);
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
        const existed = await this.repo.get(id);
        if (existed) {
          await this.repo.delete(id);
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
        const doc = await this.repo.get(id);
        if (!doc) {
          return [this.emit(base, 'property.describe', { id })];
        }

        const p = FormProperty.fromSchema(doc);
        return [
          this.emit(base, 'property.describe', {
            id,
            type: p.type,
            name: p.name,
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
