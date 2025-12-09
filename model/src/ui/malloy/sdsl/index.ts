/**
 * Malloy SDSL Adapters and Controllers
 *
 * Malloy-specific adapters, controllers, and views for SDSL.
 * Mirrors React pattern: Model → Controller → View
 */

// Model (Data Provider)
export { MalloyModel } from './malloy-model';

// Controller (Orchestrator)
export { MalloyController } from './malloy-controller';

// View (Display Translator)
export { MalloyView } from './malloy-view';
export type { MalloyViewOptions } from './malloy-view';

