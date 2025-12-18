//! Compute surface (substrate): capability traits + minimal in-memory implementation.
//!
//! The substrate is not only “storage”; it also includes computation aspects.
//! This module defines the minimal contract for projecting/allocating compute
//! resources (e.g. suites of GPUs) that can be configured by Projection Factory
//! and consumed by Eval.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Stable identifier for a compute device as seen by the substrate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComputeDeviceId(pub u64);

/// Stable identifier for a compute allocation/lease.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComputeLeaseId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputeDeviceKind {
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputeDevice {
    pub id: ComputeDeviceId,
    pub kind: ComputeDeviceKind,

    /// Optional memory capacity (bytes). Useful for GPU selection.
    pub memory_bytes: Option<u64>,

    /// Free-form labels (e.g. vendor=nvidia, arch=hopper, region=us-east-1).
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputeSuite {
    pub devices: Vec<ComputeDevice>,
}

#[derive(Debug, Clone, Default)]
pub struct ComputeSuiteRequest {
    /// Minimum number of GPUs required.
    pub min_gpus: usize,

    /// Optional minimum memory per GPU.
    pub min_gpu_memory_bytes: Option<u64>,

    /// Required labels.
    pub require_labels: HashMap<String, String>,

    /// If true, the lease is exclusive over its selected devices.
    pub exclusive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputeLease {
    pub id: ComputeLeaseId,
    pub suite: ComputeSuite,
}

#[derive(Debug, thiserror::Error)]
pub enum ComputeSurfaceError {
    #[error("insufficient compute resources")]
    Insufficient,

    #[error("unsupported compute request: {0}")]
    Unsupported(String),

    #[error("internal compute surface error: {0}")]
    Internal(String),
}

/// Substrate boundary for computation aspects.
///
/// This is intentionally minimal: it is a seam for future schedulers/placement
/// without contaminating evaluators with orchestration concerns.
pub trait ComputeSurface: Send + Sync {
    fn list_devices(&self) -> Vec<ComputeDevice>;
    fn allocate_suite(
        &self,
        request: ComputeSuiteRequest,
    ) -> Result<ComputeLease, ComputeSurfaceError>;
    fn release_suite(&self, lease_id: ComputeLeaseId) -> Result<(), ComputeSurfaceError>;
}

#[derive(Debug, Default)]
struct InMemoryComputeState {
    devices: Vec<ComputeDevice>,
    leased: HashSet<ComputeDeviceId>,
    leases: HashMap<ComputeLeaseId, Vec<ComputeDeviceId>>,
}

/// Simple in-memory compute surface.
///
/// Useful for tests and for “single-process kernel” demos.
#[derive(Debug, Clone, Default)]
pub struct InMemoryComputeSurface {
    state: Arc<Mutex<InMemoryComputeState>>,
    lease_seq: Arc<AtomicU64>,
}

impl InMemoryComputeSurface {
    pub fn with_devices(devices: Vec<ComputeDevice>) -> Self {
        Self {
            state: Arc::new(Mutex::new(InMemoryComputeState {
                devices,
                leased: HashSet::new(),
                leases: HashMap::new(),
            })),
            lease_seq: Arc::new(AtomicU64::new(1)),
        }
    }
}

impl ComputeSurface for InMemoryComputeSurface {
    fn list_devices(&self) -> Vec<ComputeDevice> {
        let guard = self
            .state
            .lock()
            .map_err(|_| ())
            .expect("compute surface mutex poisoned");
        guard.devices.clone()
    }

    fn allocate_suite(
        &self,
        request: ComputeSuiteRequest,
    ) -> Result<ComputeLease, ComputeSurfaceError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| ())
            .expect("compute surface mutex poisoned");

        let mut selected: Vec<ComputeDevice> = Vec::new();
        let mut selected_ids: Vec<ComputeDeviceId> = Vec::new();

        let gpu_candidates = guard
            .devices
            .iter()
            .filter(|d| d.kind == ComputeDeviceKind::Gpu)
            .filter(|d| !request.exclusive || !guard.leased.contains(&d.id))
            .filter(|d| {
                if let Some(min_mem) = request.min_gpu_memory_bytes {
                    match d.memory_bytes {
                        Some(mem) => mem >= min_mem,
                        None => false,
                    }
                } else {
                    true
                }
            })
            .filter(|d| {
                request
                    .require_labels
                    .iter()
                    .all(|(k, v)| d.labels.get(k) == Some(v))
            })
            .take(request.min_gpus);

        for device in gpu_candidates {
            selected.push(device.clone());
            selected_ids.push(device.id);
        }

        if selected.len() < request.min_gpus {
            return Err(ComputeSurfaceError::Insufficient);
        }

        if request.exclusive {
            for id in &selected_ids {
                guard.leased.insert(*id);
            }
        }

        let lease_id = ComputeLeaseId(self.lease_seq.fetch_add(1, Ordering::Relaxed));
        guard.leases.insert(lease_id, selected_ids);

        Ok(ComputeLease {
            id: lease_id,
            suite: ComputeSuite { devices: selected },
        })
    }

    fn release_suite(&self, lease_id: ComputeLeaseId) -> Result<(), ComputeSurfaceError> {
        let mut guard = self
            .state
            .lock()
            .map_err(|_| ())
            .expect("compute surface mutex poisoned");

        let Some(device_ids) = guard.leases.remove(&lease_id) else {
            return Err(ComputeSurfaceError::Unsupported(format!(
                "unknown lease {lease_id:?}"
            )));
        };

        for device_id in device_ids {
            guard.leased.remove(&device_id);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gpu(id: u64, memory_bytes: u64, vendor: &str) -> ComputeDevice {
        ComputeDevice {
            id: ComputeDeviceId(id),
            kind: ComputeDeviceKind::Gpu,
            memory_bytes: Some(memory_bytes),
            labels: HashMap::from([(String::from("vendor"), vendor.to_string())]),
        }
    }

    #[test]
    fn allocates_exclusive_gpu_suite() {
        let surface = InMemoryComputeSurface::with_devices(vec![
            gpu(1, 80_000, "nvidia"),
            gpu(2, 80_000, "nvidia"),
        ]);

        let lease = surface
            .allocate_suite(ComputeSuiteRequest {
                min_gpus: 2,
                exclusive: true,
                ..Default::default()
            })
            .expect("should allocate");

        assert_eq!(lease.suite.devices.len(), 2);

        let err = surface
            .allocate_suite(ComputeSuiteRequest {
                min_gpus: 1,
                exclusive: true,
                ..Default::default()
            })
            .expect_err("should be insufficient due to exclusivity");

        matches!(err, ComputeSurfaceError::Insufficient);

        surface.release_suite(lease.id).expect("should release");
        let _lease2 = surface
            .allocate_suite(ComputeSuiteRequest {
                min_gpus: 1,
                exclusive: true,
                ..Default::default()
            })
            .expect("should allocate after release");
    }
}
