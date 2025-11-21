/**
 * ActiveShape (Container driver) — ADR 0002 alignment
 *
 * Deterministic helpers for signature and facet extraction and a Zod-backed
 * creator for producing canonical ActiveShape instances from form-layer input.
 */
import { createHash } from 'crypto';
import BaseDriver from '../core/driver';
import type { ProcessorInputs } from '../core/contracts';
import { createWorld } from '../../schema/world';
import { ActiveFactory, type ActiveShape } from '../../schema/active';
import type { Repository } from '../../repository/repo';
import { makeInMemoryRepository } from '../../repository/memory';
import { ShapeSchema, type Shape } from '../../schema/shape';
import { getCore, getShape } from '../../schema/base';
import { ShapeEngine } from '../../relative/form/shape/shape-engine';
import type { EventBus } from '../core/bus';

/**
 * Loose input type for shape payloads coming from the form layer.
 * Prefer validating these via ShapeSchema.parse(...) before use.
 */
export type ShapeInput = unknown;

export type Facet = {
  key: string;
  type?: string;
  value?: any;
};

/**
 * Stable hashing helper used to produce deterministic signatures for shapes.
 */
function stableHash(value: unknown): string {
  function canonicalize(v: unknown): any {
    if (v === null || typeof v !== 'object') return v;
    if (Array.isArray(v)) return v.map(canonicalize);
    const keys = Object.keys(v as Record<string, any>).sort();
    const out: Record<string, any> = {};
    for (const k of keys) out[k] = canonicalize((v as Record<string, any>)[k]);
    return out;
  }
  const s = JSON.stringify(canonicalize(value));
  return createHash('sha256').update(s).digest('hex');
}

/**
 * Compute a deterministic signature for a Shape.
 *
 * Preference order:
 * - Use shape.core.id when present.
 * - Otherwise hash a stable selection of fields (type, name, property keys).
 */
export function computeSignature(shape: ShapeInput): string {
  if (!shape) return '';
  try {
    const parsed = ShapeSchema.parse(shape as any) as Shape;
    const core = getCore(parsed) ?? getShape(parsed)?.core ?? (parsed as any);
    if (core && core.id) return String(core.id);
    const seed = {
      type: core?.type ?? (parsed as any).type,
      name: core?.name ?? (parsed as any).name,
      props: extractShapeKeys(parsed).slice(0, 50),
    };
    return stableHash(seed);
  } catch {
    // fallback: hash raw input
    const seed = {
      type: (shape as any)?.type,
      name: (shape as any)?.name,
      props: extractShapeKeys(shape as any).slice(0, 50),
    };
    return stableHash(seed);
  }
}

/**
 * Extract salient property keys from a shape definition.
 *
 * Tolerant to both FormShape and legacy payloads.
 */
export function extractShapeKeys(shape: ShapeInput): string[] {
  if (!shape || typeof shape !== 'object') return [];
  const s = shape as any;
  const candidates: string[] = [];

  const formShape = (() => {
    try {
      const parsed = ShapeSchema.parse(s) as Shape;
      return getShape(parsed) ?? parsed;
    } catch {
      return s;
    }
  })();

  if (Array.isArray(formShape.properties)) {
    for (const p of formShape.properties) {
      if (typeof p === 'string') candidates.push(p);
      else if (p && typeof p === 'object' && p.name)
        candidates.push(String(p.name));
    }
  }

  if (
    formShape.properties &&
    typeof formShape.properties === 'object' &&
    !Array.isArray(formShape.properties)
  ) {
    candidates.push(...Object.keys(formShape.properties));
  }

  if (formShape.fields && typeof formShape.fields === 'object') {
    candidates.push(...Object.keys(formShape.fields));
  }

  const meta = new Set(['id', 'type', 'name', 'description', 'title']);
  for (const k of Object.keys(formShape)) if (!meta.has(k)) candidates.push(k);

  return Array.from(new Set(candidates)).sort();
}

/**
 * Build lightweight deterministic facets for a shape.
 *
 * Facets are compact key/type/value triples intended for engine heuristics
 * and reflect-stage processing.
 */
export function extractFacets(shape: ShapeInput): Facet[] {
  const keys = extractShapeKeys(shape);
  const s = shape as any;
  const facets: Facet[] = [];
  for (const k of keys) {
    const v =
      s.properties && s.properties[k]
        ? s.properties[k]
        : s.fields && s.fields[k]
        ? s.fields[k]
        : undefined;
    facets.push({
      key: k,
      type: typeof v,
      value: typeof v === 'string' || typeof v === 'number' ? v : undefined,
    });
  }
  facets.push({ key: '__propCount', value: keys.length, type: 'number' });
  return facets;
}

