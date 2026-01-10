# GNN IO & Dataset Design (Rust-native sketch)

Goal
----
- Provide a PyG-inspired IO/loader stack implemented in Rust for `gds/`:
  - `Data` / `HeteroData` in-memory containers
  - `InMemoryDataset` and `OnDiskDataset` lifecycle (download/process/save/load)
  - Filesystem abstraction supporting local, s3, gcs, http
  - Format readers (OFF/PLY/OBJ/NPZ/TU) and serialization helpers
  - `collate`/`slices` representation for stacked storage
  - Transform hooks: `pre_transform` (one-time) and `transform` (on access)

Design principles
-----------------
- Keep data model minimal and explicit (typed arrays for features, separate index arrays for edges).
- Prefer Arrow/ObjectStore crates for portable, zero-copy IO when possible.
- Keep PyG/PyTorch isolated: export import adapters but do core compute in Rust.
- Expose a small, stable procedure API for upper layers to call.

Suggested crates and tech
-------------------------
- Data layout and arrays: `ndarray`, `arrow2` for columnar interchange.
- Filesystem/object store: `object_store` (arrow-rs) for local/S3/GCS/http backends.
- Serialization: `parquet` or Arrow IPC for big arrays; `bincode`/`serde` for small metadata.
- On-disk DB/index: `rocksdb` or `sled` for fast key-value storage of serialized examples.
- NPZ/Numpy: `ndarray-npy` + `zip` crates to read `.npz` files.

Core types (sketch)
-------------------
```rust
pub struct Tensor { // small wrapper
    pub shape: Vec<usize>,
    pub dtype: DType,
    pub data: Vec<u8>, // contiguous buffer (f32, f64, i64, ...)
}

pub struct Data { // homogeneous graph
    pub x: Option<Tensor>,
    pub edge_index: Option<Tensor>, // shape [2, E], u32/u64
    pub edge_attr: Option<Tensor>,
    pub y: Option<Tensor>,
    pub pos: Option<Tensor>,
    pub metadata: HashMap<String, String>,
}

pub trait Transform: Send + Sync {
    fn apply(&self, data: &mut Data);
}

pub struct InMemoryDataset { // holds collated `data` + `slices`
    pub data: Data, // stacked representation
    pub slices: HashMap<String, Vec<usize>>,
}
```

Collate / slices
-----------------
- Mirror PyG approach: collate a Vec<Data> into a single `Data` where every tensor is concatenated along the appropriate cat-dim and a `slices` dictionary records per-example offsets. This enables efficient get(i) via slicing (no deep copies).
- Use Arrow/Parquet for on-disk column storage for large arrays.

Filesystem & remote IO
----------------------
- Use `object_store` abstraction to read/write remote/local objects.
- Provide utilities similar to PyG`s `fs`:
  - `exists(path)`, `read_bytes(path)`, `write_bytes(path)`, `glob(path)`.

On-disk dataset
---------------
- Implement `OnDiskDataset` backed by RocksDB/SQLite/Parquet depending on needs:
  - Key: example_id or sequence number
  - Value: serialized `Data` (Arrow IPC or bincode + buffers)
- Provide `to_on_disk_dataset(root, backend)` conversion helper.

Readers & format support
------------------------
- Implement format readers under `gds/src/io/readers/*`:
  - `read_off`, `read_obj`, `read_ply` (mesh -> `pos`, `face`)
  - `read_npz` (graph arrays saved as numpy)
  - `read_tu` (TU datasets)
- Each reader returns `Data` and is small/testable.

Transforms and preprocessing
---------------------------
- Implement `pre_transform` hook executed once during `process()` (e.g., feature normalization, knn graph construction).
- Implement `transform` applied lazily on `get()`.

Interop with PyG / import options
---------------------------------
- Export/import helpers:
  - Python-side helper to convert PyG `Data`/`InMemoryDataset` -> Arrow/Parquet/NPZ compatible files for Rust ingestion.
  - Rust utilities to read that exported representation.
- Consider an intermediate canonical on-disk format (Arrow IPC or Parquet) for cross-language portability.

Batching & samplers
--------------------
- Implement `NeighborSampler` and `DataLoader` semantics in Rust later; in short-term provide `batch_from_indices` utility that uses `slices` to form `Batch` objects.

Module layout (proposal)
------------------------
- `gds/src/io/mod.rs` (fs, readers, utils)
- `gds/src/dataset/mod.rs` (InMemoryDataset, OnDiskDataset, collate)
- `gds/src/data/mod.rs` (Data, Tensor wrapper, serialization)
- `gds/src/transforms/mod.rs` (Transform trait + utilities)
- `gds/examples/` (readers/tests, small conversion scripts)

Minimal next milestones (2-week plan)
------------------------------------
1. Implement `Data` + `Tensor` and `collate` (1-2 days).
2. Implement `fs` wrapper using `object_store` and basic read/write (2 days).
3. Implement `InMemoryDataset` load/save and `to_on_disk_dataset` (3 days).
4. Implement NPZ and OFF readers and tests (3 days).
5. Add small Python export helper and example converting a PyG `InMemoryDataset` to Arrow/NPZ for Rust ingestion (3 days).

Notes
-----
- Choosing Arrow/Parquet early yields stronger cross-language portability and faster bulk IO.
- If GPU kernels are later required, keep buffer layout compatible with `wgpu`/CUDA bridging crates and with `tch-rs` if optional torch interop is needed.

---

Created as an initial sketch to implement PyG-like IO and loaders in Rust inside `gds/`.
