# Pathfinding Progress Contract

Purpose: document the intended progress-tracker contract for pathfinding procedure builders so application machinery and procedures interact consistently and safely.

Intent
- Builders MAY accept optional `task_registry_factory` and `user_log_registry_factory` dependencies so callers (machinery or tests) can provide request-scoped registries. Builders MUST fall back to `EmptyTaskRegistryFactory` when none are provided so standalone usage continues to work.
- Procedures may create short-lived, per-run trackers for internal sub-runs (for example, per-target runs in multi-target builders). This avoids attempting to restart a Finished leaf task.

Rules for builders and procedures
- Expose setters for registry factories when applicable (`task_registry_factory(...)`, `user_log_registry_factory(...)`).
- Use the injected factories to construct trackers when available. Do not accept factories and then ignore them; prefer a single helper to centralize fallback logic.
- For multi-target or multi-run loops, create a fresh `TaskProgressTracker` per run/target. Do not reuse a tracker that may have finished.
- Respect machinery ownership of lifecycle where applicable: when execution is driven from the Machinery envelope (i.e. `AlgorithmProcessingTemplate` / `ProgressTrackerCreator`), the machinery owns begin/end/release/failure semantics. Builders should only use trackers for domain progress reporting (volume/steps) and not call lifecycle methods on behalf of the outer envelope.

Recommended helper
- Provide a single helper in `crate::core::utils::progress` (suggested name: `get_progress_tracker_for_run`) with signature similar to:
  - `fn get_progress_tracker_for_run(task: Tasks, concurrency: usize, task_registry_factory: Option<Box<dyn TaskRegistryFactory>>, user_log_registry_factory: Option<Box<dyn TaskRegistryFactory>>) -> TaskProgressTracker`.
- Helper behavior:
  - If a `task_registry_factory` is present, use it (and the provided job id / request context when available) to construct a registry-backed tracker.
  - Otherwise, fall back to `TaskProgressTracker::with_concurrency(...)` or `TaskProgressTracker::new(...)` using `EmptyTaskRegistryFactory` so existing behavior is preserved.
  - Document that the helper returns a fresh tracker suitable for a single run or a single target-run.

Memory estimation
- Standardize the memory estimate surface across pathfinding builders as:
  - `pub fn estimate_memory(&self) -> MemoryRange`
  - Use `Result<MemoryRange>` only when an estimator can genuinely fail; prefer conservative estimates returned directly otherwise.

Migration recommendations (small incremental)
1. Add the helper to `crate::core::utils::progress` and document its contract.
2. Migrate 2–4 high-priority builders to call the helper (suggested: `astar`, `bfs`, `dijkstra`, `bellman_ford`).
3. Add a small unit test that injects a mock `TaskRegistryFactory` and verifies the factory path is exercised.
4. Batch-migrate the remaining builders.

Examples & references
- Per-target rationale example: see `gds/src/procedures/pathfinding/astar.rs` where A* creates a fresh tracker per target run and documents why (avoid re-starting Finished tasks).
- Machinery creator: see `gds/src/applications/algorithms/machinery/progress_tracker_creator.rs` for how request-scoped trackers are created and owned by Machinery.
- Builder setters: see `gds/src/procedures/pathfinding/dijkstra.rs` (setter examples) and many other builders for the current property names.

Notes
- This contract preserves backwards compatibility: builders without injected factories continue to work as before.
- The helper centralizes the branching logic and documents expected behavior so future builders follow the same pattern.
