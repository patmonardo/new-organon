//! Compute-suite projection (Factory → Eval).
//!
//! This is the “Projection Factory can project suites of GPUs” seam.
//!
//! It deliberately avoids choosing *how* compute is scheduled. It only defines
//! the kernel-facing contract for selecting/allocating a suite of compute devices
//! from the substrate.

use crate::substrate::{ComputeLease, ComputeSuiteRequest, ComputeSurface, ComputeSurfaceError};

#[derive(Debug, Clone, Default)]
pub struct ComputeSuiteProjectionConfig {
    pub request: ComputeSuiteRequest,
}

#[derive(Debug, thiserror::Error)]
pub enum ComputeSuiteProjectionError {
    #[error(transparent)]
    Surface(#[from] ComputeSurfaceError),
}

/// Minimal projector: given a substrate compute surface, allocate a compute suite.
///
/// This is intentionally small; higher layers can attach policy, tenancy, quotas,
/// and trace/audit at the eval boundary.
pub trait ComputeSuiteProjector: Send + Sync {
    fn project_suite(
        &self,
        surface: &dyn ComputeSurface,
        config: &ComputeSuiteProjectionConfig,
    ) -> Result<ComputeLease, ComputeSuiteProjectionError>;
}

#[derive(Debug, Clone, Default)]
pub struct DefaultComputeSuiteProjector;

impl ComputeSuiteProjector for DefaultComputeSuiteProjector {
    fn project_suite(
        &self,
        surface: &dyn ComputeSurface,
        config: &ComputeSuiteProjectionConfig,
    ) -> Result<ComputeLease, ComputeSuiteProjectionError> {
        Ok(surface.allocate_suite(config.request.clone())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::substrate::{
        ComputeDevice, ComputeDeviceId, ComputeDeviceKind, InMemoryComputeSurface,
    };
    use std::collections::HashMap;

    #[test]
    fn factory_can_project_a_gpu_suite() {
        let surface = InMemoryComputeSurface::with_devices(vec![
            ComputeDevice {
                id: ComputeDeviceId(1),
                kind: ComputeDeviceKind::Gpu,
                memory_bytes: Some(80_000),
                labels: HashMap::new(),
            },
            ComputeDevice {
                id: ComputeDeviceId(2),
                kind: ComputeDeviceKind::Gpu,
                memory_bytes: Some(80_000),
                labels: HashMap::new(),
            },
        ]);

        let projector = DefaultComputeSuiteProjector;
        let lease = projector
            .project_suite(
                &surface,
                &ComputeSuiteProjectionConfig {
                    request: ComputeSuiteRequest {
                        min_gpus: 2,
                        exclusive: true,
                        ..Default::default()
                    },
                },
            )
            .expect("should project suite");

        assert_eq!(lease.suite.devices.len(), 2);
    }
}
