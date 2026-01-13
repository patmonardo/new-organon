// Boundary-schema facade.
// The kernel FormProcessor (Absolute Form) is implemented in Rust under `gds/`.
// This TS module provides the pure-TS “Absolute/Form” rich API over the kernel port.
export * from './gds-link.client';
export * from './kernel-port';
export * from './kernel-trace';
export * from './invariants';
export * from './trace';
