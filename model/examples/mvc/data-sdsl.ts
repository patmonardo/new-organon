import { z } from 'zod';

// =============================================================================
// DATA SDSL: The Semantic Layer
// =============================================================================

export type DataType = 'string' | 'number' | 'boolean' | 'date' | 'json';

export interface FieldDefinition {
  type: DataType;
  schema: z.ZodType<any>;
}

export interface MeasureDefinition {
  type: 'sum' | 'count' | 'avg' | 'min' | 'max' | 'custom';
  field?: string; // The field to aggregate
  sql?: string;   // Raw SQL/Expression if needed
  label?: string;
}

export interface DimensionDefinition {
  field: string;
  truncation?: 'year' | 'quarter' | 'month' | 'day' | 'hour';
  label?: string;
}

export interface JoinDefinition {
  model: DataModel<any>;
  on: string; // e.g., "customerId = id"
  type: 'left' | 'inner' | 'full';
}

export interface DataModelConfig<T extends Record<string, any>> {
  name: string;
  source: string; // Table name or file path
  fields: { [K in keyof T]: z.ZodType<T[K]> };
  measures?: Record<string, MeasureDefinition>;
  dimensions?: Record<string, DimensionDefinition | string>; // string shorthand for simple field mapping
  joins?: Record<string, JoinDefinition>;
}

export class DataModel<T extends Record<string, any>> {
  constructor(public config: DataModelConfig<T>) {}

  // Helper to create a view (query) against this model
  view(query: ViewQuery): DataView {
    return new DataView(this, query);
  }
}

export interface ViewQuery {
  group_by?: string[];
  aggregate?: string[]; // Names of measures
  filter?: Record<string, any>;
  limit?: number;
}

export class DataView {
  constructor(public model: DataModel<any>, public query: ViewQuery) {}

  // In a real implementation, this would generate the SQL/Arrow query
  toPlan(): string {
    return JSON.stringify({
      source: this.model.config.source,
      ...this.query
    }, null, 2);
  }
}

// =============================================================================
// HELPERS
// =============================================================================

export function defineModel<T extends Record<string, any>>(config: DataModelConfig<T>): DataModel<T> {
  return new DataModel(config);
}

export function sum(field: string): MeasureDefinition {
  return { type: 'sum', field };
}

export function count(): MeasureDefinition {
  return { type: 'count' };
}

export function avg(field: string): MeasureDefinition {
  return { type: 'avg', field };
}

export function dimension(field: string, truncation?: DimensionDefinition['truncation']): DimensionDefinition {
  return { field, truncation };
}
