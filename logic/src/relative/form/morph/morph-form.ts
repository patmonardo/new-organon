import {
  type Morph,
  MorphSchema,
  createMorph,
  updateMorph,
} from '@schema';

/**
 * FormMorph - Form layer wrapper over schema/morph for engine ergonomics.
 * Follows the Foundation pattern: schema-first, thin wrapper, preserves invariants.
 */
export class FormMorph {
  private constructor(private readonly doc: Morph) {}

  // Factory: create from schema document
  static fromSchema(doc: Morph): FormMorph {
    return new FormMorph(MorphSchema.parse(doc));
  }

  // Factory: create from parameters using schema helper
  static create(input: Parameters<typeof createMorph>[0]): FormMorph {
    const doc = createMorph(input);
    return new FormMorph(doc);
  }

  // Core field accessors
  get id(): string {
    return this.doc.shape.core.id;
  }

  get type(): string {
    return this.doc.shape.core.type;
  }

  get name(): string | undefined {
    return this.doc.shape.core.name;
  }

  get revision(): number {
    return this.doc.revision ?? 0;
  }

  // State accessors
  get state(): Morph['shape']['state'] {
    return this.doc.shape.state;
  }

  get signature(): Morph['shape']['signature'] {
    return this.doc.shape.signature;
  }

  get facets(): Morph['shape']['facets'] {
    return this.doc.shape.facets;
  }

  // Engine-friendly mutators (use schema helpers to preserve invariants)
  setCore(patch: { name?: string; type?: string }): void {
    const updated = updateMorph(this.doc, { core: patch } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  setState(state: Morph['shape']['state']): void {
    const updated = updateMorph(this.doc, { state } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  patchState(patch: Partial<Morph['shape']['state']>): void {
    const updated = updateMorph(this.doc, {
      state: { ...this.doc.shape.state, ...patch },
    } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  setSignature(signature: Morph['shape']['signature']): void {
    const updated = updateMorph(this.doc, { signature } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  mergeSignature(patch: Record<string, unknown>): void {
    const current = (this.doc.shape.signature ?? {}) as Record<string, unknown>;
    const updated = updateMorph(this.doc, {
      signature: { ...current, ...patch },
    } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  setFacets(facets: Morph['shape']['facets']): void {
    const updated = updateMorph(this.doc, { facets } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  mergeFacets(patch: Record<string, unknown>): void {
    const current = this.doc.shape.facets ?? {};
    const updated = updateMorph(this.doc, {
      facets: { ...current, ...patch },
    } as any);
    Object.assign(this, { doc: MorphSchema.parse(updated) });
  }

  // Persistence interface
  toSchema(): Morph {
    return this.doc;
  }

  // Engine metadata (could add context handles, trace info, etc.)
  toJSON(): any {
    return {
      ...this.doc,
      _meta: {
        formType: 'FormMorph',
        revision: this.revision,
      },
    };
  }

  // Introspection
  describe(): any {
    return {
      id: this.id,
      type: this.type,
      name: this.name,
      revision: this.revision,
      state: this.state,
      signatureKeys: Object.keys((this.signature ?? {}) as Record<string, unknown>),
      facetsKeys: Object.keys(this.facets ?? {}),
    };
  }
}
