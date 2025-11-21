import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import type { Repository } from '@repository';
import {
  type Aspect,
  AspectSchema,
  createAspect,
} from '@schema';
import { FormAspect } from './aspect-form';
import * as active from '@schema';

type BaseState = Aspect['shape']['state'];

type AspectCreateCmd = {
  kind: 'aspect.create';
  payload: Parameters<typeof createAspect>[0];
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
  payload: { id: string; state: BaseState };
  meta?: Record<string, unknown>;
};
type AspectPatchStateCmd = {
  kind: 'aspect.patchState';
  payload: { id: string; patch: Partial<BaseState> };
  meta?: Record<string, unknown>;
};

export type AspectCommand =
  | AspectCreateCmd
  | AspectDeleteCmd
  | AspectDescribeCmd
  | AspectSetCoreCmd
  | AspectSetStateCmd
  | AspectPatchStateCmd;

export class AspectEngine {
  constructor(
    private readonly repo: Repository<Aspect>,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'aspect',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getAspect(id: string): Promise<FormAspect | undefined> {
    const doc = await this.repo.get(id);
    return doc ? FormAspect.fromSchema(doc) : undefined;
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
    const id = a.id;
    const doc = AspectSchema.parse(a.toSchema());
    const current = await this.repo.get(id);
    if (current) {
      await this.repo.update(id, doc as any);
    } else {
      await this.repo.create(doc);
    }
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
        const created = AspectSchema.parse(createAspect(payload as any));
        const a = FormAspect.fromSchema(created);
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
        const existed = await this.repo.get(id);
        if (existed) {
          await this.repo.delete(id);
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
        const doc = await this.repo.get(id);
        if (!doc) {
          return [this.emit(base, 'aspect.describe', { id })];
        }

        const a = FormAspect.fromSchema(doc);
        return [
          this.emit(base, 'aspect.describe', {
            id,
            type: a.type,
            name: a.name ?? null,
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
