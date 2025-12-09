/**
 * @organon/model - Standalone Malloy BI Package
 *
 * A standalone Malloy-inspired BI package with zero dependencies on
 * GDS/GDSL/Logic/Task. Focuses on Model-View-Dashboard for BI.
 *
 * Structure:
 * - sdsl/ - SDSL Core (Species DSL) - isolated core
 * - execution/ - Execution engines (Polars, SQL, Hydrator)
 * - data/ - Data services (Entity, Dashboard, FactStore)
 * - schema/ - Zod schemas
 * - ui/react/sdsl/ - React adapters/controllers
 * - ui/radix/sdsl/ - Radix adapters/controllers
 * - ui/malloy/ - Malloy UI (speculative bubble)
 *
 * This is a standalone package - use it independently.
 */

export * from './model';

// SDSL Core - Malloy-inspired semantic modeling (isolated)
export * from './sdsl';

// Execution engines
export * from './execution';

// Data services
export * from './data';

// Schema layer
export * as schema from './schema';

// UI adapters (optional - import from ui/react/sdsl or ui/radix/sdsl directly)
export * from './ui/react/sdsl';
export * from './ui/radix/sdsl';
