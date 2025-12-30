# GDS-L (GDS Link) vs G-DSL (Generic/Global DSL)

This repo uses two closely-related (but distinct) ideas under “GDSL”:

## GDS-L: GDS Link (client → server protocol)

**GDS-L** is the *protocol layer* for invoking the Rust GDS kernel from TypeScript.

- **Payload (application call)**: `GdsApplicationCall`
  - Defined in `gdsl/src/schema/application.ts`
  - Routed by `{ facade, op, ... }`
  - Has a stable operation id: `gds.<facade>.<op>`
- **Transport (execution boundary)**: `KernelPort`
  - Generic “run a model with an input” interface: `gdsl/src/kernel-api.ts`
  - The concrete TS-JSON transport adapter is:
    - `GdsTsjsonKernelPort` (alias: `GdsLinkTsjsonKernelPort`)
    - `gdsl/src/sdk/gds-tsjson-kernel-port.ts`

On the Rust side, the TS-JSON entrypoint is the NAPI `invoke(json)` boundary (see
`gds/src/applications/services/tsjson_napi.rs`) and is documented in
`gds/doc/TSJSON_NAPI_FACADE_V1.md`.

## G-DSL: Generic / Global DSL (client-facing DSL space)

**G-DSL** is the *client-facing DSL space* that can emit GDS-L payloads.

Intuition:

- G-DSL provides a “generic” vocabulary (ex: FormDB concepts, generic programs).
- Specific **S-DSLs** are specializations that compile down into **GDS-L payloads**
  (i.e. `GdsApplicationCall`) which are then executed via a `KernelPort`.

In short:

\[
\text{G-DSL (payload design)} \Rightarrow \text{GDS-L (application call)} \Rightarrow \text{KernelPort (transport)} \Rightarrow \text{GDS kernel}
\]

## FormDB and GraphStore (storage substrate)

In this repo’s vocabulary:

- **FormDB**: a client-facing KnowledgeGraph (PropertyGraph).
- **GraphStore**: a GraphStore that embodies FormDB + FactStores.

At the boundary, this shows up as “handle-first” responses: the kernel persists
results into a GraphStore and returns **graph references** (e.g. `graphName`)
plus proof/witness metadata, rather than streaming massive payloads directly.

