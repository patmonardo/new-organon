/**
 * @organon/model - The First SDSL (Special Data Science Language)
 *
 * The MVC SDSL is a Client of the @logic Form Processor.
 * The FactStore is the First Speaker - this SDSL receives and transforms.
 *
 * Exports:
 * - sdsl/*: MVC SDSL core (FormModel, FormView, FormController, Adapters)
 * - schema/*: Data Model schemas (Dashboard, Table, etc.)
 * - data/*: Data services (Entity, Dashboard)
 */

export * from './model';

// MVC SDSL - Core language (the new foundation)
export * from './sdsl';

// Legacy schema layer (to be migrated)
// These have naming conflicts with SDSL, so import them namespaced
export * as schema from './schema';

// Data services
export * from './data';

// Examples - MVC SDSL Design Templates
// export * as examples from './examples';
