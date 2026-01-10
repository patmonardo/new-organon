# TypeScript / Rust ↔ Python Adapter Strategy (concise)

Goal

- Provide a small, auditable adapter surface to run PyG/PyTorch workloads without making the core runtime depend on PyTorch.
- Capture lessons from Torch JIT / TorchScript to support model artifact interchange when possible.

Design principles

- Procedure-first: TS/TS-procedures call Rust procedures; Rust may call an adapter (service/FFI) that runs Python when needed.
- Isolation: run Python-powered workloads in an isolated process/container to avoid polluting the Rust runtime.
- Interchange-first: move tensors/metadata via framework-agnostic formats (Arrow IPC / Parquet / NPZ) and use object-store URIs for bulk payloads.
- Reproducibility: pin Python and Torch versions for any persisted model/artifact; record runtime metadata in manifests.

Adapter Modes

1. Black-box Python service (recommended for research and complex PyG flows)

- Deploy a small long-lived service (gRPC/HTTP) that receives control RPCs and object-store URIs for data. It runs PyG, Torch training, explainers, and exports artifacts.
- Advantages: minimal coupling, independent lifecycle, easy to scale to GPU nodes.
- Data plane: Arrow IPC or Parquet for tabular/columnar tensors; NPZ for small, single-file payloads; use presigned S3/GCS URIs for large payloads.

2. Managed subprocess pool (lightweight, hermetic)

- Launch Python worker subprocesses from Rust with controlled virtualenvs. Communicate via stdin/stdout or unix sockets and use Arrow IPC files for data exchange.
- Advantages: simpler deployment, lower latency than network service, easier to debug locally.

3. Embedded FFI (PyO3 / cpython) — avoid unless necessary

- Embedding Python into Rust is powerful but increases surface area and risk. Use only for tiny utilities where process isolation is unacceptable.

Model artifact strategy (Torch JIT learnings)

- Preferred export formats:
  - ONNX: best when model ops map to ONNX and you need cross-framework portability.
  - TorchScript (.pt via torch.jit.trace/script): when ONNX is insufficient; requires LibTorch for runtime.
  - Saved Python artifacts: include training code + environment if reproducible re-training is required.
- Custom ops:
  - TorchScript works with custom ops but requires registering plugins in LibTorch at runtime — plan for plugin build and ABI compatibility.
  - Prefer avoiding custom ops for artifacts intended to be portable; if unavoidable, capture build recipe and versioned plugin binaries in the artifact manifest.

API sketch (control RPCs)

- CreateProjection(manifest) -> job_id
- RunTransform(job_id, dataset_uri) -> transfer manifest / storage URI
- TrainModel(job_id, training_spec) -> model_artifact_uri
- ExportModel(job_id, format=[onnx|torchscript]) -> artifact_uri
- ExplainModel(job_id, input_uri) -> explanation_uri

Practical recommendations

- Default to the Black-box Python service for research/experimentation.
- Use TorchScript/ONNX exports for inference paths that must be consumed by non-Python runtimes; keep test vectors to validate exports.
- Always record: torch version, cuda/cuDNN, Python env, package hashes, and operator/onnx-compatibility notes in artifact metadata.
- Provide simple conversion helpers in `gds/tools/` (e.g., `export_to_onnx.py`, `script_model.py`) and CI checks that verify export integrity.

Security and reproducibility

- Run untrusted Python workloads in constrained containers. Prefer read-only mounts where possible.
- Sign or checksum model artifacts; provide an allowlist for custom ops and prebuilt plugin binaries.

Next actionable steps

- Draft a gRPC proto for the black-box service (control + streaming TransferChunk) and a minimal Python service skeleton.
- Implement a small `gds/tools/export_model.py` helper to produce TorchScript and ONNX artifacts and record metadata.
