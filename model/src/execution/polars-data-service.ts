import { DataView } from '../data/sdsl';
import { PolarsExecutionEngine, ExecutionOptions } from './polars-engine';
import { SemanticDataService, SemanticResult } from './semantic-hydrator';

/**
 * PolarsDataService
 * -----------------
 * Adapts the PolarsExecutionEngine to the SemanticDataService interface.
 * This serves as the "Local Aquifer" implementation.
 */
export class PolarsDataService implements SemanticDataService {
  constructor(private engine: PolarsExecutionEngine) {}

  async execute(view: DataView, options?: ExecutionOptions): Promise<SemanticResult> {
    const result = await this.engine.execute(view, options);

    // Map ExecutionResult to SemanticResult (they are structurally identical for now, but explicit is good)
    return {
      plan: result.plan,
      rows: result.rows,
      meta: result.meta,
    };
  }
}
