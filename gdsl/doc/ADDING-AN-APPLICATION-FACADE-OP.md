# Adding a GDS Application Facade op (end-to-end)

This repo treats “calling the kernel” as a stable protocol pipeline:

1. **GDS-L payload** (`GdsApplicationCall`) in TypeScript
2. **Transport** (`KernelPort`) via TS-JSON/NAPI
3. **Rust facade handler** routes by `{ facade, op }` and returns a TS-JSON envelope

## 1) Define / extend the call shape (TypeScript)

Add a new op to the relevant facade schema in:

- `gdsl/src/schema/gds.application.ts`
- `gdsl/src/schema/application.ts`
- (or the specific facade file; GraphStore facade schema has been removed from GDSL)

The call must include:

- `facade`: the routing namespace (string literal)
- `op`: the operation name (string literal)
- `user`, `databaseId`: shared context fields

## 2) Implement the Rust handler (TSJSON)

Edit:

- `gds/src/applications/services/tsjson_napi.rs`

Add a new `match op { ... }` branch inside the appropriate handler:

- `handle_graph_store_catalog`
- `handle_graph_store`
- `handle_form_eval`

Return:

- `ok(op, json!({ ... }))` on success
- `err(op, CODE, MESSAGE)` on failure

## 3) Invoke from app code (TypeScript)

Use the canonical invoker helper:

- `invokeGdsApplicationCall(port, call)` from `gdsl/src/sdk/gds-link.ts`

This takes a typed `GdsApplicationCall` and calls through whatever `KernelPort`
you provide (TS-JSON/NAPI, mock, remote adapter).


