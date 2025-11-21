import { z } from "zod";
import {
  ContextSchema,
  type Context,
  ContextCore,
  ContextState,
  createContext as createContextDoc,
  updateContext as updateContextDoc,
} from '@schema';
import { EntityRef } from '@schema';
import { Id } from '@schema';

type BaseState = Context["shape"]["state"];

export class FormContext {
  private doc: Context;

  private constructor(doc: Context) {
    this.doc = ContextSchema.parse(doc);
  }

  // Factory: create a new Context (schema defaults/validation applied)
  static create(input: {
    type: z.input<typeof ContextCore.shape.type>;
    id?: string;
    name?: z.input<typeof ContextCore.shape.name>;
    description?: z.input<typeof ContextCore.shape.description>;
    state?: z.input<typeof ContextState>;
    entities?: z.input<typeof EntityRef>[];
    relations?: z.input<typeof Id>[];
    version?: string;
    ext?: Record<string, unknown>;
  }): FormContext {
    const doc = createContextDoc(input as any);
    return new FormContext(doc);
  }

  // Factory: wrap an existing schema doc (or input parsed to one)
  static from(input: z.input<typeof ContextSchema> | Context): FormContext {
    const doc = ContextSchema.parse(input as any);
    return new FormContext(doc);
  }
  static fromSchema(doc: Context): FormContext {
    return new FormContext(doc);
  }

  // Access underlying schema doc
  toSchema(): Context {
    return this.doc;
  }
  toJSON(): Context {
    return this.doc;
  }

  // Basic getters
  get id(): string { return this.doc.shape.core.id; }
  get type(): string { return this.doc.shape.core.type as string; }
  get name(): string | undefined { return this.doc.shape.core.name; }
  get description(): string | undefined { return this.doc.shape.core.description; }
  get createdAt(): string { return this.doc.shape.core.createdAt; }
  get updatedAt(): string { return this.doc.shape.core.updatedAt; }
  get revision(): number { return this.doc.revision; }
  get version(): string | undefined { return this.doc.version; }

  // State (open map via BaseState)
  get state(): BaseState { return this.doc.shape.state; }

  // Core/state mutators (schema-safe)
  setCore(core: { name?: string; type?: string }): this {
    this.doc = updateContextDoc(this.doc, {
      core: {
        ...(core.name !== undefined ? { name: core.name } : {}),
        ...(core.type !== undefined ? { type: core.type } : {}),
      },
    });
    return this;
  }
  setName(name?: string): this {
    this.doc = updateContextDoc(this.doc, { core: { name } });
    return this;
  }
  setDescription(description?: string): this {
    this.doc = updateContextDoc(this.doc, { core: { description } });
    return this;
  }
  setState(state: BaseState): this {
    this.doc = updateContextDoc(this.doc, { state } as any);
    return this;
  }
  patchState(patch: Partial<BaseState>): this {
    this.doc = updateContextDoc(this.doc, { state: patch } as any);
    return this;
  }

  // Membership: entities
  addEntity(ref: z.input<typeof EntityRef>): this {
    const parsed = EntityRef.parse(ref);
    const key = `${parsed.type}:${parsed.id}`;
    const exists = new Set(this.doc.shape.entities.map(e => `${e.type}:${e.id}`));
    if (!exists.has(key)) {
      this.doc = updateContextDoc(this.doc, { entities: [...this.doc.shape.entities, parsed] });
    }
    return this;
  }
  addEntities(refs: z.input<typeof EntityRef>[]): this {
    const exists = new Set(this.doc.shape.entities.map(e => `${e.type}:${e.id}`));
    const toAdd = (refs ?? []).map(r => EntityRef.parse(r)).filter(r => !exists.has(`${r.type}:${r.id}`));
    if (toAdd.length > 0) {
      this.doc = updateContextDoc(this.doc, { entities: [...this.doc.shape.entities, ...toAdd] });
    }
    return this;
  }
  removeEntity(ref: z.input<typeof EntityRef>): this {
    const parsed = EntityRef.parse(ref);
    const next = this.doc.shape.entities.filter(e => !(e.id === parsed.id && e.type === parsed.type));
    if (next.length !== this.doc.shape.entities.length) {
      this.doc = updateContextDoc(this.doc, { entities: next });
    }
    return this;
  }
  clearEntities(): this {
    if (this.doc.shape.entities.length) {
      this.doc = updateContextDoc(this.doc, { entities: [] });
    }
    return this;
  }

  // Membership: relations (by id)
  addRelation(id: z.input<typeof Id>): this {
    if (!this.doc.shape.relations.includes(id)) {
      this.doc = updateContextDoc(this.doc, { relations: [...this.doc.shape.relations, id] });
    }
    return this;
  }
  addRelations(ids: z.input<typeof Id>[]): this {
    const set = new Set(this.doc.shape.relations);
    const toAdd = (ids ?? []).filter(id => !set.has(id));
    if (toAdd.length > 0) {
      this.doc = updateContextDoc(this.doc, { relations: [...this.doc.shape.relations, ...toAdd] });
    }
    return this;
  }
  removeRelation(id: z.input<typeof Id>): this {
    if (this.doc.shape.relations.includes(id)) {
      const next = this.doc.shape.relations.filter(r => r !== id);
      this.doc = updateContextDoc(this.doc, { relations: next });
    }
    return this;
  }
  clearRelations(): this {
    if (this.doc.shape.relations.length) {
      this.doc = updateContextDoc(this.doc, { relations: [] });
    }
    return this;
  }

  // Signature / facets
  setSignature(signature?: Record<string, unknown> | null): this {
    // undefined -> preserve; null -> clear; object -> replace
    if (signature === undefined) return this;
    if (signature === null) {
      this.doc = updateContextDoc(this.doc, { signature: null } as any);
      return this;
    }
    this.doc = updateContextDoc(this.doc, { signature } as any);
    return this;
  }
  mergeSignature(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.signature ?? {}) as Record<string, unknown>;
    this.doc = updateContextDoc(this.doc, { signature: { ...current, ...patch } } as any);
    return this;
  }
  setFacets(facets?: Record<string, unknown> | null): this {
    // undefined -> preserve; null -> clear to {}; object -> replace
    if (facets === undefined) return this;
    if (facets === null) {
      this.doc = updateContextDoc(this.doc, { facets: {} } as any);
      return this;
    }
    this.doc = updateContextDoc(this.doc, { facets } as any);
    return this;
  }
  mergeFacets(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    this.doc = updateContextDoc(this.doc, { facets: { ...current, ...patch } } as any);
    return this;
  }
  mergeFacet(ns: string, patch: Record<string, unknown> | null | undefined): this {
    if (!ns || !patch || Object.keys(patch).length === 0) return this;
    const all = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    const prev = (all[ns] as Record<string, unknown>) ?? {};
    const next = { ...prev, ...patch };
    this.doc = updateContextDoc(this.doc, { facets: { ...all, [ns]: next } } as any);
    return this;
  }
}
