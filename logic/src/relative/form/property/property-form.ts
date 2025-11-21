import {
  type Property,
  PropertySchema,
  createProperty,
  updateProperty,
} from '@schema';

type BaseState = Property['shape']['state'];

export class FormProperty {
  private doc: Property;

  private constructor(doc: Property) {
    this.doc = PropertySchema.parse(doc);
  }

  // Factory
  static create(input: Parameters<typeof createProperty>[0]) {
    const doc = createProperty(input as any);
    return new FormProperty(PropertySchema.parse(doc));
  }
  static fromSchema(doc: Property) {
    return new FormProperty(PropertySchema.parse(doc));
  }
  static from(input: Property | Record<string, unknown>) {
    return new FormProperty(PropertySchema.parse(input as any));
  }

  // Serialize
  toSchema(): Property {
    return this.doc;
  }
  toJSON(): Property {
    return this.doc;
  }

  // Core getters (skeletal)
  get id(): string {
    return this.doc.shape.core.id;
  }
  get type(): string {
    return this.doc.shape.core.type;
  }
  get name(): string | null {
    return this.doc.shape.core.name ?? null;
  }

  // State helpers
  get state(): BaseState {
    return this.doc.shape.state;
  }

  // Core mutators
  setCore(core: { name?: string; type?: string }): this {
    this.doc = updateProperty(this.doc, {
      core: {
        ...(core.name !== undefined ? { name: core.name } : {}),
        ...(core.type !== undefined ? { type: core.type } : {}),
      },
    });
    return this;
  }
  setState(state: BaseState): this {
    this.doc = updateProperty(this.doc, { state });
    return this;
  }
  patchState(patch: Partial<BaseState>): this {
    this.doc = updateProperty(this.doc, { state: patch });
    return this;
  }

  // Signature / facets
  setSignature(signature?: Record<string, unknown> | null): this {
    if (signature === undefined) return this; // preserve
    if (signature === null) return this.applyPatch({ signature: null }); // clear
    return this.applyPatch({ signature: { ...signature } }); // replace
  }
  mergeSignature(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.signature ?? {}) as Record<string, unknown>;
    return this.applyPatch({ signature: { ...current, ...patch } });
  }
  setFacets(facets?: Record<string, unknown> | null): this {
    if (facets === undefined) return this; // preserve
    if (facets === null) return this.applyPatch({ facets: {} }); // clear to {}
    return this.applyPatch({ facets: { ...facets } });
  }
  mergeFacets(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    return this.applyPatch({ facets: { ...current, ...patch } });
  }
  mergeFacet(
    ns: string,
    patch: Record<string, unknown> | null | undefined,
  ): this {
    if (!ns || !patch || Object.keys(patch).length === 0) return this;
    const all = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    const prev = (all[ns] as Record<string, unknown>) ?? {};
    const next = { ...prev, ...patch };
    return this.applyPatch({ facets: { ...all, [ns]: next } });
  }

  private applyPatch(patch: Partial<{
    core?: Partial<any>;
    state?: Partial<BaseState>;
    signature?: Record<string, unknown> | null;
    facets?: Record<string, unknown>;
    version?: string;
    ext?: Record<string, unknown>;
  }>): this {
    this.doc = updateProperty(this.doc, patch as any);
    return this;
  }
}
