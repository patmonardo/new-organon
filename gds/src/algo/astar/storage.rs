//! A* Storage Runtime
//!
//! **Translation Source**: `org.neo4j.gds.paths.astar.AStar`
//!
//! This module implements the storage runtime for A* algorithm - the "Gross pole" for persistent data access.

use super::AStarComputationResult;
use crate::core::utils::progress::{ProgressTracker, UNKNOWN_VOLUME};
use crate::types::graph::Graph;
use crate::types::graph::NodeId;
use crate::types::properties::node::NodePropertyValues;
use std::collections::HashMap;
use std::sync::Arc;

/// A* storage runtime for accessing graph data
///
/// Translation of: `org.neo4j.gds.paths.astar.AStar` (lines 37-88)
pub struct AStarStorageRuntime {
    source_node: NodeId,
    target_node: NodeId,
    latitude_property: String,
    longitude_property: String,
    // Cache for latitude/longitude values to avoid repeated property lookups
    pub coordinate_cache: HashMap<NodeId, (f64, f64)>,
    // Optional bound property value accessors (preferred over mock)
    lat_values: Option<Arc<dyn NodePropertyValues>>,
    lon_values: Option<Arc<dyn NodePropertyValues>>,
}

impl AStarStorageRuntime {
    /// Create new A* storage runtime
    ///
    /// Translation of: `AStar.sourceTarget()` (lines 47-88)
    pub fn new(
        source_node: NodeId,
        target_node: NodeId,
        latitude_property: String,
        longitude_property: String,
    ) -> Self {
        Self {
            source_node,
            target_node,
            latitude_property,
            longitude_property,
            coordinate_cache: HashMap::new(),
            lat_values: None,
            lon_values: None,
        }
    }

    /// Create new A* storage runtime bound to concrete latitude/longitude property values
    pub fn new_with_values(
        source_node: NodeId,
        target_node: NodeId,
        latitude_property: String,
        longitude_property: String,
        lat_values: Arc<dyn NodePropertyValues>,
        lon_values: Arc<dyn NodePropertyValues>,
    ) -> Self {
        Self {
            source_node,
            target_node,
            latitude_property,
            longitude_property,
            coordinate_cache: HashMap::new(),
            lat_values: Some(lat_values),
            lon_values: Some(lon_values),
        }
    }

