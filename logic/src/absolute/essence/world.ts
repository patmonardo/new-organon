import {
  createWorld,
  updateWorld,
  type World,
  type WorldEdge,
} from '../../schema/world';
import type { ProcessorInputs } from '../core/contracts';
import crypto from 'crypto';
import BaseDriver from '../core/driver';
import type { Repository } from '../../repository/repo';
import { makeInMemoryRepository } from '../../repository/memory';
import { PropertySchema, type Property } from '../../schema/property';
import { ActiveFactory, type ActiveProperty } from '../../schema/active';
import { PropertyEngine } from '../../relative/form/property/property-engine';
import type { EventBus } from '../core/bus';
import { getCore, getShape } from '../../schema/base';

const THING = 'system.Thing';
type ThingRef = { id: string; type: typeof THING };

/** Extract entity id across mixed shapes/refs using canonical accessors. */
function pickEntityId(e: any): string | undefined {
  if (!e) return undefined;
  const core = getCore(e) ?? (getShape(e)?.core as any) ?? (e as any);
  const id = core?.id ?? (e as any)?.id;
  return id !== undefined && id !== null ? String(id) : undefined;
}

/** Pick canonical relation kind/type (falls back to 'related_to'). */
function pickKind(r: any): string {
  const core = getCore(r) ?? (getShape(r)?.core as any) ?? (r as any);
  return core?.kind ?? core?.type ?? 'related_to';
}

/** Pick relation direction and normalize to allowed values. */
function pickDirection(r: any): 'directed' | 'bidirectional' {
  const core = getCore(r) ?? (getShape(r)?.core as any) ?? (r as any);
  const d = core?.direction ?? core?.shape?.direction ?? 'directed';
  return d === 'bidirectional' ? 'bidirectional' : 'directed';
}

/** Pick source ThingRef from relation/aspect-like shapes. */
function pickSource(r: any): ThingRef | undefined {
  const shape = getShape(r) ?? (r as any);
  const s = shape?.source ?? (r as any)?.source;
  if (!s || !s?.id) return undefined;
  return { id: String(s.id), type: THING };
}

/** Pick target ThingRef from relation/aspect-like shapes. */
function pickTarget(r: any): ThingRef | undefined {
  const shape = getShape(r) ?? (r as any);
  const t = shape?.target ?? (r as any)?.target;
  if (!t || !t?.id) return undefined;
  return { id: String(t.id), type: THING };
}

function ensureThing(world: World, ref: ThingRef): World {
  const things = world.shape.things ?? [];
  if (things.some((t) => t.id === ref.id)) return world;
  return updateWorld(world, { things: [...things, ref] });
}

function edgeKey(e: WorldEdge): string {
  if (e.direction === 'bidirectional') {
    const a = [e.source.id, e.target.id].sort();
    return `undir|${e.kind}|${a[0]}|${a[1]}`;
  }
  return `dir|${e.kind}|${e.source.id}|${e.target.id}`;
}

export class WorldDriver extends BaseDriver {
  constructor() {
    super('WorldDriver');
  }

  assemble(input: ProcessorInputs): World {
    let world = createWorld({ type: 'system.World', name: 'World' });

    // Entities → Things
    for (const e of input.entities ?? []) {
      const id = pickEntityId(e);
      if (!id) continue;
      world = ensureThing(world, { id, type: THING });
    }

    // Relations → Edges (normalized + deduped)
    const seen = new Set<string>();
    const edges: WorldEdge[] = [];

    for (const r of input.relations ?? []) {
      const source = pickSource(r);
      const target = pickTarget(r);
      if (!source || !target) continue;

      world = ensureThing(ensureThing(world, source), target);

      const edge: WorldEdge = {
        kind: pickKind(r),
        direction: pickDirection(r),
        source,
        target,
      };

      const key = edgeKey(edge);
      if (!seen.has(key)) {
        seen.add(key);
        edges.push(edge);
      }
    }

    // Stable ordering
    const sortedThings = [...(world.shape.things ?? [])].sort((a, b) =>
      a.id.localeCompare(b.id),
    );
    const sortedEdges = edges.sort((a, b) => {
      const k = a.kind.localeCompare(b.kind);
      if (k !== 0) return k;
      const s = a.source.id.localeCompare(b.source.id);
      if (s !== 0) return s;
      const t = a.target.id.localeCompare(b.target.id);
      if (t !== 0) return t;
      return a.direction.localeCompare(b.direction);
    });

    // Horizon — scope from Contexts if present, else Absolute World
    const firstCtx = (input.contexts ?? [])[0];
    const ctxCore =
      getCore(firstCtx) ??
      (getShape(firstCtx)?.core as any) ??
      (firstCtx as any);
    const horizonScope = ctxCore?.name ?? ctxCore?.id ?? 'Absolute World';

    world = updateWorld(world, {
      relations: sortedEdges,
      things: sortedThings,
      horizon: { scope: String(horizonScope) },
    });

    return world;
  }

