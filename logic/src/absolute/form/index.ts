// Boundary-schema facade.
// The kernel FormProcessor (Absolute Form) is implemented in Rust under `gds/`.
// This TS module re-exports the GDSL boundary schema surface AND provides the
// pure-TS “Absolute/Form” rich API over GDS Link (GDS-L).
export * from '@organon/gdsl/schema';
export * from './gds-link.client';
