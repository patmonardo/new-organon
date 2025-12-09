import type { FormModel } from '../sdsl/form-model';
import type { DataView } from '../data/sdsl';
import type { ExecutionOptions } from './polars-engine';

export type RowLike = Record<string, unknown>;

export interface SemanticResult {
  plan: string;
  rows: RowLike[];
  meta?: Record<string, unknown>;
}

export interface SemanticDataService {
  execute(view: DataView, options?: ExecutionOptions): Promise<SemanticResult>;
}

export interface HydratorContext {
  /** Arbitrary params passed from the controller (e.g., customerId). */
  params?: Record<string, unknown>;
  /** Execution overrides (limit, etc.). */
  execution?: ExecutionOptions;
}

export type ValueTransform = (value: unknown, row: RowLike, context: HydratorContext) => unknown;

export interface FormBinding {
  fieldId: string;
  source: string;
  transform?: ValueTransform;
  fallback?: unknown;
}

export interface CollectionBinding {
  id: string;
  source?: string;
  select?: string[];
  limit?: number;
  fieldId?: string;
  derive?: (rows: RowLike[], context: HydratorContext) => RowLike[];
  transform?: (items: RowLike[], context: HydratorContext) => RowLike[];
}

export interface MetricBinding {
  name: string;
  source?: string;
  derive?: (rows: RowLike[], context: HydratorContext) => unknown;
  fieldId?: string;
  transform?: (value: unknown, rows: RowLike[], context: HydratorContext) => unknown;
  fallback?: unknown;
}

export interface HydratorSpec {
  id: string;
  view: (context: HydratorContext) => DataView;
  fields?: FormBinding[];
  collections?: CollectionBinding[];
  metrics?: MetricBinding[];
  /** Map from source token (e.g., "$plan" or "$meta.engine") to form field id. */
  metaFields?: Record<string, string>;
}

export interface HydratorSnapshot {
  id: string;
  plan: string;
  rows: RowLike[];
  metrics: Record<string, unknown>;
  collections: Record<string, RowLike[]>;
  assignedFields: string[];
  timestamp: number;
  meta?: Record<string, unknown>;
}

export class SemanticHydrator {
  constructor(private readonly service: SemanticDataService) {}

  async hydrate(
    model: FormModel,
    spec: HydratorSpec,
    context: HydratorContext = {}
  ): Promise<HydratorSnapshot> {
    const view = spec.view(context);
    const result = await this.service.execute(view, context.execution);
    const rows = result.rows ?? [];
    const primaryRow = rows[0] ?? {};
    const assignedFields = new Set<string>();

    const assignField = (fieldId: string, value: unknown) => {
      model.setField(fieldId, value);
      assignedFields.add(fieldId);
    };

    if (spec.fields) {
      for (const binding of spec.fields) {
        const raw = resolveValue(binding.source, primaryRow, rows, result, context);
        let value = raw ?? binding.fallback;
        if (binding.transform) {
          value = binding.transform(value, primaryRow, context);
        }
        if (value !== undefined) {
          assignField(binding.fieldId, value);
        }
      }
    }

    const collections: Record<string, RowLike[]> = {};
    if (spec.collections) {
      for (const binding of spec.collections) {
        let items: RowLike[];
        if (binding.derive) {
          items = binding.derive(rows, context);
        } else if (binding.source) {
          const raw = resolveValue(binding.source, primaryRow, rows, result, context);
          items = Array.isArray(raw) ? raw as RowLike[] : [];
        } else {
          items = rows as RowLike[];
        }

        if (binding.select && binding.select.length > 0) {
          items = items.map(item => pickFields(item, binding.select!));
        }

        if (binding.limit !== undefined) {
          items = items.slice(0, binding.limit);
        }

        if (binding.transform) {
          items = binding.transform(items, context);
        }

        collections[binding.id] = items;
        if (binding.fieldId) {
          assignField(binding.fieldId, items);
        }
      }
    }

    const metrics: Record<string, unknown> = {};
    if (spec.metrics) {
      for (const binding of spec.metrics) {
        let metricValue: unknown;
        if (binding.derive) {
          metricValue = binding.derive(rows, context);
        } else if (binding.source) {
          metricValue = resolveValue(binding.source, primaryRow, rows, result, context);
        }
        if (metricValue === undefined) {
          metricValue = binding.fallback;
        }
        if (binding.transform) {
          metricValue = binding.transform(metricValue, rows, context);
        }
        if (metricValue !== undefined) {
          metrics[binding.name] = metricValue;
          if (binding.fieldId) {
            assignField(binding.fieldId, metricValue);
          }
        }
      }
    }

    if (spec.metaFields) {
      for (const [source, fieldId] of Object.entries(spec.metaFields)) {
        const value = resolveValue(source, primaryRow, rows, result, context);
        if (value !== undefined) {
          assignField(fieldId, value);
        }
      }
    }

    const snapshot: HydratorSnapshot = {
      id: spec.id,
      plan: result.plan,
      rows,
      metrics,
      collections,
      assignedFields: Array.from(assignedFields),
      timestamp: Date.now(),
      meta: result.meta,
    };

    return snapshot;
  }
}

function resolveValue(
  source: string,
  primaryRow: RowLike,
  rows: RowLike[],
  result: SemanticResult,
  context: HydratorContext
): unknown {
  if (!source) {
    return undefined;
  }

  if (source === '$row') {
    return primaryRow;
  }
  if (source.startsWith('$row.')) {
    return resolvePath(primaryRow, source.slice(5));
  }
  if (source === '$rows') {
    return rows;
  }
  if (source === '$plan') {
    return result.plan;
  }
  if (source === '$meta') {
    return result.meta;
  }
  if (source.startsWith('$meta.')) {
    return resolvePath(result.meta ?? {}, source.slice(6));
  }
  if (source === '$params') {
    return context.params;
  }
  if (source.startsWith('$params.')) {
    return resolvePath(context.params ?? {}, source.slice(8));
  }

  return resolvePath(primaryRow, source);
}

function resolvePath(target: unknown, path: string): unknown {
  if (target == null || !path) {
    return undefined;
  }

  return path.split('.').reduce<unknown>((acc, segment) => {
    if (acc == null) {
      return undefined;
    }

    if (Array.isArray(acc)) {
      const index = Number(segment);
      return Number.isInteger(index) ? acc[index] : undefined;
    }

    return (acc as Record<string, unknown>)[segment];
  }, target);
}

function pickFields(source: RowLike, keys: string[]): RowLike {
  const result: RowLike = {};
  for (const key of keys) {
    if (key in source) {
      result[key] = source[key];
    }
  }
  return result;
}
