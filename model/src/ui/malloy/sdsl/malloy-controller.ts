/**
 * MalloyController - Orchestrator
 *
 * Stands beneath Model and View.
 * Gets perfect data from Model for View.
 * More aware of Views than Models.
 * Mirrors React pattern: Controller orchestrates Model + View.
 */

import type React from 'react';
import type { ViewSpec } from '../../../data/sdsl';
import { MalloyModel } from './malloy-model';
import { MalloyView } from './malloy-view';

/**
 * MalloyController - Orchestrates Model + View
 *
 * Similar to React Controller pattern:
 * - Stands beneath Model and View
 * - More aware of Views than Models
 * - Gets perfect data from Model for View
 * - Passes perfect data to View
 * - Displays JSX
 */
export class MalloyController {
  private malloyView: MalloyView;

  constructor(
    private malloyModel: MalloyModel
  ) {
    this.malloyView = new MalloyView();
  }

  /**
   * Render a view - orchestrates Model + View
   *
   * Flow:
   * 1. Gets perfect data from Model
   * 2. Aware of View, passes perfect data
   * 3. View translates to JSX
   * 4. Controller displays JSX
   */
  async renderView(viewSpec: ViewSpec): Promise<React.ReactNode> {
    // Step 1: Get perfect data from Model
    const result = await this.malloyModel.executeView(viewSpec);

    // Step 2: Aware of View, pass perfect data
    const jsx = await this.malloyView.display("view", "jsx", {
      viewSpec,
      result, // Perfect data for View
    });

    // Step 3: Controller displays JSX
    return jsx;
  }

  /**
   * Render dashboard - orchestrates multiple views
   *
   * EXCLUDED FOR NOW - Stub implementation
   */
  async renderDashboard(
    viewSpecs: ViewSpec[]
  ): Promise<React.ReactNode> {
    // Stub - excluded while focusing on transactional CRUD
    return null;
  }
}

