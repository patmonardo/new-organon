// logic package public API.
// This package is primarily a scripting language surface that builds on the
// core @organon/gdsl library (schemas + Dataset primitives). Re-export gdsl
// here for convenience to consumers who import logic.

export * from "./logic"
export * from "./api";
// export * from "./schema"
// export * from "./repository"
// NOTE: Absolute/FormProcessor now lives in the Rust `gds` kernel.
// TS `@organon/logic` is the discursive/relative layer; use `@organon/gdsl/schema`
// for boundary schemas.
// export * from "./absolute/form";
// export * from "./relative"
