import { z } from "zod";
import {
  EntitySchema,
  EntityState as EntityStateSchema,
  createEntity as createEntityDoc,
  updateEntity as updateEntityDoc,
  createEntityRef,
  formatEntityKey,
  type Entity,
  type EntityRef,
  type EntityState,
} from '@schema';

export type FormEntityId = string;

export class FormEntity {
  private doc: Entity;

  private constructor(doc: Entity) {
    this.doc = EntitySchema.parse(doc);
  }

  // Factory: create from params (schema defaults/validation applied)
  static create(input: {
    type: string;
    id?: string;
    name?: string;
    description?: string;
    state?: z.infer<typeof EntityStateSchema>;
    version?: string;
    ext?: Record<string, unknown>;
  }): FormEntity {
    const doc = createEntityDoc(input as any);
    return new FormEntity(doc);
  }

  // Factory: wrap an existing schema doc (or input parsed to one)
  static from(input: Entity | z.infer<typeof EntitySchema>): FormEntity {
    const doc = EntitySchema.parse(input as any);
    return new FormEntity(doc);
  }

  // alias used by engines
  static fromSchema(doc: Entity): FormEntity {
    return new FormEntity(doc);
  }

  // Serialization
  toSchema(): Entity {
    return this.doc;
  }
  toJSON(): Entity {
    return this.doc;
  }

  // Core getters
  get id(): string {
    return this.doc.shape.core.id;
  }
  get type(): string {
    return this.doc.shape.core.type;
  }
  get name(): string | undefined {
    return this.doc.shape.core.name;
  }
  get description(): string | undefined {
    return this.doc.shape.core.description;
  }

  // timestamps / metadata (present via BaseCore)
  get createdAt(): string {
    return this.doc.shape.core.createdAt;
  }
  get updatedAt(): string {
    return this.doc.shape.core.updatedAt;
  }

  // State accessors
  get state(): EntityState {
    return this.doc.shape.state;
  }

  get revision(): number {
    return this.doc.revision ?? 0;
  }
  get version(): string | undefined {
    return this.doc.version;
  }

  // Reference/key helpers
  toRef(): EntityRef {
    return createEntityRef(this.doc);
  }
  get key(): string {
    return formatEntityKey(this.toRef());
  }

  // Core mutators (schema-safe via updateEntityDoc)
  setCore(core: { name?: string; type?: string }): this {
    this.doc = updateEntityDoc(this.doc, { core } as any);
    return this;
  }
  setState(state: EntityState): this {
    this.doc = updateEntityDoc(this.doc, { state } as any);
    return this;
  }
  patchState(patch: Partial<EntityState>): this {
    this.doc = updateEntityDoc(this.doc, { state: patch } as any);
    return this;
  }

  // Signature / facets (skeletal extension helpers)
  setSignature(signature?: Record<string, unknown> | null): this {
    // undefined -> preserve (no-op); null -> clear; object -> replace
    if (signature === undefined) return this;
    if (signature === null) {
      this.doc = updateEntityDoc(this.doc, { signature: null } as any); // clear
      return this;
    }
    this.doc = updateEntityDoc(this.doc, { signature } as any); // replace
    return this;
  }

  mergeSignature(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.signature ?? {}) as Record<string, unknown>;
    this.doc = updateEntityDoc(this.doc, { signature: { ...current, ...patch } } as any);
    return this;
  }
  setFacets(facets: Record<string, unknown>): this {
    this.doc = updateEntityDoc(this.doc, { facets } as any);
    return this;
  }
  mergeFacets(patch: Record<string, unknown>): this {
    const all = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    this.doc = updateEntityDoc(this.doc, { facets: { ...all, ...patch } } as any);
    return this;
  }
  mergeFacet(
    ns: string,
    patch: Record<string, unknown> | null | undefined,
  ): this {
    if (!ns || !patch || Object.keys(patch).length === 0) return this;
    const all = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    const prev = (all[ns] as Record<string, unknown>) ?? {};
    const nextNs = { ...prev, ...patch };
    const nextAll = { ...all, [ns]: nextNs };
    this.doc = updateEntityDoc(this.doc, { facets: nextAll } as any);
    return this;
  }

  // Convenience mutators used by tests
  setName(name?: string): this {
    this.doc = updateEntityDoc(this.doc, { core: { name } } as any);
    return this;
  }
  setDescription(description?: string): this {
    this.doc = updateEntityDoc(this.doc, { core: { description } } as any);
    return this;
  }
  addTag(tag: string): this {
    const tags = Array.isArray(this.doc.shape.state.tags)
      ? [...this.doc.shape.state.tags]
      : [];
    if (!tags.includes(tag)) {
      tags.push(tag);
      this.doc = updateEntityDoc(this.doc, { state: { tags } } as any);
    }
    return this;
  }
  removeTag(tag: string): this {
    const tags = Array.isArray(this.doc.shape.state.tags)
      ? this.doc.shape.state.tags.filter((t) => t !== tag)
      : [];
    this.doc = updateEntityDoc(this.doc, { state: { tags } } as any);
    return this;
  }
  patchMeta(patch: Record<string, unknown>): this {
    const next = { ...(this.doc.shape.state.meta ?? {}), ...patch };
    this.doc = updateEntityDoc(this.doc, { state: { meta: next } } as any);
    return this;
  }
}
