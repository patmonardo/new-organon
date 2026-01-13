import { z } from 'zod';
import { BaseState } from '@schema';
import {
  MorphSchema,
  MorphComposition,
  type Morph,
  type MorphFacets,
} from '@schema/morph';

/**
 * FormMorph - Form layer wrapper over schema/morph for engine ergonomics.
 * Follows the Foundation pattern: schema-first, thin wrapper, preserves invariants.
 */
type BaseStateShape = z.infer<typeof BaseState>;

export class FormMorph {
  private _doc: Morph;

  private constructor(doc: Morph) {
    this._doc = MorphSchema.parse(doc);
  }

  // Factory: create from schema document
  static fromSchema(doc: Morph): FormMorph {
    return new FormMorph(doc);
  }

  // Factory: create from parameters using schema helper
  static create(input: {
    id?: string;
    type: string;
    name?: string;
    description?: string;
    inputType?: string;
    outputType?: string;
    transformFn?: string;
    composition?: z.input<typeof MorphComposition>;
    config?: Record<string, unknown>;
    meta?: Record<string, unknown>;
    signature?: Record<string, unknown>;
    facets?: z.input<typeof MorphFacets>;
    state?: z.input<typeof BaseState>;
    status?: string;
    tags?: string[];
  }): FormMorph {
    const now = Date.now();
    const doc: Morph = MorphSchema.parse({
      id:
        input.id ??
        `morph:${now.toString(36)}:${Math.random().toString(36).slice(2, 8)}`,
      type: input.type,
      name: input.name,
      description: input.description,
      inputType: input.inputType ?? 'FormShape',
      outputType: input.outputType ?? 'FormShape',
      transformFn: input.transformFn,
      state: input.state ?? {},
      signature: input.signature,
      facets: input.facets ?? {},
      composition: MorphComposition.parse(input.composition ?? {}),
      config: input.config ?? {},
      status: input.status,
      tags: input.tags,
      meta: input.meta,
      createdAt: now,
      updatedAt: now,
    });
    return new FormMorph(doc);
  }

  // Core field accessors
  get id(): string {
    return this._doc.id;
  }

  get type(): string {
    return this._doc.type;
  }

  get name(): string | undefined {
    return this._doc.name;
  }

  // State accessors
  get state(): Morph['state'] {
    return (this._doc.state ?? {}) as BaseStateShape;
  }

  get signature(): Morph['signature'] {
    return this._doc.signature;
  }

  get facets(): Morph['facets'] {
    return this._doc.facets;
  }

  get composition(): Morph['composition'] {
    return this._doc.composition;
  }

  get config(): Morph['config'] {
    return this._doc.config ?? {};
  }

  // Engine-friendly mutators (use schema helpers to preserve invariants)
  setCore(patch: { name?: string; type?: string }): void {
    this._doc = MorphSchema.parse({
      ...this._doc,
      type: patch.type ?? this._doc.type,
      name: patch.name !== undefined ? patch.name : this._doc.name,
      updatedAt: Date.now(),
    });
  }

  setState(state: Morph['state']): void {
    this._doc = MorphSchema.parse({
      ...this._doc,
      state,
      updatedAt: Date.now(),
    });
  }

  patchState(patch: Partial<Morph['state']>): void {
    const nextState = { ...(this._doc.state ?? {}), ...patch };
    this._doc = MorphSchema.parse({
      ...this._doc,
      state: nextState,
      updatedAt: Date.now(),
    });
  }

  setSignature(signature: Morph['signature']): void {
    if (signature === undefined) return;
    const next = signature === null ? undefined : { ...signature };
    this._doc = MorphSchema.parse({
      ...this._doc,
      signature: next,
      updatedAt: Date.now(),
    });
  }

  mergeSignature(patch: Record<string, unknown>): void {
    const current = (this._doc.signature ?? {}) as Record<string, unknown>;
    if (!patch || Object.keys(patch).length === 0) return;
    this._doc = MorphSchema.parse({
      ...this._doc,
      signature: { ...current, ...patch },
      updatedAt: Date.now(),
    });
  }

  setFacets(facets: Morph['facets']): void {
    if (facets === undefined) return;
    this._doc = MorphSchema.parse({
      ...this._doc,
      facets: facets ?? {},
      updatedAt: Date.now(),
    });
  }

  mergeFacets(patch: Record<string, unknown>): void {
    const current = this._doc.facets ?? {};
    if (!patch || Object.keys(patch).length === 0) return;
    this._doc = MorphSchema.parse({
      ...this._doc,
      facets: { ...current, ...patch },
      updatedAt: Date.now(),
    });
  }

  setComposition(composition: Morph['composition']): void {
    this._doc = MorphSchema.parse({
      ...this._doc,
      composition,
      updatedAt: Date.now(),
    });
  }

  // Persistence interface
  toSchema(): Morph {
    return this._doc;
  }

  // Engine metadata (could add context handles, trace info, etc.)
  toJSON(): any {
    return {
      ...this._doc,
      _meta: { formType: 'FormMorph' },
    };
  }

  // Introspection
  describe(): any {
    return {
      id: this.id,
      type: this.type,
      name: this.name,
      state: this.state,
      signatureKeys: Object.keys(
        (this.signature ?? {}) as Record<string, unknown>,
      ),
      facetsKeys: Object.keys(this.facets ?? {}),
    };
  }
}
