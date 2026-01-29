# Collections IO Foundations (PyG + torch-frame)

This note summarizes IO foundations we can safely borrow (conceptually) from PyG and torch-frame without copying code. It is intended to keep Collections IO aligned with proven patterns while we iterate on Polars/Arrow.

## PyG fsspec layer (torch_geometric.io.fs)

**High-level idea**: All IO operations route through a small filesystem adapter that chooses a backend from the URI scheme and then exposes a consistent set of helpers (exists, ls, cp, rm, etc.). The adapter also handles local vs remote path behavior and caching for remote downloads.

Key behaviors (paraphrased):

- **Backend dispatch**: Determine filesystem from a path URI (e.g., local, memory, HTTP, S3, GCS). The URI scheme determines the backend.
- **Local vs remote**: Treat `file` or `memory` as local; for remote paths, normalize listing results by unstripping protocols.
- **Path normalization**: Normalize only disk paths; keep remote URIs intact.
- **Core FS helpers**: `exists`, `makedirs`, `isdir`, `isfile`, `ls`, `rm`, `mv`, `glob`.
- **Copy + extraction**:
  - Copy supports both single files and directories.
  - Handles glob patterns for directories.
  - Optional auto‑extract for `.tar.gz`, `.zip`, `.gz`.
  - Optional caching for remote sources (simplecache) with temp cleanup.
- **Torch serialization**: `torch_save`/`torch_load` write to arbitrary FS (via fsspec) and handle weight‑only loading fallbacks.

Relevant sources:

- [gds/pytorch_geometric/torch_geometric/io/fs.py](../pytorch_geometric/torch_geometric/io/fs.py)
- [gds/pytorch_geometric/torch_geometric/io/txt_array.py](../pytorch_geometric/torch_geometric/io/txt_array.py)

## PyG dataset IO conventions

**High-level idea**: Datasets implement a consistent “raw → processed” pipeline under a `root` directory, and use the IO adapter for storage.

Key behaviors (paraphrased):

- **Dataset root layout**: Raw and processed assets live under a dataset root; processed artifacts are saved/loaded by dataset classes.
- **Download helpers**: `download_url` streams from network to a target folder using fsspec for writes (supports remote destinations).
- **In‑memory dataset storage**: `InMemoryDataset.save/load` uses `torch_save/torch_load` to store a tuple of data, slices, and class metadata.
- **On‑disk dataset option**: In‑memory datasets can be converted to an on‑disk variant with a selected backend.

Relevant sources:

- [gds/pytorch_geometric/torch_geometric/data/download.py](../pytorch_geometric/torch_geometric/data/download.py)
- [gds/pytorch_geometric/torch_geometric/data/in_memory_dataset.py](../pytorch_geometric/torch_geometric/data/in_memory_dataset.py)

## torch-frame IO foundations

**High-level idea**: torch‑frame focuses on tabular datasets with explicit semantic types and a materialization step. Downloading is simpler (direct URL to file) but the typed metadata concepts are useful for Collections.

Key behaviors (paraphrased):

- **Download helper**: Direct URL download into a root directory; no fsspec abstraction.
- **Materialization**: A dataset goes through a “materialize” step that maps columns to semantic types and computes stats.
- **Typed column metadata**: Column‑to‑stype mapping and per‑column stats are core to data handling.

Relevant sources:

- [gds/pytorch-frame/torch_frame/data/download.py](../pytorch-frame/torch_frame/data/download.py)
- [gds/pytorch-frame/torch_frame/data/dataset.py](../pytorch-frame/torch_frame/data/dataset.py)

## Collections IO takeaways (foundation only)

**Recommended minimal surface** (conceptual parity, not code parity):

1. **URI‑based filesystem resolver** (local, memory, HTTP, S3/GCS later).
2. **Uniform fs helpers**: `exists`, `makedirs`, `isdir`, `isfile`, `ls`, `rm`, `mv`, `glob`.
3. **Copy + optional extract**: Support file/dir copies, wildcard expansion, and archive extraction.
4. **Remote caching**: Optional cache for remote downloads (ephemeral, opt‑in).
5. **Serialization helpers**: For dataset manifests or binary payloads (analogous to `torch_save/torch_load`).
6. **Dataset layout**: Keep a clean `raw/processed` or `collections/<name>` layout with a manifest and IO policy.
7. **Typed metadata**: Preserve value types and schema metadata alongside the data artifacts.

## What we should avoid (for now)

- Premature graph‑specific structure in Collections IO (keep it generic).
- Large dependency surface for nonessential FS backends.
- Complex on‑disk graph layouts before Collections IO is stable.

## Next step

- Integrate a lightweight fs abstraction into Collections catalog IO (URI resolver + copy/extract + caching), then map IO formats to Polars/Arrow readers.
