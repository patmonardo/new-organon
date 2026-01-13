import { EntityShapeSchema, type EntityShapeRepo } from '@schema/entity';
import type { DialecticState, Moment } from '@schema';

export type FormEntityId = string;

// FormEntity — nexus between the EntityShape record (repo) and runtime behavior
export class FormEntity {
  private _doc: EntityShapeRepo;

  private constructor(doc: EntityShapeRepo) {
    this._doc = EntityShapeSchema.parse(doc);
  }

  // Factory: create from params (validated)
  static create(input: {
    type: string;
    formId: string;
    id?: string;
    name?: string;
    description?: string;
    values?: Record<string, unknown>;
    signature?: Record<string, unknown>;
    facets?: Record<string, unknown>;
    status?: string;
    tags?: string[];
    meta?: Record<string, unknown>;
  }): FormEntity {
    const now = Date.now();
    const doc: EntityShapeRepo = {
      id:
        input.id ?? `entity:${now}:${Math.random().toString(36).slice(2, 10)}`,
      type: input.type,
      name: input.name,
      description: input.description,
      formId: input.formId,
      values: input.values ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
      status: input.status,
      tags: input.tags,
      meta: input.meta,
      createdAt: now,
      updatedAt: now,
    };
    return new FormEntity(doc);
  }

  // Factory: wrap an existing repo record (validates)
  static fromRecord(doc: EntityShapeRepo): FormEntity {
    return new FormEntity(EntityShapeSchema.parse(doc));
  }

  // Compatibility alias
  static from(doc: EntityShapeRepo): FormEntity {
    return FormEntity.fromRecord(doc);
  }

  toRecord(): EntityShapeRepo {
    return this._doc;
  }

  toSchema(): EntityShapeRepo {
    return this.toRecord();
  }

  toJSON(): EntityShapeRepo {
    return this._doc;
  }

  // Core getters
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
  get formId(): string {
    return this._doc.formId;
  }
  get values(): Record<string, unknown> {
    return this._doc.values ?? {};
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

  // Dialectic Helpers
  getDialecticState(): DialecticState | undefined {
    return (this._doc.facets as any)?.dialecticState as
      | DialecticState
      | undefined;
  }

  getMoments(): Moment[] {
    const sig = this.signature;
    if (!sig) return [];
    return Object.entries(sig).map(([name, def]: [string, any]) => ({
      name,
      definition: def?.definition,
      type: def?.type,
      relation: def?.relation,
      relatedTo: def?.relatedTo,
    }));
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

  setState(state: {
    status?: string;
    tags?: string[];
    meta?: Record<string, unknown>;
  }): this {
    this._doc = {
      ...this._doc,
      status: state.status,
      tags: state.tags,
      meta: state.meta,
      updatedAt: Date.now(),
    };
    return this;
  }

  patchState(
    patch: Partial<{
      status?: string;
      tags?: string[];
      meta?: Record<string, unknown>;
    }>,
  ): this {
    this._doc = {
      ...this._doc,
      status: patch.status !== undefined ? patch.status : this._doc.status,
      tags: patch.tags !== undefined ? patch.tags : this._doc.tags,
      meta:
        patch.meta !== undefined
          ? { ...(this._doc.meta ?? {}), ...patch.meta }
          : this._doc.meta,
      updatedAt: Date.now(),
    };
    return this;
  }

  setValues(values: Record<string, unknown>): this {
    this._doc = { ...this._doc, values: values ?? {}, updatedAt: Date.now() };
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

  setFacets(facets: Record<string, unknown>): this {
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

  setName(name?: string): this {
    return this.setCore({ name });
  }

  setDescription(description?: string): this {
    this._doc = { ...this._doc, description, updatedAt: Date.now() };
    return this;
  }

  addTag(tag: string): this {
    const tags = Array.isArray(this._doc.tags) ? [...this._doc.tags] : [];
    if (!tags.includes(tag)) tags.push(tag);
    this._doc = { ...this._doc, tags, updatedAt: Date.now() };
    return this;
  }

  removeTag(tag: string): this {
    const tags = Array.isArray(this._doc.tags)
      ? this._doc.tags.filter((t) => t !== tag)
      : [];
    this._doc = { ...this._doc, tags, updatedAt: Date.now() };
    return this;
  }

  patchMeta(patch: Record<string, unknown>): this {
    const next = { ...(this._doc.meta ?? {}), ...patch };
    this._doc = { ...this._doc, meta: next, updatedAt: Date.now() };
    return this;
  }
}
