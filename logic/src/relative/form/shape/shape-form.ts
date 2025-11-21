import { ShapeSchema } from '@schema';
import type { Shape } from '@schema';
import { createShape } from '@schema';
import { updateShape } from '@schema';

type BaseState = Shape["shape"]["state"];

/**
 * FormShape — Principle of Shape of Forms.
 */
export class FormShape {
  private _doc: Shape;

  private constructor(doc: Shape) {
    // Validate on construction to keep invariants tight
    this._doc = ShapeSchema.parse(doc);
  }

  // Factory: create from inputs (delegates to schema helper)
  static create(input: {
    type: string;
    name?: string;
    id?: string;
    state?: Record<string, unknown>;
    signature?: Record<string, unknown>;
    facets?: Record<string, unknown>;
  }): FormShape {
    const doc = createShape(input as any);
    return new FormShape(doc);
  }

  // Factory: wrap an existing Shape doc (validates)
  static from(doc: Shape): FormShape {
    return new FormShape(doc);
  }

  // Accessors required by ShapeEngine
  get id(): string {
    return this._doc.shape.core.id;
  }
  get type(): string {
    return this._doc.shape.core.type;
  }
  get name(): string | undefined {
    return this._doc.shape.core.name;
  }
  get state(): BaseState {
    return this._doc.shape.state;
  }
  get signature(): Record<string, unknown> | undefined {
    return this._doc.shape.signature as any;
  }
  get facets(): Record<string, unknown> {
    return (this._doc.shape.facets ?? {}) as any;
  }
  // Data is stored under facets.data
  get data(): unknown {
    return (this._doc.shape.facets as any)?.data;
  }

  toSchema(): Shape {
    return this._doc;
  }

  toJSON(): any {
    // Include top-level id for convenience in tests/telemetry
    return { id: this._doc.shape.core.id, ...this._doc };
  }

  // Mutators (chainable)

  setName(name?: string) {
    this._doc = updateShape(this._doc, { core: { name } });
    return this;
  }

  setType(type: string) {
    this._doc = updateShape(this._doc, { core: { type } as any });
    return this;
  }

  setState(state: Record<string, unknown>) {
    // Replace state
    this._doc = updateShape(this._doc, { state: state as any });
    return this;
  }

  patchState(patch: Record<string, unknown>) {
    // Merge state fields
    this._doc = updateShape(this._doc, { state: patch as any });
    return this;
  }

  // Data helpers
  setData(data: unknown) {
    const facets = { ...this.facets, data };
    this._doc = updateShape(this._doc, { facets });
    return this;
  }

  clearData() {
    const { data: _drop, ...rest } = this.facets;
    this._doc = updateShape(this._doc, { facets: rest });
    return this;
  }

  mergeFacets(facets: Record<string, unknown>) {
    const merged = { ...(this._doc.shape.facets ?? {}), ...facets };
    this._doc = updateShape(this._doc, { facets: merged });
    return this;
  }

  setFacets(facets: Record<string, unknown>) {
    this._doc = updateShape(this._doc, { facets });
    return this;
  }

  patchSignature(partial: Record<string, unknown>) {
    const merged = { ...(this._doc.shape.signature ?? {}), ...partial };
    this._doc = updateShape(this._doc, { signature: merged });
    return this;
  }

  setSignature(sig?: Record<string, unknown>) {
    if (sig === undefined) {
      return this.clearSignature();
    }
    this._doc = updateShape(this._doc, { signature: sig });
    return this;
  }

  clearSignature() {
    // Use null sentinel; schema clears to undefined
    this._doc = updateShape(this._doc, { signature: null as any });
    return this;
  }
}
