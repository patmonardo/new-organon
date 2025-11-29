/**
 * Customer Example - Semantic MVC with Hydration
 *
 * This is the primary customer example demonstrating:
 * - Semantic Data Modeling (SDSL)
 * - SemanticHydrator for bridging data to forms
 * - ReactController and ReactView
 * - Radix adapter for UI rendering
 * - Polars execution engine
 */

// Domain types and schemas
export * from './customer';

// Semantic data model
export * from './customer-model';
export * from './invoice-model';

// Data service
export * from './customer-data.service';

// MVC components
export * from './customer-controller';
export * from './customer-view';

// Runtime demo
export { default as runDemo } from './runtime';

