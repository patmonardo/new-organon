import { z } from 'zod';
import { DataModelConfig, DataModel, MeasureDefinition, DimensionDefinition, JoinDefinition } from './sdsl';
import { MalloyIR, SourceIR, MeasureIR, DimensionIR, JoinIR, QueryIR } from '../schema/malloy-ir';

/**
 * Malloy IR Generator - Generate Malloy IR from defineModel config
 * 
 * This generates Malloy IR from our TS-JSON defineModel pattern,
 * allowing us to embed rich Malloy IR directly in our schemas.
 */

// Convert MeasureDefinition to Malloy IR
function convertMeasureToMalloyIR(measure: MeasureDefinition): MeasureIR {
  return {
    type: measure.type,
    field: measure.field,
    expression: measure.sql,
  };
}

// Convert DimensionDefinition to Malloy IR
function convertDimensionToMalloyIR(dimension: DimensionDefinition | string): DimensionIR {
  if (typeof dimension === 'string') {
    return { field: dimension };
  }
  return {
    field: dimension.field,
    truncation: dimension.truncation,
  };
}

// Convert JoinDefinition to Malloy IR
function convertJoinToMalloyIR(join: JoinDefinition, modelName: string): JoinIR {
  return {
    model: modelName, // Use model name as reference
    on: join.on,
    type: join.type,
  };
}

// Generate Malloy IR from DataModelConfig
export function generateMalloyIRFromConfig<T extends Record<string, any>>(
  config: DataModelConfig<T>
): MalloyIR {
  // Convert measures
  const measures: Record<string, MeasureIR> = {};
  if (config.measures) {
    for (const [name, measure] of Object.entries(config.measures)) {
      measures[name] = convertMeasureToMalloyIR(measure);
    }
  }

  // Convert dimensions
  const dimensions: Record<string, DimensionIR> = {};
  if (config.dimensions) {
    for (const [name, dimension] of Object.entries(config.dimensions)) {
      dimensions[name] = convertDimensionToMalloyIR(dimension);
    }
  }

  // Convert joins
  const joins: Record<string, JoinIR> = {};
  if (config.joins) {
    for (const [name, join] of Object.entries(config.joins)) {
      // Get model name from join.model.config.name
      const modelName = (join.model as any).config?.name || name;
      joins[name] = convertJoinToMalloyIR(join, modelName);
    }
  }

  // Create source IR
  const sourceIR: SourceIR = {
    type: 'table',
    sql: `SELECT * FROM ${config.source}`, // Generate SQL from source
    measures: Object.keys(measures).length > 0 ? measures : undefined,
    dimensions: Object.keys(dimensions).length > 0 ? dimensions : undefined,
    joins: Object.keys(joins).length > 0 ? joins : undefined,
  };

  // Create Malloy IR
  const malloyIR: MalloyIR = {
    sources: {
      [config.name]: sourceIR,
    },
    queries: {}, // Queries can be generated later from views
  };

  return malloyIR;
}

// Enhanced DataModel with Malloy IR
export class DataModelWithMalloyIR<T extends Record<string, any>> extends DataModel<T> {
  public readonly malloyIR: MalloyIR;

  constructor(config: DataModelConfig<T>) {
    super(config);
    this.malloyIR = generateMalloyIRFromConfig(config);
  }

  // Generate query IR from view query
  generateQueryIR(queryName: string, query: { group_by?: string[]; aggregate?: string[]; filter?: any; limit?: number }): QueryIR {
    return {
      source: this.config.name,
      group_by: query.group_by,
      aggregate: query.aggregate,
      filter: query.filter,
      limit: query.limit,
    };
  }

  // Add query to Malloy IR
  addQuery(queryName: string, query: QueryIR): void {
    if (!this.malloyIR.queries) {
      this.malloyIR.queries = {};
    }
    this.malloyIR.queries[queryName] = query;
  }
}

// Enhanced defineModel that generates Malloy IR
export function defineModelWithMalloyIR<T extends Record<string, any>>(
  config: DataModelConfig<T>
): DataModelWithMalloyIR<T> {
  return new DataModelWithMalloyIR(config);
}

// Type guard to check if DataModel has Malloy IR
export function hasMalloyIR(model: DataModel<any>): model is DataModelWithMalloyIR<any> {
  return 'malloyIR' in model;
}

