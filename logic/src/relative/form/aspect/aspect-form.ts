import { z } from 'zod';
import {
  type Aspect,
  AspectCore,
  AspectState,
  AspectSchema,
  createAspect,
  updateAspect,
} from '@schema';

type BaseState = Aspect['shape']['state'];

export class FormAspect {
  private doc: Aspect;

  private constructor(doc: Aspect) {
    this.doc = AspectSchema.parse(doc);
  }

  // Factory: create from params (skeletal)
  static create(input: {
    type: z.input<typeof AspectCore.shape.type>;
    id?: string;
    name?: z.input<typeof AspectCore.shape.name>;
    state?: z.input<typeof AspectState>;
    version?: string;
    ext?: Record<string, unknown>;
    signature?: Record<string, unknown>;
    facets?: Record<string, unknown>;
  }): FormAspect {
    const doc = AspectSchema.parse(createAspect(input as any));
    return new FormAspect(doc);
  }

  static from(input: z.input<typeof AspectSchema> | Aspect): FormAspect {
    const doc = AspectSchema.parse(input as any);
    return new FormAspect(doc);
  }
  static fromSchema(doc: Aspect): FormAspect {
    return new FormAspect(AspectSchema.parse(doc));
  }

  toSchema(): Aspect { return this.doc; }
  toJSON(): Aspect { return this.doc; }

  // Core getters
  get id(): string { return this.doc.shape.core.id; }
  get type(): string { return this.doc.shape.core.type; }
  get name(): string | null { return this.doc.shape.core.name ?? null; }

  // State (was missing)
  get state(): BaseState {
    return this.doc.shape.state;
  }

  // Core/state mutators
  setCore(core: { name?: string; type?: string }): this {
    this.doc = AspectSchema.parse(
      updateAspect(this.doc, {
        core: {
          ...(core.name !== undefined ? { name: core.name } : {}),
          ...(core.type !== undefined ? { type: core.type } : {}),
        },
      } as any),
    );
    return this;
  }
  setState(state: BaseState): this {
    this.doc = AspectSchema.parse(updateAspect(this.doc, { state } as any));
    return this;
  }
  patchState(patch: Partial<BaseState>): this {
    this.doc = AspectSchema.parse(updateAspect(this.doc, { state: patch } as any));
    return this;
  }

  // Signature / facets
  setSignature(signature?: Record<string, unknown> | null): this {
    // undefined -> preserve; null -> clear; object -> replace
    if (signature === undefined) return this;
    if (signature === null) {
      this.doc = AspectSchema.parse(updateAspect(this.doc, { signature: null } as any));
      return this;
    }
    this.doc = AspectSchema.parse(updateAspect(this.doc, { signature } as any));
    return this;
  }
  mergeSignature(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.signature ?? {}) as Record<string, unknown>;
    this.doc = AspectSchema.parse(
      updateAspect(this.doc, { signature: { ...current, ...patch } } as any),
    );
    return this;
  }

  setFacets(facets?: Record<string, unknown> | null): this {
    // undefined -> preserve; null -> clear to {}; object -> replace
    if (facets === undefined) return this;
    if (facets === null) {
      this.doc = AspectSchema.parse(updateAspect(this.doc, { facets: {} } as any));
      return this;
    }
    this.doc = AspectSchema.parse(updateAspect(this.doc, { facets } as any));
    return this;
  }
  mergeFacets(patch: Record<string, unknown> | null | undefined): this {
    if (!patch || Object.keys(patch).length === 0) return this;
    const current = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    this.doc = AspectSchema.parse(
      updateAspect(this.doc, { facets: { ...current, ...patch } } as any),
    );
    return this;
  }
  mergeFacet(ns: string, patch: Record<string, unknown> | null | undefined): this {
    if (!ns || !patch || Object.keys(patch).length === 0) return this;
    const all = (this.doc.shape.facets ?? {}) as Record<string, unknown>;
    const prev = (all[ns] as Record<string, unknown>) ?? {};
    const nextNs = { ...prev, ...patch };
    const nextAll = { ...all, [ns]: nextNs };
    this.doc = AspectSchema.parse(updateAspect(this.doc, { facets: nextAll } as any));
    return this;
  }
}