  indexContent(input: ProcessorInputs): {
    subtleWorldTotal: number;
    grossByThing: Record<string, number>;
  } {
    let subtleWorldTotal = 0;
    const grossByThing: Record<string, number> = {};

    for (const c of input.content) {
      if (c.shape.kind === 'subtle') {
        subtleWorldTotal += 1;
      } else {
        const id = c.shape.of.id;
        grossByThing[id] = (grossByThing[id] ?? 0) + 1;
      }
    }
    return { subtleWorldTotal, grossByThing };
  }

  // --- Property analytics (mates with reflect.ts property facet logic) ---

  computePropertyFacets(properties: WorldProperty[] = []): {
    propertyFacets: Record<string, WorldPropertyFacet>;
    signatures: Record<string, string>;
  } {
    const propertyFacets: Record<string, WorldPropertyFacet> = {};
    const signatures: Record<string, string> = {};
    for (const p of properties) {
      const pid = String((p as any).id);
      const pkey = (p as any).key ?? '';
      const pEntity = (p as any).entity;
      const pEntityId =
        typeof pEntity === 'string' ? pEntity : pEntity && (pEntity as any).id;
      const positing = { key: pkey, entity: pEntity };
      const external = { valueType: typeof (p as any).value };
      const determining = {
        expressive: typeof (p as any).value !== 'undefined',
      };
      propertyFacets[pid] = { positing, external, determining, evidence: [] };
      const sig = stableHash([
        pid,
        String(pEntityId),
        String(pkey),
        String((p as any).value),
      ]);
      signatures[pid] = sig;
      propertyFacets[pid].evidence.push(`sig:${sig}`);
    }
    return { propertyFacets, signatures };
  }

  // World -> ActiveProperty driver (first pass)
  // Produces ActiveProperty instances from world-scoped property-like inputs.

  /**
   * Convert arbitrary input into the canonical ActiveProperty using schema factory.
   * Throws on invalid input.
   */
  toActiveProperty(input: unknown): ActiveProperty {
    return ActiveFactory.parseProperty(input ?? {}) as ActiveProperty;
  }

  /**
   * Produce a canonical ActiveProperty from a world-scoped property record.
   * Attaches provenance computed from facets/signature.
   */
  fromWorldProperty(p: WorldProperty): ActiveProperty {
    const subjectId = typeof p.entity === 'string' ? p.entity : p.entity?.id;
    const id = String(p.id ?? `${subjectId}:${p.key}`);
    // Attach provenance using computed facets/signature for traceability
    const { propertyFacets, signatures } = this.computePropertyFacets([p]);
    const prov = {
      world: true,
      signature: signatures[id] ?? signatures[String(p.id)] ?? null,
      facets: propertyFacets[id] ?? undefined,
    } as any;
    const candidate = {
      id,
      subjectId: String(subjectId ?? 'unknown'),
      key: String(p.key ?? 'unknown'),
      value: p.value,
      // capture primitive dtype to support downstream Qual/Quant analytics
      dtype:
        typeof p.value === 'string'
          ? 'string'
          : typeof p.value === 'number'
          ? 'number'
          : typeof p.value === 'boolean'
          ? 'boolean'
          : undefined,
      active: true,
      provenance: prov,
    } as any;
    return ActiveFactory.parseProperty(candidate) as ActiveProperty;
  }

