import { FormShapeSchema, type FormShapeRepo } from '@schema/form';
import type { DialecticState, Moment } from '@schema';

/**
 * FormShape — Principle of Shape of Forms.
 *
 * Works directly with FormShapeSchema (repository form structure).
 * Dialectical data (signature, facets) is stored in the form's data/metadata.
 */
export class FormShape {
  private _doc: FormShapeRepo;

  private constructor(doc: FormShapeRepo) {
    // Validate on construction to keep invariants tight
    this._doc = FormShapeSchema.parse(doc);
  }

  // Factory: create from inputs
  static create(input: {
    type?: string; // Not in FormShapeSchema, but we can store in data
    name?: string;
    id?: string;
    state?: Record<string, unknown>;
    signature?: Record<string, unknown>;
    facets?: Record<string, unknown>;
  }): FormShape {
    const now = Date.now();
    const doc: FormShapeRepo = {
      id: input.id || `form:${now}:${Math.random().toString(36).slice(2, 10)}`,
      name: input.name || '',
      fields: [],
      // Store dialectical data in facets (we'll use a custom field or extend schema)
      // For now, store in data field as a workaround
      data: {
        dialectical: {
          type: input.type || 'system.Form',
          state: input.state || {},
          signature: input.signature,
          facets: input.facets || {},
        },
      },
      createdAt: now,
      updatedAt: now,
    };
    return new FormShape(doc);
  }

  // Factory: wrap an existing FormShape record (validates)
  static fromRecord(doc: FormShapeRepo): FormShape {
    return new FormShape(FormShapeSchema.parse(doc));
  }

  // Compatibility shim; prefer fromRecord
  static from(doc: FormShapeRepo): FormShape {
    return FormShape.fromRecord(doc);
  }

  // Accessors
  get id(): string {
    return this._doc.id;
  }
  get type(): string {
    // Extract from dialectical data
    return (this._doc.data as any)?.dialectical?.type || 'system.Form';
  }
  get name(): string {
    return this._doc.name;
  }
  get state(): Record<string, unknown> {
    return (this._doc.data as any)?.dialectical?.state || {};
  }
  get signature(): Record<string, unknown> | undefined {
    return (this._doc.data as any)?.dialectical?.signature;
  }
  get facets(): Record<string, unknown> {
    return (this._doc.data as any)?.dialectical?.facets || {};
  }
  // Data is stored under facets.data
  get data(): unknown {
    return this.facets.data;
  }

  // Dialectic Helpers
  getDialecticState(): DialecticState | undefined {
    return this.facets.dialecticState as DialecticState | undefined;
  }

  getMoments(): Moment[] {
    const sig = this.signature;
    if (!sig) return [];

    return Object.entries(sig).map(([name, def]: [string, any]) => ({
      name,
      definition: def.definition,
      type: def.type,
      relation: def.relation,
      relatedTo: def.relatedTo,
    }));
  }

  toRecord(): FormShapeRepo {
    return this._doc;
  }

  toSchema(): FormShapeRepo {
    return this.toRecord();
  }

  toJSON(): any {
    return this._doc;
  }

  // Mutators (chainable)

  setName(name?: string) {
    this._doc = { ...this._doc, name: name || '' };
    return this;
  }

  setType(type: string) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, type },
      },
    };
    return this;
  }

  setState(state: Record<string, unknown>) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, state },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  patchState(patch: Record<string, unknown>) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    const currentState = dialectical.state || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, state: { ...currentState, ...patch } },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  // Data helpers
  setData(data: unknown) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    const facets = { ...(dialectical.facets || {}), data };
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, facets },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  clearData() {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    const { data: _drop, ...rest } = dialectical.facets || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, facets: rest },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  mergeFacets(facets: Record<string, unknown>) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    const currentFacets = dialectical.facets || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: {
          ...dialectical,
          facets: { ...currentFacets, ...facets },
        },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  setFacets(facets: Record<string, unknown>) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, facets },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  patchSignature(partial: Record<string, unknown>) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    const currentSignature = dialectical.signature || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: {
          ...dialectical,
          signature: { ...currentSignature, ...partial },
        },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  setSignature(sig?: Record<string, unknown>) {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, signature: sig },
      },
      updatedAt: Date.now(),
    };
    return this;
  }

  clearSignature() {
    const dialectical = (this._doc.data as any)?.dialectical || {};
    this._doc = {
      ...this._doc,
      data: {
        ...(this._doc.data as any),
        dialectical: { ...dialectical, signature: undefined },
      },
      updatedAt: Date.now(),
    };
    return this;
  }
}
