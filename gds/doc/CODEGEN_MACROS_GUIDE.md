# GDS codegen macros guide (macro_rules)

This repo currently uses **`macro_rules!`** as the “proc-macro substitute” for code generation.
The goal is to keep changes **behavior-preserving** while we progressively reduce noise (warnings), duplication, and sharp edges, until the planned migration to Reality proc-macros.

## Where the macros live

The core system is under:

- `gds/src/projection/codegen/`

Key modules:

- `gds/src/projection/codegen/values/value_type_table.rs`
  - **`value_type_table!`** is the canonical “type table” driver.
  - Many generators call a callback macro once per value type.
- `gds/src/projection/codegen/values/*_macros.rs`
  - `gds_value_scalar!`, `gds_value_binary!`, `gds_value_array_*` build per-type primitives.
- `gds/src/projection/codegen/property/triadic_macros.rs`
  - Generates the universal adapters for **node / relationship / graph** property values.
  - Pattern: `generate_all_*_adapters!()` iterates the table and emits a family of types.
- `gds/src/projection/codegen/property/property_values.rs`
  - Dense macro-heavy impl blocks; changes here can explode warnings due to expansion fanout.
- `gds/src/projection/codegen/algorithm/focused_macros.rs`
  - `define_algorithm_spec!`, and helpers for storage/computation runtime boilerplate.
- `gds/src/projection/codegen/config/define_config.rs`
  - `define_config!` for config schema/validation boilerplate.

## Are the “triadic macros” speculative?

No — they are **already in active use**.

`gds/src/projection/codegen/property/triadic_macros.rs` is the generator behind the concrete default property-value types that the rest of the crate consumes:

- Node property values: `generate_all_node_adapters!()` / `generate_all_node_array_adapters!()` are invoked in `gds/src/types/properties/node/impls/default_node_property_values.rs`.
- Relationship property values: `generate_all_relationship_adapters!()` is invoked in `gds/src/types/properties/relationship/impls/default_relationship_property_values.rs`.
- Graph property values: `generate_all_graph_adapters!()` / `generate_all_graph_array_adapters!()` are invoked in `gds/src/types/properties/graph/impls/default_graph_property_values.rs`.

Those generated types show up throughout the runtime (e.g. default graph store + property stores), so deleting/avoiding these macros would mean reintroducing a lot of hand-written duplication.

### So should we be using them?

Yes, if we want these properties simultaneously:

- **Single source of truth** for supported scalar/array types (the value-type table)
- **Consistent conversion/error policy** across node/relationship/graph properties
- **Pluggable storage backends** (Vec/Huge/Arrow) without duplicating impl blocks

However, “use them” doesn’t mean “they’re finished.” They intentionally skip some categories today (e.g. relationship arrays and some graph scalar categories) because the trait surface isn’t complete everywhere yet.

### What it means to “own” them

In practice, owning these macros means treating them as part of the public kernel API:

- `value_type_table!` defines what exists.
- `triadic_macros.rs` defines what gets generated for each row and what is skipped.
- `property_values.rs` defines the trait-impl semantics and conversion behavior.

If those three stay coherent (and tested), the rest of GDS can evolve quickly without copy/paste explosions.

## Macro mental model (how to read these)

Most codegen in this repo is this pipeline:

1. `value_type_table!{ callback: my_macro }`
2. the table calls `my_macro!(...)` once per value type row
3. the callback expands to one or more `struct`/`impl` blocks

When you’re lost, find:

- the **table definition** (the authoritative list of types)
- the **callback macro** (the “per-row” generator)
- the **public-facing macro** (usually `generate_all_*`) that wires those together

## How to debug expansions (practical)

### Option A: Use `cargo expand`

If you can install it locally:

- `cargo install cargo-expand`

Then you can inspect expansions:

- `cargo expand -p gds projection::codegen::property::property_values`

Tip: focus on a single module at a time; expanding the whole crate is very noisy.

### Option B: Rustc macro backtraces

When something fails deep inside a macro:

- `RUSTFLAGS="-Z macro-backtrace" cargo build -p gds`

This requires nightly Rust.

## Style/conventions used here

### Prefer fully-qualified paths in macro expansions

Inside macro-generated code, prefer `$crate::...` paths.
This:

- makes expansions robust to module moves
- reduces accidental name capture
- makes it easier to remove unused imports in calling modules

### Control warning fanout: fix at the expansion site

A single unused import inside an expansion can produce **dozens** of warnings.
Preferred order:

1. Remove truly-unused imports/locals.
2. If it is intentionally unused for “future implementation”, prefix with `_`.
3. Only then: add **targeted** `#[allow(unused_imports)]` / `#[allow(unused_variables)]`.

Guideline: put `#[allow(...)]` **as close to the offending expansion as possible**.

### Avoid crate-wide `#![allow(...)]`

We generally avoid broad suppressions at the crate root.
They hide real regressions and make later cleanup harder.

## Safe refactor checklist (macro_rules)

When making changes that should not alter behavior:

- Keep emitted public type names stable (downstream code may refer to them).
- Don’t change trait bounds unless necessary.
- Avoid changing evaluation order in generated expressions.
- Prefer moving code into helper macros *without* changing tokens.
- Rebuild with `cargo build -p gds` and compare warning counts.

## “Good next exercises” for learning

1. Add a new value type to `value_type_table!` (even if unused) and trace which generators pick it up.
2. Follow `generate_all_relationship_adapters!()` into its per-type callback and locate the emitted types.
3. Find a repeated pattern in `property_values.rs` and extract a helper macro that takes the minimal token set.
4. Use `cargo expand` to verify the helper macro emits identical code.

## Common pitfalls

- **Shadowed identifiers**: macro parameters named like `node_id`, `graph`, `config` can accidentally shadow locals.
  - Use unique names or force the call site to pass explicit identifiers.
- **`use ... as _;` imports**: they’re useful when a trait must be in scope for method resolution, but they are easy to leave behind.
  - If there is no method call that requires the trait, the import will warn.
- **Accidental `Clone`/allocations** in generated code.
  - If a macro expands into `to_string()`/`clone()` in hot paths, it can become a perf footgun.

---

If you tell me which macro family you want to learn next (values vs adapters vs algorithm specs), I can walk you through one end-to-end: table → generator → emitted types → call sites.
