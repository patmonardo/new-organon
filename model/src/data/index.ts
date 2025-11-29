/**
 * @model/data - Data Access Services
 *
 * The Data layer provides service interfaces for CRUD operations
 * on Model entities. These services abstract the underlying
 * persistence layer (semantic models, FactStore, etc.) from the MVC components.
 *
 * Architecture:
 * - Services are stateless, pure functions
 * - Each service can be backed by semantic queries, form stores, or mocks
 * - Services can be mocked for testing
 */

// Entity service
export {
  type EntityInput,
  type EntityFilter,
  type EntityService,
  MockEntityService,
  mockEntityService
} from './entity.service';

// Dashboard service
export {
  type DashboardInput,
  type DashboardFilter,
  type StoredDashboard,
  type DashboardService,
  MockDashboardService,
  mockDashboardService
} from './dashboard.service';

// FactStore - Root GDSL interface (mocked for MVC development)
export {
  type Appearance,
  type Fact,
  type Assertion,
  type FactStoreInterface,
  AppearanceSchema,
  FactSchema,
  AssertionSchema,
  MockFactStore,
  getFactStore,
  setFactStore,
  resetFactStore,
} from './fact-store';

// Data SDSL execution stubs
export {
  PolarsExecutionEngine,
  type PolarsDataset,
  type ExecutionOptions,
  type ExecutionResult,
} from './polars-engine';

export {
  SqlEngine,
  type SqlQuery,
} from './sql-engine';

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
