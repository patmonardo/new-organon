import { z } from 'zod';
import type { Command, Event } from '@absolute';
import type { EventBus } from '@absolute';
import { InMemoryEventBus } from '@absolute';
import { startTrace, childSpan } from '@absolute';
import * as active from '@schema';
import type { DialecticEvaluateCmd, DialecticCommand } from '@schema';
import { toDialecticalInfo, type DiscursiveRuleTag, BaseState } from '@schema';
import { ContextShapeSchema, type ContextShapeRepo } from '@schema/context';
import { FormContext } from './context-form';

type BaseStateShape = z.infer<typeof BaseState>;

type ContextCreateCmd = {
  kind: 'context.create';
  payload: Parameters<typeof FormContext.create>[0];
  meta?: Record<string, unknown>;
};
type ContextDeleteCmd = {
  kind: 'context.delete';
  payload: { id: string };
  meta?: Record<string, unknown>;
};
type ContextSetCoreCmd = {
  kind: 'context.setCore';
  payload: { id: string; name?: string; type?: string };
  meta?: Record<string, unknown>;
};
type ContextSetStateCmd = {
  kind: 'context.setState';
  payload: { id: string; state: BaseStateShape };
  meta?: Record<string, unknown>;
};
type ContextPatchStateCmd = {
  kind: 'context.patchState';
  payload: { id: string; patch: Partial<BaseStateShape> };
  meta?: Record<string, unknown>;
};
type ContextDescribeCmd = {
  kind: 'context.describe';
  payload: { id: string };
  meta?: Record<string, unknown>;
};

export type ContextCommand =
  | ContextCreateCmd
  | ContextDeleteCmd
  | ContextSetCoreCmd
  | ContextSetStateCmd
  | ContextPatchStateCmd
  | ContextDescribeCmd
  | DialecticCommand;

type ContextStore = {
  getContextById(id: string): Promise<ContextShapeRepo | null>;
  saveContext(data: Partial<ContextShapeRepo>): Promise<ContextShapeRepo>;
  deleteContext?(id: string): Promise<boolean>;
};

export class ContextEngine {
  private readonly contexts = new Map<string, FormContext>();

  constructor(
    private readonly repo?: ContextStore,
    private readonly bus: EventBus = new InMemoryEventBus(),
    private readonly scope: string = 'context',
  ) {}

  get eventBus(): EventBus {
    return this.bus;
  }

  async getContext(id: string): Promise<FormContext | undefined> {
    const cached = this.contexts.get(id);
    if (cached) return cached;
    if (!this.repo) return undefined;

    const doc = await this.repo.getContextById(id);
    if (!doc) return undefined;
    const ctx = this.repoRecordToFormContext(doc);
    this.contexts.set(ctx.id, ctx);
    return ctx;
  }

  private repoRecordToFormContext(repoContext: ContextShapeRepo): FormContext {
    return FormContext.fromRecord(ContextShapeSchema.parse(repoContext));
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
      kind !== 'context.create' &&
      kind !== 'context.setCore' &&
      kind !== 'context.setState'
    ) {
      return undefined;
    }

    const id = (payload as any)?.id as string | undefined;
    if (!id) return undefined;

    const op = kind === 'context.create' ? 'assert' : 'revise';

    const tags: DiscursiveRuleTag[] = [];
    const add = (t: DiscursiveRuleTag) => {
      if (!tags.some((x) => x.layer === t.layer && x.rule === t.rule))
        tags.push(t);
    };

    if (kind === 'context.create') add({ layer: 'context', rule: 'identity' });
    if (kind === 'context.setCore')
      add({ layer: 'context', rule: 'difference' });
    if (kind === 'context.setState')
      add({ layer: 'context', rule: 'contradiction' });

    add({ layer: 'context', rule: 'world' } as any);

