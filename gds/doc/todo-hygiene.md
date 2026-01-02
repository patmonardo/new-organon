# GDS task-marker hygiene

- Format: `Note(owner,YYYY-MM-DD): message` (owner = handle/team; date = target or reevaluation).
- Scope: `gds/src/**` and runtime code. Docs (`gds/doc/**`) and Markdown are excluded from the check.
- Script: `tools/check-gds-todos.sh` (defaults to changed GDS files; pass paths to override). Fails on bare task markers.
- Rationale: keeps placeholders actionable and time-bound; pair TODOs with linked issue IDs in the message when possible.
- Long-lived stubs: prefer feature flags or `#[allow(dead_code)] // reason` with the same owner/date pattern nearby.
