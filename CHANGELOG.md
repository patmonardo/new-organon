# Changelog

All notable changes to this repository will be documented in this file.

## 2026-01-24 — Configuration harmonization

- **DecisionTree / RandomForest**: Harmonized `max_depth` semantics so that a
  value of `0` means "unlimited" (previously DecisionTree used `usize::MAX`).
  Updated validation, runtime checks, and tests across `ml/decision_tree` and
  `ml/models/random_forest` to reflect this convention. Improved docstrings
  and comments to call out the behavior explicitly.
