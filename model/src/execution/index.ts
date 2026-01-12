/**
 * @model/execution - Execution Engines
 *
 * Execution engines for running queries against data models.
 * Separated from data services for clean architecture.
 */

// Polars execution engine
export {
  PolarsExecutionEngine,
  type PolarsDataset,
  type ExecutionOptions,
  type ExecutionResult,
} from './polars-engine';

// SQL execution engine
export { SqlEngine, type SqlQuery } from './sql-engine';

// Semantic hydrator (bridges execution results to forms)
export {
  SemanticHydrator,
  type SemanticDataService,
  type SemanticResult,
  type HydratorContext,
  type HydratorSpec,
  type HydratorSnapshot,
  type RowLike,
  type FormBinding,
  type CollectionBinding,
  type MetricBinding,
} from './semantic-hydrator';
