/**
 * SDSL Core - Species DSL (Simplified)
 *
 * Malloy-inspired semantic modeling for standalone BI.
 * All based on Specs - spec-based architecture.
 */

// =============================================================================
// TYPES
// =============================================================================

export interface MeasureDefinition {
  type: 'sum' | 'count' | 'avg' | 'min' | 'max' | 'custom';
  field?: string;
  sql?: string;
  label?: string;
}

export interface DimensionDefinition {
  field: string;
  truncation?: 'year' | 'quarter' | 'month' | 'day' | 'hour';
  label?: string;
}

export interface JoinDefinition {
  model: DataModel;
  on: string;
  type: 'left' | 'inner' | 'full';
}

export interface DataModelConfig {
  name: string;
  source: string;
  measures?: Record<string, MeasureDefinition>;
  dimensions?: Record<string, DimensionDefinition | string>;
  joins?: Record<string, JoinDefinition>;
}

export interface ViewQuery {
  // Basic query (backward compatible)
  group_by?: string[];
  aggregate?: string[];
  filter?: Record<string, any>;
  limit?: number;
  order_by?: Array<{ field: string; direction: 'asc' | 'desc' }>;

  // Malloy-Lite extensions (optional, non-breaking)
  where?: FilterDefinition[];  // Structured filters
  parameters?: ViewParameter[]; // View parameters
}

// Malloy-Lite types
export interface FilterDefinition {
  field: string;
  operator: '=' | '!=' | '>' | '<' | '>=' | '<=' | 'in' | 'not in' | 'like' | 'is null' | 'is not null';
  value: any;
  logic?: 'and' | 'or';
}

export interface ViewParameter {
  name: string;
  type: 'string' | 'number' | 'date' | 'boolean';
  default?: any;
}

// ViewSpec - Reusable query definition (Malloy-Lite)
export interface ViewSpec {
  id: string;
  name?: string;
  model: string;  // Reference to DataModel name
  group_by?: string[];
  aggregate?: string[];
  where?: FilterDefinition[];
  filter?: Record<string, any>;  // Simplified filter (backward compat)
  limit?: number;
  order_by?: Array<{ field: string; direction: 'asc' | 'desc' }>;
  parameters?: ViewParameter[];
}

// =============================================================================
// MODEL
// =============================================================================

export class DataModel {
  constructor(public config: DataModelConfig) {}

  // Create view from ViewQuery (backward compatible)
  view(query: ViewQuery): DataView {
    return new DataView(this, query);
  }

  // Create view from ViewSpec (Malloy-Lite)
  viewFromSpec(spec: ViewSpec): DataView {
    // Validate that spec.model matches this model
    if (spec.model !== this.config.name) {
      throw new Error(`ViewSpec model "${spec.model}" does not match model "${this.config.name}"`);
    }

    // Convert ViewSpec to ViewQuery
    const query: ViewQuery = {
      group_by: spec.group_by,
      aggregate: spec.aggregate,
      where: spec.where,
      filter: spec.filter,
      limit: spec.limit,
      order_by: spec.order_by,
      parameters: spec.parameters,
    };

    return new DataView(this, query);
  }
}

// =============================================================================
// VIEW
// =============================================================================

export class DataView {
  constructor(
    public model: DataModel,
    public query: ViewQuery
  ) {}

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

export function defineModel(config: DataModelConfig): DataModel {
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

export function dimension(
  field: string,
  truncation?: DimensionDefinition['truncation']
): DimensionDefinition {
  return { field, truncation };
}