    return {
      factStore: {
        mode: 'logic',
        store: 'FormDB',
        op,
        kind: 'Context',
        ids: [id],
      },
      dialectic: toDialecticalInfo(tags),
    };
  }

  private async mustGet(id: string): Promise<FormContext> {
    const c = await this.getContext(id);
    if (!c) throw new Error(`Context not found: ${id}`);
    return c;
  }

  private async persist(ctx: FormContext) {
    if (!this.repo) return;
    await this.repo.saveContext(ctx.toRecord());
  }

  async handle(cmd: ContextCommand | Command): Promise<Event[]> {
    const base = startTrace(
      'ContextEngine',
      (cmd as any).meta?.correlationId as string | undefined,
      (cmd as any).meta,
    );

    switch (cmd.kind) {
      case 'context.create': {
        const { payload } = cmd as ContextCreateCmd;
        const ctx = FormContext.create(payload);
        this.contexts.set(ctx.id, ctx);
        await this.persist(ctx);
        return [
          this.emit(base, 'context.create', {
            id: ctx.id,
            type: ctx.type,
            name: ctx.name ?? null,
          }),
        ];
      }

      case 'context.delete': {
        const { id } = (cmd as ContextDeleteCmd).payload;
        const existed = this.contexts.delete(id);
        if (this.repo?.deleteContext) await this.repo.deleteContext(id);
        return [this.emit(base, 'context.delete', { id, ok: existed })];
      }

      case 'context.setCore': {
        const { id, name, type } = (cmd as ContextSetCoreCmd).payload;
        const c = await this.mustGet(id);
        c.setCore({ name, type });
        await this.persist(c);
        return [
          this.emit(base, 'context.setCore', {
            id,
            name: c.name ?? null,
            type: c.type,
          }),
        ];
      }

      case 'context.setState': {
        const { id, state } = (cmd as ContextSetStateCmd).payload;
        const c = await this.mustGet(id);
        c.setState(state);
        await this.persist(c);
        return [this.emit(base, 'context.setState', { id })];
      }

      case 'context.patchState': {
        const { id, patch } = (cmd as ContextPatchStateCmd).payload;
        const c = await this.mustGet(id);
        c.patchState(patch);
        await this.persist(c);
        return [this.emit(base, 'context.patchState', { id })];
      }

      case 'context.describe': {
        const { id } = (cmd as ContextDescribeCmd).payload;
        const c = await this.mustGet(id);
        const doc = c.toRecord();
        return [
          this.emit(base, 'context.describe', {
            id,
            type: c.type,
            name: c.name ?? null,
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

      case 'dialectic.evaluate': {
        const { dialecticState, context: evalContext } = (
          cmd as DialecticEvaluateCmd
        ).payload;

        const ctx = FormContext.create({
          id: dialecticState.id,
          type: dialecticState.concept,
          name: dialecticState.title,
          description: dialecticState.description,
        });

        const presuppositions = dialecticState.moments
          .filter((m) => m.type === 'determination' || m.type === 'quality')
          .map((m) => ({
            name: m.name,
            definition: m.definition,
            posited: true,
          }));

        const scopeModal =
          dialecticState.phase === 'quality'
            ? 'actual'
            : dialecticState.phase === 'quantity'
            ? 'possible'
            : dialecticState.phase === 'reflection'
            ? 'necessary'
            : 'actual';

        const domain = [
          ...new Set(
            dialecticState.moments.map((m) => m.relatedTo).filter(Boolean),
          ),
        ] as string[];

        const conditions = dialecticState.invariants.map((inv) => ({
          id: inv.id,
          constraint: inv.constraint,
          predicate: inv.predicate,
        }));

        ctx.setState({
          status: 'active',
          meta: {
            presuppositions,
            scope: {
              modal: scopeModal,
              domain,
              phase: dialecticState.phase,
            },
            conditions,
            parentContext: evalContext?.parentContextId,
          },
        } as any);

        this.contexts.set(ctx.id, ctx);
        await this.persist(ctx);

        return [
          this.emit(base, 'dialectic.evaluated', {
            stateId: dialecticState.id,
            concept: dialecticState.concept,
            phase: dialecticState.phase,
            kind: 'context',
            presuppositionCount: presuppositions.length,
            conditionCount: conditions.length,
            scopeModal,
          }),
        ];
      }

      default:
        throw new Error(`unsupported command: ${(cmd as Command).kind}`);
    }
  }

  async process(
    contexts: Array<active.ActiveContext>,
    _particulars: any[] = [],
    _context?: any,
  ): Promise<{ actions: any[]; snapshot: { count: number } }> {
    const list = active.parseActiveContexts(contexts);
    const actions: any[] = [];
    for (const c of list) {
      if (c.revoked === true) {
        if (c.id) actions.push({ type: 'context.delete', id: c.id });
        continue;
      }
      actions.push({
        type: 'context.upsert',
        id: c.id ?? childIdFromName(c.name),
        name: c.name,
        kind: c.kind ?? 'system.Context',
      });
    }
    return { actions, snapshot: { count: list.length } };
  }

  async commit(actions: any[], _snapshot: { count: number }) {
    const events: Event[] = [];
    for (const a of actions) {
      if (a.type === 'context.delete') {
        const [evt] = await this.handle({
          kind: 'context.delete',
          payload: { id: a.id },
        } as any);
        events.push(evt);
      } else if (a.type === 'context.upsert') {
        const id = a.id as string;
        const existing = await this.getContext(id);
        if (!existing) {
          const [evt] = await this.handle({
            kind: 'context.create',
            payload: { id, type: a.kind, name: a.name },
          } as any);
          events.push(evt);
        } else {
          const [evt] = await this.handle({
            kind: 'context.setCore',
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
  if (!name) return `context:${Math.random().toString(36).slice(2, 10)}`;
  return `context:${name.toLowerCase().replace(/\s+/g, '-')}`;
}
