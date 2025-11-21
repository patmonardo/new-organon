# Facets

What
- Facets are non-authoritative metadata attached to any Form doc.
- Shape-level field: `shape.facets: Record<string, unknown>`.

Why
- Annotations, UI hints, cached projections, derived flags, indexes.
- Not part of core invariants; safe to drop/rebuild.

How (Engine verbs)
- setFacets(id, facets): replace with the given object.
- mergeFacets(id, patch): shallow-merge keys; does not deep-merge nested objects.

Relation to other fields
- core: identity/type/name — authoritative, schema-validated.
- state: operational defaults (`status|tags|meta`), validated, versioned.
- signature: capabilities/claims; may be merged/cleared explicitly.
- facets: free-form, not used for invariants or concurrency; persistable hints.

Conventions
- Use namespaces: `ui.*`, `calc.*`, `cache.*`, `index.*`.
- Do not store critical domain rules here.
- Prefer small, serializable values; version if needed (e.g., `cache.v=1`).

Testing notes
- Engines emit `*.facets.set` and `*.facets.merged`.
- Current `describe` payload may not surface `facetsKeys`; assert events or fetch doc and inspect `shape.facets`.
