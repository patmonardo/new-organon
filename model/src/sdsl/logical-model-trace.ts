import type { DataModel, DataView } from '../data/sdsl';
import type { FactTraceEvent } from './fact-trace';
import type { EventMeta, FactStoreInfo } from './terminology';

export const LOGICAL_MODEL_TRACE_KINDS = {
  model: 'model.define',
  view: 'view.define',
  viewPlan: 'view.plan',
} as const;

function mergeFactStore(
  base: FactStoreInfo | undefined,
  override: Partial<FactStoreInfo> | undefined,
): FactStoreInfo | undefined {
  if (!base && !override) return undefined;
  return {
    ...(base ?? {}),
    ...(override ?? {}),
    ids:
      override?.ids !== undefined
        ? override.ids
        : base?.ids !== undefined
          ? base.ids
          : undefined,
  };
}

export function dataModelToFactTraceEvent(
  model: DataModel,
  opts?: { id?: string; meta?: EventMeta; factStore?: Partial<FactStoreInfo> },
): FactTraceEvent {
  const id = opts?.id ?? model.config.name;

  const meta: EventMeta | undefined =
    opts?.meta || opts?.factStore
      ? {
          ...(opts?.meta ?? {}),
          factStore: mergeFactStore(
            {
              store: 'sdsl',
              kind: 'model',
              op: 'assert',
              ids: [id],
              note: `DataModel defined: ${model.config.name}`,
            },
            opts?.factStore,
          ),
        }
      : {
          factStore: {
            store: 'sdsl',
            kind: 'model',
            op: 'assert',
            ids: [id],
            note: `DataModel defined: ${model.config.name}`,
          },
        };

  return {
    kind: LOGICAL_MODEL_TRACE_KINDS.model,
    payload: model.config,
    meta,
  };
}

export function dataViewToFactTraceEvents(
  view: DataView,
  opts?: {
    id?: string;
    meta?: EventMeta;
    viewFactStore?: Partial<FactStoreInfo>;
    planFactStore?: Partial<FactStoreInfo>;
  },
): FactTraceEvent[] {
  const viewId = opts?.id ?? `${view.model.config.name}:view`;

  const viewMeta: EventMeta = {
    ...(opts?.meta ?? {}),
    factStore: mergeFactStore(
      {
        store: 'sdsl',
        kind: 'view',
        op: 'assert',
        ids: [viewId],
        note: `DataView defined for model: ${view.model.config.name}`,
      },
      opts?.viewFactStore,
    ),
  };

  const planMeta: EventMeta = {
    ...(opts?.meta ?? {}),
    factStore: mergeFactStore(
      {
        store: 'sdsl',
        kind: 'view.plan',
        op: 'project',
        ids: [viewId],
        note: `DataView plan projected for model: ${view.model.config.name}`,
      },
      opts?.planFactStore,
    ),
  };

  return [
    {
      kind: LOGICAL_MODEL_TRACE_KINDS.view,
      payload: {
        id: viewId,
        model: view.model.config.name,
        query: view.query,
      },
      meta: viewMeta,
    },
    {
      kind: LOGICAL_MODEL_TRACE_KINDS.viewPlan,
      payload: {
        id: viewId,
        model: view.model.config.name,
        plan: view.toPlan(),
      },
      meta: planMeta,
    },
  ];
}

export function modelAndViewToFactTraceEvents(
  model: DataModel,
  view: DataView,
  opts?: {
    modelId?: string;
    viewId?: string;
    meta?: EventMeta;
  },
): FactTraceEvent[] {
  return [
    dataModelToFactTraceEvent(model, { id: opts?.modelId, meta: opts?.meta }),
    ...dataViewToFactTraceEvents(view, { id: opts?.viewId, meta: opts?.meta }),
  ];
}
