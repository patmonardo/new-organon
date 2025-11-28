/**
 * @model/data - Data Access Services
 *
 * The Data layer provides service interfaces for CRUD operations
 * on Model entities. These services abstract the underlying
 * persistence (Prisma/Postgres) from the MVC components.
 *
 * Architecture:
 * - Services are stateless, pure functions
 * - Each service corresponds to a Prisma model
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