  batchFromWorld(properties: WorldProperty[] = []) {
    return properties.map((p) => this.fromWorldProperty(p));
  }

  // --- Engine bridge: drive PropertyEngine with ActiveProperty ------------
  private createPropertyEngine(repo?: Repository<Property>, bus?: EventBus) {
    const r: Repository<Property> =
      (repo as Repository<Property> | undefined) ??
      (makeInMemoryRepository(
        PropertySchema as any,
      ) as unknown as Repository<Property>);
    return new PropertyEngine(r, bus);
  }

  async processPropertiesFromWorld(
    properties: WorldProperty[] = [],
    opts?: {
      repo?: Repository<Property>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createPropertyEngine(opts?.repo, opts?.bus);
    const actives = this.batchFromWorld(properties);
    return engine.process(
      actives as any,
      opts?.particulars ?? [],
      opts?.context,
    );
  }

  async commitPropertiesFromWorld(
    properties: WorldProperty[] = [],
    opts?: {
      repo?: Repository<Property>;
      bus?: EventBus;
      particulars?: any[];
      context?: any;
    },
  ) {
    const engine = this.createPropertyEngine(opts?.repo, opts?.bus);
    const actives = this.batchFromWorld(properties);
    const { actions, snapshot } = await engine.process(
      actives as any,
      opts?.particulars ?? [],
      opts?.context,
    );
    const commitResult = await engine.commit(actions, snapshot);
    return { actions, snapshot, commitResult } as const;
  }
}

// Content index: subtle mapped to World (global), gross mapped to Thing (by entity id)
// Backwards-compatible default instance and thin wrappers
export const DefaultWorldDriver = new WorldDriver();

export function assembleWorld(input: ProcessorInputs): World {
  return DefaultWorldDriver.assemble(input);
}

export function indexContent(input: ProcessorInputs) {
  return DefaultWorldDriver.indexContent(input);
}

// --- Property analytics (mates with reflect.ts property facet logic) ---

export type WorldProperty = {
  id: string;
  entity?: { id: string } | string;
  key?: string;
  value?: unknown;
} & Record<string, unknown>;

export type WorldPropertyFacet = {
  positing: Record<string, unknown>;
  external: Record<string, unknown>;
  determining: Record<string, unknown>;
  evidence: string[];
};

function stableHash(inputs: string[]): string {
  const data = inputs.sort().join('|');
  return crypto.createHash('sha1').update(data).digest('hex');
}

/**
 * Compute world-scope property facets and stable signatures.
 * Mirrors reflect.ts property facet/signature logic (no spectrum).
 */
export function computePropertyFacets(properties: WorldProperty[] = []) {
  return DefaultWorldDriver.computePropertyFacets(properties);
}

// World -> ActiveProperty driver (first pass)
// Produces ActiveProperty instances from world-scoped property-like inputs.

export function toActiveProperty(input: unknown) {
  return DefaultWorldDriver.toActiveProperty(input);
}

export function fromWorldProperty(p: WorldProperty) {
  return DefaultWorldDriver.fromWorldProperty(p);
}

export function batchFromWorld(properties: WorldProperty[] = []) {
  return DefaultWorldDriver.batchFromWorld(properties);
}

// Engine bridge wrappers
export function processPropertiesFromWorld(
  properties: WorldProperty[] = [],
  opts?: {
    repo?: Repository<Property>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultWorldDriver.processPropertiesFromWorld(properties, opts);
}

export function commitPropertiesFromWorld(
  properties: WorldProperty[] = [],
  opts?: {
    repo?: Repository<Property>;
    bus?: EventBus;
    particulars?: any[];
    context?: any;
  },
) {
  return DefaultWorldDriver.commitPropertiesFromWorld(properties, opts);
}

export default DefaultWorldDriver;