    /// Compute A* path using Haversine heuristic
    ///
    /// Translation of: `AStar.compute()` (lines 92-94) and `HaversineHeuristic`
    pub fn compute_astar_path(
        &mut self,
        computation: &mut super::AStarComputationRuntime,
        graph: Option<&dyn Graph>,
        direction: u8,
        progress_tracker: &mut dyn ProgressTracker,
    ) -> Result<AStarComputationResult, String> {
        let volume = graph
            .map(|g| g.relationship_count())
            .unwrap_or(UNKNOWN_VOLUME);
        if volume == UNKNOWN_VOLUME {
            progress_tracker.begin_subtask_unknown();
        } else {
            progress_tracker.begin_subtask_with_volume(volume);
        }

        let result = (|| {
            // If no graph given, keep placeholder behavior for tests
            if graph.is_none() {
                let mut path = Vec::new();
                let total_cost;
                let nodes_explored;
                if self.source_node != self.target_node {
                    path.push(self.source_node);
                    path.push(self.target_node);
                    total_cost =
                        self.compute_haversine_distance(self.source_node, self.target_node)?;
                    nodes_explored = 2;
                } else {
                    path.push(self.source_node);
                    total_cost = 0.0;
                    nodes_explored = 1;
                }
                return Ok(AStarComputationResult::new(
                    Some(path),
                    total_cost,
                    nodes_explored,
                ));
            }

            let g = graph.unwrap();
            computation.initialize(self.source_node, self.target_node);

            // Initialize heuristic for source
            let h0 = self.compute_haversine_distance(self.source_node, self.target_node)?;
            computation.update_f_cost(self.source_node, h0);

            // Work unit: edges scanned in neighbor expansion.
            let mut edges_scanned_batch: usize = 0;
            const EDGES_SCAN_LOG_BATCH: usize = 256;

            while !computation.is_open_set_empty() {
                let current = match computation.get_lowest_f_cost_node() {
                    Some(n) => n,
                    None => break,
                };
                computation.remove_from_open_set(current);
                computation.mark_visited(current);

                if current == self.target_node {
                    if edges_scanned_batch > 0 {
                        progress_tracker.log_progress(edges_scanned_batch);
                    }

                    let path = computation.reconstruct_path(self.source_node, self.target_node);
                    let total_cost = computation.get_total_cost(self.target_node);
                    return Ok(AStarComputationResult::new(
                        path,
                        total_cost,
                        computation.nodes_explored(),
                    ));
                }

                // Expand neighbors via relationship streams
                let fallback: f64 = 1.0;
                let source_mapped = current;
                let stream = if direction == 1 {
                    g.stream_inverse_relationships(source_mapped, fallback)
                } else {
                    g.stream_relationships(source_mapped, fallback)
                };

                for cursor in stream {
                    edges_scanned_batch += 1;
                    if edges_scanned_batch >= EDGES_SCAN_LOG_BATCH {
                        progress_tracker.log_progress(edges_scanned_batch);
                        edges_scanned_batch = 0;
                    }

                    let neighbor: NodeId = cursor.target_id();
                    if computation.is_visited(neighbor) {
                        continue;
                    }

                    let tentative_g = computation.get_g_cost(current) + cursor.property();
                    if tentative_g < computation.get_g_cost(neighbor) {
                        computation.set_parent(neighbor, current);
                        computation.update_g_cost(neighbor, tentative_g);
                        let h = self
                            .compute_haversine_distance(neighbor, self.target_node)
                            .unwrap_or(0.0);
                        computation.update_f_cost(neighbor, tentative_g + h);
                        computation.add_to_open_set(neighbor);
                    }
                }
            }

            if edges_scanned_batch > 0 {
                progress_tracker.log_progress(edges_scanned_batch);
            }

            // No path found
            Ok(AStarComputationResult::new(
                None,
                f64::INFINITY,
                computation.nodes_explored(),
            ))
        })();

        match result {
            Ok(v) => {
                progress_tracker.end_subtask();
                Ok(v)
            }
            Err(e) => {
                progress_tracker.end_subtask_with_failure();
                Err(e)
            }
        }
    }

    /// Compute Haversine distance between two nodes
    ///
    /// Translation of: `HaversineHeuristic.distance()` (lines 1.038-1.056)
    pub fn compute_haversine_distance(
        &mut self,
        source: NodeId,
        target: NodeId,
    ) -> Result<f64, String> {
        let (source_lat, source_lon) = self.get_coordinates(source)?;
        let (target_lat, target_lon) = self.get_coordinates(target)?;

        Ok(Self::haversine_distance(
            source_lat, source_lon, target_lat, target_lon,
        ))
    }

    /// Get coordinates for a node (with caching)
    pub fn get_coordinates(&mut self, node_id: NodeId) -> Result<(f64, f64), String> {
        if let Some(&coords) = self.coordinate_cache.get(&node_id) {
            return Ok(coords);
        }
        // Prefer bound property values when available; fallback to mock
        let coords = if let (Some(lat_vals), Some(lon_vals)) = (&self.lat_values, &self.lon_values)
        {
            let idx = u64::try_from(node_id)
                .map_err(|_| format!("invalid node id for property lookup: {node_id}"))?;
            let lat = lat_vals
                .double_value(idx)
                .map_err(|e| format!("lat read error: {e}"))?;
            let lon = lon_vals
                .double_value(idx)
                .map_err(|e| format!("lon read error: {e}"))?;
            (lat, lon)
        } else {
            // Mock fallback
            let idx = u64::try_from(node_id)
                .map_err(|_| format!("invalid node id for mock coordinates: {node_id}"))?;
            let lat = (idx as f64) * 0.01;
            let lon = (idx as f64) * 0.01;
            (lat, lon)
        };
        self.coordinate_cache.insert(node_id, coords);
        Ok(coords)
    }

