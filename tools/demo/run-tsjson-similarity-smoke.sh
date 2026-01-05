#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
REQ_JSON="$ROOT_DIR/tools/fixtures/tsjson/similarity-smoke.json"

if [[ ! -f "$REQ_JSON" ]]; then
  echo "Missing request JSON: $REQ_JSON" >&2
  exit 1
fi

cd "$ROOT_DIR"

# Runs a batch request against the in-process TSJSON catalog.
# This is meant to be a stable, low-friction regression check for:
# - similarity apps wiring (machinery envelopes)
# - procedure facade stream/stats/estimate plumbing
#
# Note: For KNN on Double node properties, `similarityMetric: EUCLIDEAN` is not supported.

cargo run -p gds --bin tsjson_cli --quiet < "$REQ_JSON"