/**
 * Produce a Zod-validated ActiveShape from arbitrary input.
 *
 * This function maps form-layer payloads into the canonical ActiveShape
 * representation using the ActiveFactory helper.
 */
export function toActiveShape(input: unknown): ActiveShape {
  return ActiveFactory.parseShape(input ?? {}) as ActiveShape;
}

/**
 * Construct a canonical ActiveShape from a validated Shape input.
 *
 * Steps:
 *  - Normalize/validate the incoming payload using ShapeSchema.parse(...)
 *  - Compute a deterministic signature
 *  - Assemble candidate ActiveShape fields and validate via ActiveFactory
 */
export function fromFormShape(input: ShapeInput): ActiveShape {
  const parsed = ShapeSchema.parse(input as any) as Shape;
  const shapeCore = (getShape(parsed)?.core ?? (parsed as any)) as any;

  const signature = computeSignature(parsed);
  const candidate = {
    id: shapeCore?.id ?? undefined,
    type: shapeCore?.type ?? undefined,
    name: shapeCore?.name ?? undefined,
    signature,
    facets: extractFacets(parsed),
    raw: parsed,
  };

  return ActiveFactory.parseShape(candidate as any) as ActiveShape;
}

/**
 * Exposed functional surface for shape utilities.
 */
export default {
  computeSignature,
  extractFacets,
  fromFormShape,
  toActiveShape,
};

// --- Driver API ------------------------------------------------------------

/**
 * EssenceDriver
 *
 * Driver that exposes shape-centric engine helpers and integrates with the
 * ShapeEngine. Keeps deterministic utility functions available while
 * providing class-based driver ergonomics.
 */
export class EssenceDriver extends BaseDriver {
  constructor() {
    super('EssenceDriver');
  }

  /**
   * Assemble a minimal world surface consumed by processors.
   */
  assemble(input: ProcessorInputs) {
    return createWorld({
      type: 'system.World',
      name: 'Essence',
      horizon: { essence: { shapes: (input?.shapes ?? []).length } },
    });
  }

  /**
   * Delegate helpers to the functional surface for determinism.
   */
  computeSignature(shape: ShapeInput) {
    return computeSignature(shape);
  }

  extractFacets(shape: ShapeInput) {
    return extractFacets(shape);
  }

  fromFormShape(shape: ShapeInput) {
    return fromFormShape(shape);
  }

  /**
   * BaseDriver Active conversion hooks (shape-focused).
   */
  toActive(input: unknown): ActiveShape {
    return toActiveShape(input);
  }

  fromActive(input: ActiveShape) {
    return input;
  }

  /**
   * Create a ShapeEngine instance with an in-memory repository by default.
   */
  private createEngine(repo?: Repository<Shape>, bus?: EventBus) {
    const r: Repository<Shape> =
      (repo as Repository<Shape> | undefined) ??
      (makeInMemoryRepository(
        ShapeSchema as any,
      ) as unknown as Repository<Shape>);
    return new ShapeEngine(r, bus);
  }

  private toActiveBatch(inputs: unknown[] = []): ActiveShape[] {
    return (inputs ?? []).map((i) => toActiveShape(i ?? {}));
  }

  /**
   * Process an array of shapes through the ShapeEngine.
   */
  async processShapes(
    shapes: unknown[],
    opts?: {
      repo?: Repository<Shape>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createEngine(opts?.repo, opts?.bus);
    const actives = this.toActiveBatch(shapes);
    return engine.process(
      actives as any,
      opts?.particulars ?? [],
      opts?.context,
    );
  }

  /**
   * Commit shapes by processing and then committing engine actions.
   */
  async commitShapes(
    shapes: unknown[],
    opts?: {
      repo?: Repository<Shape>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createEngine(opts?.repo, opts?.bus);
    const actives = this.toActiveBatch(shapes);
    const { actions, snapshot } = await engine.process(
      actives as any,
      opts?.particulars ?? [],
      opts?.context,
    );
    const commitResult = await engine.commit(actions, snapshot);

    // normalize events so callers can rely on payload.shape.core.id
    const normalizedEvents = (commitResult?.events ?? []).map((e: any) =>
      this.normalizeEvent(e),
    );
    const normalizedCommitResult = {
      ...commitResult,
      events: normalizedEvents,
    };

    return { actions, snapshot, commitResult: normalizedCommitResult } as const;
  }
}

/**
 * Default driver instance for convenience.
 */
export const DefaultEssenceDriver = new EssenceDriver();

/**
 * Functional wrappers delegating to the default driver instance.
 */
export async function processShapes(
  shapes: unknown[],
  opts?: {
    repo?: Repository<Shape>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultEssenceDriver.processShapes(shapes, opts);
}

export async function commitShapes(
  shapes: unknown[],
  opts?: {
    repo?: Repository<Shape>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultEssenceDriver.commitShapes(shapes, opts);
}