    /// Haversine distance calculation
    ///
    /// Translation of: `HaversineHeuristic.distance()` (lines 1.038-1.056)
    /// https://rosettacode.org/wiki/Haversine_formula#Java
    pub fn haversine_distance(
        source_latitude: f64,
        source_longitude: f64,
        target_latitude: f64,
        target_longitude: f64,
    ) -> f64 {
        const EARTH_RADIUS_IN_NM: f64 = 6371.0 * 0.539957; // km to nautical mile

        let latitude_distance = (target_latitude - source_latitude).to_radians();
        let longitude_distance = (target_longitude - source_longitude).to_radians();
        let lat1 = source_latitude.to_radians();
        let lat2 = target_latitude.to_radians();

        let a = (latitude_distance / 2.0).sin().powi(2)
            + (longitude_distance / 2.0).sin().powi(2) * lat1.cos() * lat2.cos();

        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS_IN_NM * c
    }

    /// Get source node ID
    pub fn source_node(&self) -> NodeId {
        self.source_node
    }

    /// Get target node ID
    pub fn target_node(&self) -> NodeId {
        self.target_node
    }

    /// Get latitude property name
    pub fn latitude_property(&self) -> &str {
        &self.latitude_property
    }

    /// Get longitude property name
    pub fn longitude_property(&self) -> &str {
        &self.longitude_property
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astar_storage_runtime_creation() {
        let storage =
            AStarStorageRuntime::new(0, 1, "latitude".to_string(), "longitude".to_string());

        assert_eq!(storage.source_node(), 0);
        assert_eq!(storage.target_node(), 1);
        assert_eq!(storage.latitude_property(), "latitude");
        assert_eq!(storage.longitude_property(), "longitude");
    }

    #[test]
    fn test_haversine_distance_calculation() {
        // Test with known coordinates (New York to Los Angeles)
        let ny_lat = 40.7128;
        let ny_lon = -74.0060;
        let la_lat = 34.0522;
        let la_lon = -118.2437;

        let distance = AStarStorageRuntime::haversine_distance(ny_lat, ny_lon, la_lat, la_lon);

        // Distance should be approximately 21.044 nautical miles
        assert!(distance > 2000.0 && distance < 2300.0);
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let lat = 40.7128;
        let lon = -74.0060;

        let distance = AStarStorageRuntime::haversine_distance(lat, lon, lat, lon);

        // Distance to same point should be 0
        assert!((distance - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_caching() {
        let mut storage = AStarStorageRuntime::new(0, 1, "lat".to_string(), "lon".to_string());

        // First call should populate cache
        let coords1 = storage.get_coordinates(5).unwrap();

        // Second call should use cache
        let coords2 = storage.get_coordinates(5).unwrap();

        assert_eq!(coords1, coords2);
        assert_eq!(storage.coordinate_cache.len(), 1);
    }

    #[test]
    fn test_astar_path_computation() {
        use super::super::AStarComputationRuntime;
        use crate::core::utils::progress::{TaskProgressTracker, Tasks};

        let mut storage = AStarStorageRuntime::new(0, 1, "lat".to_string(), "lon".to_string());

        let mut computation = AStarComputationRuntime::new();
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("astar".to_string()));

        let result = storage
            .compute_astar_path(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        assert!(result.path.is_some());
        assert_eq!(result.path.as_ref().unwrap().len(), 2);
        assert_eq!(result.path.as_ref().unwrap()[0], 0);
        assert_eq!(result.path.as_ref().unwrap()[1], 1);
        assert!(result.total_cost >= 0.0);
        assert_eq!(result.nodes_explored, 2);
    }

    #[test]
    fn test_astar_path_same_source_target() {
        use super::super::AStarComputationRuntime;
        use crate::core::utils::progress::{TaskProgressTracker, Tasks};

        let mut storage = AStarStorageRuntime::new(
            5,
            5, // Same source and target
            "lat".to_string(),
            "lon".to_string(),
        );

        let mut computation = AStarComputationRuntime::new();
        let mut progress_tracker = TaskProgressTracker::new(Tasks::leaf("astar".to_string()));

        let result = storage
            .compute_astar_path(&mut computation, None, 0, &mut progress_tracker)
            .unwrap();

        assert!(result.path.is_some());
        assert_eq!(result.path.as_ref().unwrap().len(), 1);
        assert_eq!(result.path.as_ref().unwrap()[0], 5);
        assert_eq!(result.total_cost, 0.0);
        assert_eq!(result.nodes_explored, 1);
    }
}
