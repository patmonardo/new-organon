import type { Shape } from '@schema';
import type { Context } from '@schema';
import type { Morph } from '@schema';
import type { Entity } from '@schema';
import type { Property } from '@schema';
// reflect.ts no longer exports these types; declare local types compatible with reflectStage output
type ThingLike = {
  id: string;
  type?: string;
  essence?: any;
  properties?: unknown;
};
type PropertyLike = {
  id: string;
  entity?: { id?: string; type?: string } | string;
  key?: string;
  value?: unknown;
};
type ReflectResult = {
  thingFacets: Record<string, unknown>;
  propertyFacets?: Record<string, unknown> | undefined;
  signatures: Record<string, any> | undefined;
  evidence: string[];
};
import type { KriyaActionResult } from './action';
import type { Relation } from '@schema';

export type Principles = {
  shapes: Shape[];
  contexts: Context[];
  morphs: Morph[];
};

export type EssenceGraph = {
  entities: Entity[];
  properties: Property[];
  relations: Relation[];
};

export type Projections = {
  indexes?: Record<string, unknown>;
  views?: Record<string, unknown>;
};

export type Controls = {
  actions: Array<{ kind: string; args?: unknown }>;
};

export type Work = {
  tasks: Array<{ id: string; kind: string; args?: unknown }>;
  workflow: { nodes: string[]; edges: Array<{ from: string; to: string }> };
};

export type KriyaOptions = {
  // Ground / runtime controls
  fixpointMaxIters?: number;
  budgetMs?: number;
  // pass-through options for reflect stage (e.g. { contextId })
  reflectOpts?: Record<string, unknown>;
  // Orchestrator / processor controls
  projectContent?: boolean;
  contentIndexSource?: 'inputs' | 'projected' | 'both';
  deriveSyllogistic?: boolean;
  // persistence commit triad: { relation, property, bus }
  triad?: any;
  commitGround?: boolean;
  // optional action thresholds passed through to action stage
  actionThreshold?: number;
  // extendable for future flags
  [key: string]: unknown;
};

// Note: the public KriyaResult (orchestrator-level result shape) is defined in
// `core/orchestrator.ts`. `runCycle` returns an internal cycle result that is
// compatible with that shape but we avoid exporting the same symbol here to
// prevent re-export collisions.

export type StageFns = {
  seed: (p: Principles) => Promise<Pick<EssenceGraph, 'entities'>>;
  contextualize: (
    p: Principles,
    g: Pick<EssenceGraph, 'entities'>,
  ) => Promise<Pick<EssenceGraph, 'properties'>>;
  // optional reflect stage: inspects seeded entities + contextualized properties and returns reflective facets/signatures
  reflect?: (
    things: ThingLike[],
    properties: PropertyLike[],
    opts?: KriyaOptions,
  ) => Promise<ReflectResult>;
  ground: (
    p: Principles,
    g: Pick<EssenceGraph, 'entities' | 'properties'>,
    opts?: KriyaOptions,
  ) => Promise<Pick<EssenceGraph, 'relations'>>;
  // optional action stage: computes actions (reciprocal effects) from the graph and reflect facets
  action?: (
    graph: EssenceGraph,
    reflect?: ReflectResult,
    opts?: { threshold?: number; contextId?: string },
  ) => Promise<KriyaActionResult>;
  model: (g: EssenceGraph) => Promise<Projections>;
  control: (g: EssenceGraph, proj: Projections) => Promise<Controls>;
  plan: (ctrl: Controls) => Promise<Work>;
};

export type CycleResult = {
  graph: EssenceGraph;
  projections: Projections;
  controls: Controls;
  work: Work;
  reflect?: ReflectResult;
  action?: KriyaActionResult;
};

