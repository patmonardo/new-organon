/**
 * @model/data - Data Services
 *
 * Data access services for CRUD operations on entities.
 * These are separate from execution engines.
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

// FactStore - Mock interface (for standalone BI, no Logic dependency)
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
