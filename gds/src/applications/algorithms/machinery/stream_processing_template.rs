//! Stream Processing Template - minimal Java-parity template for stream mode.
//!
//! This implements the "template method pattern" for algorithm execution:
//! 1. Load graph from catalog
//! 2. (Optional) memory guard check
//! 3. Create progress tracker
//! 4. Run algorithm computation
//! 5. Render result
//!
//! For stream mode, there's no side-effect step (mutate/write).

use crate::applications::algorithms::machinery::{
    DefaultProgressTrackerCreator, ProgressTrackerCreator,
};
use crate::applications::algorithms::metadata::Algorithm;
use crate::config::base_types::Config;
use crate::core::loading::{CatalogLoader, GraphResources, LoadError};
use crate::core::utils::progress::Tasks;
use crate::types::catalog::GraphCatalog;

/// Minimal stream processing template.
///
/// This is the "machinery" that orchestrates stream-mode algorithm execution.
/// It handles loading, progress tracking, and result rendering so that
/// algorithm code just provides the computation.
pub struct StreamProcessingTemplate {
    progress_tracker_creator: DefaultProgressTrackerCreator,
}

impl StreamProcessingTemplate {
    pub fn new() -> Self {
        Self {
            progress_tracker_creator: DefaultProgressTrackerCreator::new(),
        }
    }

    /// Process an algorithm in stream mode.
    ///
    /// # Type Parameters
    /// - `C`: Algorithm configuration type
    /// - `R`: Raw result from algorithm computation
    /// - `O`: Final output type after rendering
    ///
    /// # Arguments
    /// - `catalog`: The graph catalog to load from
    /// - `graph_name`: Name of the graph in the catalog
    /// - `config`: Algorithm configuration
    /// - `algorithm`: The algorithm identity (for progress tracking)
    /// - `compute`: The computation function (takes GraphResources, returns raw result)
    /// - `render`: The rendering function (transforms raw result to output)
    pub fn process<C, R, O>(
        &self,
        catalog: &dyn GraphCatalog,
        graph_name: &str,
        config: &C,
        algorithm: Algorithm,
        compute: impl FnOnce(&GraphResources) -> R,
        render: impl FnOnce(R) -> O,
    ) -> Result<O, StreamProcessingError>
    where
        C: Config,
    {
        // Step 1: Load graph from catalog
        let resources = CatalogLoader::load_or_err(catalog, graph_name)
            .map_err(StreamProcessingError::Load)?;

        // Step 2: Memory guard (placeholder - currently always passes)
        // Future: self.memory_guard.assert_can_run(...)

        // Step 3: Create progress tracker
        let _progress_tracker = self.progress_tracker_creator.create_progress_tracker(
            config,
            Tasks::leaf(algorithm.as_string(), 0),
        );

        // Step 4: Run computation
        let raw_result = compute(&resources);

        // Step 5: Render result
        let output = render(raw_result);

        Ok(output)
    }
}

impl Default for StreamProcessingTemplate {
    fn default() -> Self {
        Self::new()
    }
}

/// Errors that can occur during stream processing.
#[derive(Debug, thiserror::Error)]
pub enum StreamProcessingError {
    #[error("Load failed: {0}")]
    Load(#[from] LoadError),

    #[error("Computation failed: {0}")]
    Computation(String),
}

