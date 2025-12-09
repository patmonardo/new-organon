/**
 * MalloyModel - Data Provider (Not a Viewer)
 *
 * Provides query results from Malloy views.
 * Mirrors React pattern: Model provides data, Controller orchestrates.
 */

import type { DataModel } from '../../../data/sdsl';
import type { ViewSpec } from '../../../data/sdsl';
import type { ExecutionResult } from '../../../execution';
import type { PolarsExecutionEngine } from '../../../execution';

/**
 * MalloyModel - Provides perfect data for Views
 *
 * Similar to React Model pattern:
 * - Provides data (not a viewer)
 * - Offers special interfaces (executeView)
 * - Not aware of Views
 */
export class MalloyModel {
  constructor(
    private dataModel: DataModel,
    private engine: PolarsExecutionEngine
  ) {}

  /**
   * Execute a view and return perfect data for View
   */
  async executeView(viewSpec: ViewSpec): Promise<ExecutionResult> {
    // Create view from spec
    const view = this.dataModel.viewFromSpec(viewSpec);

    // Execute via engine
    const result = await this.engine.execute(view);

    // Return perfect data for View
    return result;
  }

  /**
   * Get the underlying DataModel
   */
  getDataModel(): DataModel {
    return this.dataModel;
  }
}