export async function runCycle(
  p: Principles,
  fns: StageFns,
  opts?: KriyaOptions,
): Promise<CycleResult> {
  // Normalize options and provide small observability & budget checks
  const _opts: KriyaOptions = {
    fixpointMaxIters: 100,
    budgetMs: undefined,
    ...opts,
  };
  const startTs = Date.now();
  const log =
    typeof _opts.log === 'function'
      ? (_opts.log as (m: string, d?: any) => void)
      : undefined;
  function checkBudget(stage?: string) {
    if (
      typeof _opts.budgetMs === 'number' &&
      Date.now() - startTs > _opts.budgetMs
    ) {
      const msg = `kriya.runCycle: budget exceeded${
        stage ? ` at ${stage}` : ''
      }`;
      log?.(msg);
      throw new Error(msg);
    }
  }
  // Validate required stages early
  if (
    typeof fns.seed !== 'function' ||
    typeof fns.contextualize !== 'function' ||
    typeof fns.ground !== 'function' ||
    typeof fns.model !== 'function' ||
    typeof fns.control !== 'function' ||
    typeof fns.plan !== 'function'
  ) {
    throw new Error(
      'runCycle: missing required stage function (seed/contextualize/ground/model/control/plan)',
    );
  }
  // Ring 1 — Essence
  const seeded = await fns.seed(p); // Shape → Entity
  checkBudget('seed');
  log?.('kriya:seed completed', { elapsed: Date.now() - startTs });
  const ctxed = await fns.contextualize(p, seeded); // Context → Property
  checkBudget('contextualize');
  log?.('kriya:contextualize completed', { elapsed: Date.now() - startTs });
  let reflectResult: ReflectResult | undefined = undefined;
  // Optional reflect stage (compute Citta/vṛtti facets and signatures)
  if (typeof fns.reflect === 'function') {
    // adapt entities/properties to reflect's expected shapes
    // use runtime extraction and any casts to avoid tight coupling to schema types
    const seededAny = seeded as any;
    const ctxAny = ctxed as any;
    const things = (seededAny.entities || []).map((e: any) => {
      const id = e.id ?? e.shape?.core?.id ?? (e as any).core?.id;
      const type = e.type ?? e.shape?.core?.type ?? (e as any).core?.type;
      const essence = e.essence ?? e.ext?.essence ?? undefined;
      return { id, type, essence, properties: undefined };
    });
    const props = (ctxAny.properties || []).map((p0: any) => {
      const id = p0.id ?? p0.shape?.core?.id ?? (p0 as any).core?.id;
      const entityId =
        p0.entity ?? p0.entityId ?? p0.shape?.core?.entity ?? (p0 as any).owner;
      const entityType =
        (p0 as any).entityType ?? p0.shape?.core?.entityType ?? undefined;
      const key = p0.key ?? p0.shape?.core?.key ?? (p0 as any).name;
      const value = (p0 as any).value ?? (p0 as any).default ?? null;
      return { id, entity: { id: entityId, type: entityType }, key, value };
    });
    // call reflect with provided reflect options
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    reflectResult = await fns.reflect!(
      things as any,
      props as any,
      opts?.reflectOpts as any,
    );
  }
  // Pass reflect result to ground as an advisory field on opts so groundStage
  // can consult spectrum/advice without changing public APIs.
  const groundOpts = { ...(opts as any), reflectResult } as any;
  const grounded = await fns.ground(p, { ...seeded, ...ctxed }, groundOpts); // Morph → Relation
  checkBudget('ground');
  log?.('kriya:ground completed', { elapsed: Date.now() - startTs });

  const graph: EssenceGraph = {
    entities: seeded.entities,
    properties: ctxed.properties,
    relations: grounded.relations,
  };

  // Rings 2–3 — read/plan layers
  const projections = await fns.model(graph);
  checkBudget('model');
  log?.('kriya:model completed', { elapsed: Date.now() - startTs });
  const controls = await fns.control(graph, projections);
  checkBudget('control');
  log?.('kriya:control completed', { elapsed: Date.now() - startTs });
  const work = await fns.plan(controls);
  checkBudget('plan');
  log?.('kriya:plan completed', { elapsed: Date.now() - startTs });

  // optional action stage (compute reciprocal effects)
  let actionResult: KriyaActionResult | undefined = undefined;
  if (typeof fns.action === 'function') {
    // pass reflectResult and lightweight opts through (use reflectOpts for context if available)
    const actionOpts = {
      threshold: (opts as any)?.actionThreshold,
      contextId: (opts as any)?.reflectOpts?.contextId,
    };
    actionResult = await fns.action!(graph, reflectResult, actionOpts);
  }

  return {
    graph,
    projections,
    controls,
    work,
    reflect: reflectResult,
    action: actionResult,
  };
}
