/**
 * @model/schema - SDSL Data Model Schemas
 *
 * The Schema layer defines the Special Data Science Language (SDSL)
 * for data models - the first SDSL in our architecture.
 *
 * These schemas are:
 * - Projectable from @logic/FactStore forms
 * - Renderable via React/Next components
 * - Persistable via FactStore or any backend adapter
 */

// Core MVC schemas
export * from './model';
export * from './view';
export * from './controller';

// Data Model shapes
export * from './shape';
// export * from './dashboard';
export * from './card';
export * from './table';
// export * from './chart';  // needs visualization module
export * from './presentation';
export * from './messaging';
export * from './document';
export * from './list';
export * from './link';
export * from './text';
export * from './button';
export * from './font';
export * from './image';
export * from './radix';
