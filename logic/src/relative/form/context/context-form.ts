import { z } from 'zod';
import { BaseState, EntityRef, Id, Type } from '@schema';
import { ContextShapeSchema, type ContextShapeRepo } from '@schema/context';

type BaseStateShape = z.infer<typeof BaseState>;

export class FormContext {
  private _doc: ContextShapeRepo;

  private constructor(doc: ContextShapeRepo) {
    this._doc = ContextShapeSchema.parse(doc);
  }

  // Factory: create from inputs (validated)
  static create(input: {
    type: z.input<typeof Type>;
    id?: string;
    name?: string;
    description?: string;
    state?: z.input<typeof BaseState>;
    entities?: Array<z.input<typeof EntityRef>>;
    relations?: Array<z.input<typeof Id>>;
    signature?: Record<string, unknown>;
    facets?: Record<string, unknown>;
  }): FormContext {
    const now = Date.now();
    const doc: ContextShapeRepo = {
      id:
        input.id ?? `context:${now}:${Math.random().toString(36).slice(2, 10)}`,
      type: Type.parse(input.type),
      name: input.name,
      description: input.description,
      state: BaseState.parse(input.state ?? {}),
      entities: (input.entities ?? []).map((e) => EntityRef.parse(e)),
      relations: (input.relations ?? []).map((r) => Id.parse(r)),
      signature: input.signature,
      facets: input.facets ?? {},
      createdAt: now,
      updatedAt: now,
    };
    return new FormContext(doc);
  }

  // Factory: wrap an existing repo record (validates)
  static fromRecord(doc: ContextShapeRepo): FormContext {
    return new FormContext(ContextShapeSchema.parse(doc));
  }

  // Compatibility alias
  static from(doc: ContextShapeRepo): FormContext {
    return FormContext.fromRecord(doc);
  }

  toRecord(): ContextShapeRepo {
    return this._doc;
  }

  toSchema(): ContextShapeRepo {
    return this.toRecord();
  }

  toJSON(): ContextShapeRepo {
    return this._doc;
  }

  // Getters
  get id(): string {
    return this._doc.id;
  }
  get type(): string {
    return this._doc.type;
  }
  get name(): string | undefined {
    return this._doc.name;
  }
  get description(): string | undefined {
    return this._doc.description;
  }
  get state(): BaseStateShape {
    return (this._doc.state ?? {}) as BaseStateShape;
  }
  get entities() {
    return this._doc.entities ?? [];
  }
  get relations() {
    return this._doc.relations ?? [];
  }
  get signature(): Record<string, unknown> | undefined {
    return this._doc.signature as Record<string, unknown> | undefined;
  }
  get facets(): Record<string, unknown> | undefined {
    return this._doc.facets as Record<string, unknown> | undefined;
  }
  get createdAt(): number | undefined {
    return this._doc.createdAt;
  }
  get updatedAt(): number | undefined {
    return this._doc.updatedAt;
  }

  // Mutators
  setCore(core: { name?: string; type?: string }): this {
    this._doc = {
      ...this._doc,
      name: core.name !== undefined ? core.name : this._doc.name,
      type: core.type !== undefined ? core.type : this._doc.type,
      updatedAt: Date.now(),
    };
    return this;
  }

  setState(state: BaseStateShape): this {
    this._doc = {
      ...this._doc,
      state: BaseState.parse(state ?? {}),
      updatedAt: Date.now(),
    };
    return this;
  }

  patchState(patch: Partial<BaseStateShape>): this {
    const current = (this._doc.state ?? {}) as BaseStateShape;
    this._doc = {
      ...this._doc,
      state: BaseState.parse({ ...current, ...patch }),
      updatedAt: Date.now(),
    };
    return this;
  }

  addEntity(ref: z.input<typeof EntityRef>): this {
    const parsed = EntityRef.parse(ref);
    const key = `${parsed.type}:${parsed.id}`;
    const existing = new Set(
      (this._doc.entities ?? []).map((e) => `${e.type}:${e.id}`),
    );
    if (!existing.has(key)) {
      this._doc = {
        ...this._doc,
        entities: [...(this._doc.entities ?? []), parsed],
        updatedAt: Date.now(),
      };
    }
    return this;
  }

  addEntities(refs: Array<z.input<typeof EntityRef>>): this {
    const existing = new Set(
      (this._doc.entities ?? []).map((e) => `${e.type}:${e.id}`),
    );
    const toAdd = (refs ?? [])
      .map((r) => EntityRef.parse(r))
      .filter((r) => !existing.has(`${r.type}:${r.id}`));
    if (toAdd.length) {
      this._doc = {
        ...this._doc,
        entities: [...(this._doc.entities ?? []), ...toAdd],
        updatedAt: Date.now(),
      };
    }
    return this;
  }

  removeEntity(ref: z.input<typeof EntityRef>): this {
    const parsed = EntityRef.parse(ref);
    const next = (this._doc.entities ?? []).filter(
      (e) => !(e.id === parsed.id && e.type === parsed.type),
    );
    this._doc = { ...this._doc, entities: next, updatedAt: Date.now() };
    return this;
  }

  addRelation(rel: z.input<typeof Id>): this {
    const parsed = Id.parse(rel);
    const relations = this._doc.relations ?? [];
    if (!relations.includes(parsed)) {
      this._doc = {
        ...this._doc,
        relations: [...relations, parsed],
        updatedAt: Date.now(),
      };
    }
    return this;
  }

  addRelations(rels: Array<z.input<typeof Id>>): this {
    const existing = new Set(this._doc.relations ?? []);
    const toAdd = (rels ?? [])
      .map((r) => Id.parse(r))
      .filter((r) => !existing.has(r));
    if (toAdd.length) {
      this._doc = {
        ...this._doc,
        relations: [...(this._doc.relations ?? []), ...toAdd],
        updatedAt: Date.now(),
      };
    }
    return this;
  }

  removeRelation(rel: z.input<typeof Id>): this {
    const parsed = Id.parse(rel);
    const next = (this._doc.relations ?? []).filter((r) => r !== parsed);
    this._doc = { ...this._doc, relations: next, updatedAt: Date.now() };
    return this;
  }

  setSignature(signature?: Record<string, unknown>): this {
    this._doc = { ...this._doc, signature, updatedAt: Date.now() };
    return this;
  }

  mergeSignature(patch: Record<string, unknown>): this {
    const current = (this._doc.signature ?? {}) as Record<string, unknown>;
    this._doc = {
      ...this._doc,
      signature: { ...current, ...patch },
      updatedAt: Date.now(),
    };
    return this;
  }

  setFacets(facets?: Record<string, unknown>): this {
    this._doc = { ...this._doc, facets, updatedAt: Date.now() };
    return this;
  }

  mergeFacets(patch: Record<string, unknown>): this {
    const current = (this._doc.facets ?? {}) as Record<string, unknown>;
    this._doc = {
      ...this._doc,
      facets: { ...current, ...patch },
      updatedAt: Date.now(),
    };
    return this;
  }
}
