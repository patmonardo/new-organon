import { z } from 'zod';
import { BaseState } from '@schema';
import {
  AspectSchema,
  AspectShapeSchema,
  type Aspect,
  type AspectShapeRepo,
} from '@schema/aspect';

type BaseStateShape = z.infer<typeof BaseState>;

export class FormAspect {
  private _doc: AspectShapeRepo;

  private constructor(doc: AspectShapeRepo) {
    this._doc = AspectShapeSchema.parse(doc);
  }

  static create(input: {
    id?: string;
    type: string;
    name?: string;
    state?: z.input<typeof BaseState>;
    signature?: Record<string, unknown>;
    facets?: Record<string, unknown>;
    status?: string;
    tags?: string[];
    meta?: Record<string, unknown>;
  }): FormAspect {
    const now = Date.now();
    const doc: AspectShapeRepo = {
      id: input.id ?? FormAspect.generateId(now),
      type: input.type,
      name: input.name,
      state: BaseState.parse(input.state ?? {}),
      signature: input.signature,
      facets: input.facets ?? {},
      status: input.status,
      tags: input.tags,
      meta: input.meta,
      createdAt: now,
      updatedAt: now,
    };
    return new FormAspect(doc);
  }

  static fromRecord(doc: AspectShapeRepo): FormAspect {
    return new FormAspect(AspectShapeSchema.parse(doc));
  }

  static from(doc: Aspect): FormAspect {
    return FormAspect.fromRecord(doc as AspectShapeRepo);
  }

  toRecord(): AspectShapeRepo {
    return this._doc;
  }

  toSchema(): Aspect {
    return this._doc as Aspect;
  }

  toJSON(): AspectShapeRepo {
    return this._doc;
  }

  get id(): string {
    return this._doc.id;
  }
  get type(): string {
    return this._doc.type;
  }
  get name(): string | undefined {
    return this._doc.name;
  }
  get state(): BaseStateShape {
    return (this._doc.state ?? {}) as BaseStateShape;
  }
  get signature(): Record<string, unknown> | undefined {
    return this._doc.signature as Record<string, unknown> | undefined;
  }
  get facets(): Record<string, unknown> | undefined {
    return this._doc.facets as Record<string, unknown> | undefined;
  }
  get status(): string | undefined {
    return this._doc.status;
  }
  get tags(): string[] | undefined {
    return this._doc.tags;
  }
  get meta(): Record<string, unknown> | undefined {
    return this._doc.meta as Record<string, unknown> | undefined;
  }
  get createdAt(): number | undefined {
    return this._doc.createdAt;
  }
  get updatedAt(): number | undefined {
    return this._doc.updatedAt;
  }

  setCore(core: { name?: string; type?: string }): this {
    this._doc = AspectSchema.parse({
      ...this._doc,
      name: core.name !== undefined ? core.name : this._doc.name,
      type: core.type !== undefined ? core.type : this._doc.type,
      updatedAt: Date.now(),
    });
    return this;
  }

  setState(state: BaseStateShape): this {
    this._doc = AspectSchema.parse({
      ...this._doc,
      state: BaseState.parse(state ?? {}),
      updatedAt: Date.now(),
    });
    return this;
  }

  patchState(patch: Partial<BaseStateShape>): this {
    const current = (this._doc.state ?? {}) as BaseStateShape;
    this._doc = AspectSchema.parse({
      ...this._doc,
      state: BaseState.parse({ ...current, ...patch }),
      updatedAt: Date.now(),
    });
    return this;
  }

  setSignature(signature?: Record<string, unknown> | null): this {
    if (signature === undefined) return this;
    const next = signature === null ? undefined : { ...signature };
    this._doc = AspectSchema.parse({
      ...this._doc,
      signature: next,
      updatedAt: Date.now(),
    });
    return this;
  }

  mergeSignature(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this._doc.signature ?? {}) as Record<string, unknown>;
    this._doc = AspectSchema.parse({
      ...this._doc,
      signature: { ...current, ...patch },
      updatedAt: Date.now(),
    });
    return this;
  }

  setFacets(facets?: Record<string, unknown> | null): this {
    if (facets === undefined) return this;
    const next = facets === null ? {} : { ...facets };
    this._doc = AspectSchema.parse({
      ...this._doc,
      facets: next,
      updatedAt: Date.now(),
    });
    return this;
  }

  mergeFacets(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this._doc.facets ?? {}) as Record<string, unknown>;
    this._doc = AspectSchema.parse({
      ...this._doc,
      facets: { ...current, ...patch },
      updatedAt: Date.now(),
    });
    return this;
  }

  mergeFacet(
    ns: string,
    patch: Record<string, unknown> | null | undefined,
  ): this {
    if (!ns || !patch || Object.keys(patch).length === 0) return this;
    const all = (this._doc.facets ?? {}) as Record<string, unknown>;
    const prev = (all[ns] as Record<string, unknown>) ?? {};
    const next = { ...prev, ...patch };
    this._doc = AspectSchema.parse({
      ...this._doc,
      facets: { ...all, [ns]: next },
      updatedAt: Date.now(),
    });
    return this;
  }

  private static generateId(now: number): string {
    return `aspect:${now.toString(36)}:${Math.random()
      .toString(36)
      .slice(2, 8)}`;
  }
}
