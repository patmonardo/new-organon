// logic package public API.
// This package is the TS discursive/relative layer that authors kernel calls.

export * from './logic';
export * from './absolute';
export * from './repository';
// Expose schema and relative layers under namespaces to avoid name collisions (e.g., FormShape class vs schema record)
export * as schema from './schema';
export * as relative from './relative';
