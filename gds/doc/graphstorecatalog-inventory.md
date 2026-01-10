GraphStoreCatalog — Java sources inventory

This document maps the Java classes in the `graphstorecatalog` folder to their primary responsibilities. Use this as the reference for the migration/JSX plan.

- `CatalogConfigurationService.java`: configuration entrypoints for catalog behavior and defaults. → Rust: TODO (not ported)
- `CypherProjectApplication.java`: Cypher-based projection / project orchestration. → Rust: TODO (not ported)
- `DefaultGraphCatalogApplications.java`: Facade exposing the assembled catalog applications. → Rust: [gds/src/applications/graph_store_catalog/facade/default_graph_catalog_applications.rs](gds/src/applications/graph_store_catalog/facade/default_graph_catalog_applications.rs)
- `DefaultGraphCatalogApplicationsBuilder.java`: Builder that wires dependencies for the default facade. → Rust: [gds/src/applications/graph_store_catalog/facade/default_graph_catalog_applications_builder.rs](gds/src/applications/graph_store_catalog/facade/default_graph_catalog_applications_builder.rs)
- `DegreeDistributionApplier.java`: Applies or materializes degree-distribution results on stores. → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/degree_distribution_applier.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/degree_distribution_applier.rs)
- `DegreeDistributionService.java`: Computes degree distribution statistics/histograms. → Rust: [gds/src/applications/graph_store_catalog/services/degree_distribution_service.rs](gds/src/applications/graph_store_catalog/services/degree_distribution_service.rs)
- `DropGraphApplication.java`: Removes graphs from the catalog / handles drop semantics. → Rust: [gds/src/applications/graph_store_catalog/applications/simple_applications/drop_graph_application.rs](gds/src/applications/graph_store_catalog/applications/simple_applications/drop_graph_application.rs)
- `DropNodePropertiesApplication.java`: Removes node properties from a graph store.
  → Rust: [gds/src/applications/graph_store_catalog/applications/simple_applications/drop_node_properties_application.rs](gds/src/applications/graph_store_catalog/applications/simple_applications/drop_node_properties_application.rs)
- `DropRelationshipsApplication.java`: Removes relationship types or relationships from a store. → Rust: [gds/src/applications/graph_store_catalog/applications/simple_applications/drop_relationships_application.rs](gds/src/applications/graph_store_catalog/applications/simple_applications/drop_relationships_application.rs)
- `EstimateCommonNeighbourAwareRandomWalkApplication.java`: Memory estimate for CN-aware RWR sampling. → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/estimate_common_neighbour_aware_random_walk_application.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/estimate_common_neighbour_aware_random_walk_application.rs)
- `ExportLocation.java`: Export destination representation (CSV, DB, stream). → Rust: [gds/src/applications/graph_store_catalog/services/export_location.rs](gds/src/applications/graph_store_catalog/services/export_location.rs)
- `ExportToCsvApplication.java`: Streams graph data to CSV sinks. → Rust: [gds/src/applications/graph_store_catalog/applications/export_applications/export_to_csv_application.rs](gds/src/applications/graph_store_catalog/applications/export_applications/export_to_csv_application.rs)
- `ExportToCsvEstimateApplication.java`: Estimates cost/size/cost for CSV export. → Rust: [gds/src/applications/graph_store_catalog/applications/export_applications/export_to_csv_estimate_application.rs](gds/src/applications/graph_store_catalog/applications/export_applications/export_to_csv_estimate_application.rs)
- `ExportToDatabaseApplication.java`: Streams graph data into target databases. → Rust: [gds/src/applications/graph_store_catalog/applications/export_applications/export_to_database_application.rs](gds/src/applications/graph_store_catalog/applications/export_applications/export_to_database_application.rs)
- `FictitiousGraphStoreLoader.java`: Test/mock loader that creates synthetic/fake stores. → Rust: [gds/src/applications/graph_store_catalog/loaders/fictitious_graph_store_loader.rs](gds/src/applications/graph_store_catalog/loaders/fictitious_graph_store_loader.rs)
- `GenerateGraphApplication.java`: Generates synthetic graphs and inserts them into the catalog. → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/generate_graph_application.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/generate_graph_application.rs)
- `GenericProjectApplication.java`: Generic projection operations (higher-level project API). → Rust: [gds/src/applications/graph_store_catalog/applications/project_applications/generic_project_application.rs](gds/src/applications/graph_store_catalog/applications/project_applications/generic_project_application.rs)
- `GraphAccessGraphPropertiesConfig.java`: Config for accessing graph-level properties. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_access_graph_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_access_graph_properties_config.rs)
- `GraphCatalogApplications.java`: High-level entrypoints used by services to dispatch catalog ops. → Rust: [gds/src/applications/graph_store_catalog/facade/graph_catalog_applications.rs](gds/src/applications/graph_store_catalog/facade/graph_catalog_applications.rs)
- `GraphDropNodePropertiesConfig.java`: Config for node-property drop operations. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_drop_node_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_drop_node_properties_config.rs)
- `GraphExportNodePropertiesConfig.java`: Config controlling node-property exports. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_export_node_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_export_node_properties_config.rs)
- `GraphListingService.java`: Lists graphs and their metadata (including degree dist if asked). → Rust: [gds/src/applications/graph_store_catalog/services/graph_listing_service.rs](gds/src/applications/graph_store_catalog/services/graph_listing_service.rs)
- `GraphMemoryUsageApplication.java`: Computes/returns memory usage estimates for stored graphs. → Rust: [gds/src/applications/graph_store_catalog/applications/simple_applications/graph_memory_usage_application.rs](gds/src/applications/graph_store_catalog/applications/simple_applications/graph_memory_usage_application.rs)
- `GraphNameValidationService.java`: Validates graph names for collisions/format/constraints. → Rust: [gds/src/applications/graph_store_catalog/services/graph_name_validation_service.rs](gds/src/applications/graph_store_catalog/services/graph_name_validation_service.rs)
- `GraphNodePropertiesConfig.java`: Node property write/config schema. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_node_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_node_properties_config.rs)
- `GraphProjectMemoryUsageService.java`: Memory estimation for projection operations. → Rust: [gds/src/applications/graph_store_catalog/applications/project_applications/graph_project_memory_usage_service.rs](gds/src/applications/graph_store_catalog/applications/project_applications/graph_project_memory_usage_service.rs)
- `GraphRemoveGraphPropertiesConfig.java`: Config for removing graph-level properties. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_remove_graph_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_remove_graph_properties_config.rs)
- `GraphSamplingApplication.java`: Sampling entrypoints (RWR, random, induced-subgraph sampling). → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/graph_sampling_application.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/graph_sampling_application.rs)
- `GraphStoreCreator.java`: Utilities to create/populate `DefaultGraphStore` instances. → Rust: [gds/src/applications/graph_store_catalog/loaders/graph_store_creator.rs](gds/src/applications/graph_store_catalog/loaders/graph_store_creator.rs)
- `GraphStoreFromCatalogLoader.java`: Loader resolving stores already present in the catalog. → Rust: [gds/src/applications/graph_store_catalog/loaders/graph_store_from_catalog_loader.rs](gds/src/applications/graph_store_catalog/loaders/graph_store_from_catalog_loader.rs)
- `GraphStoreFromDatabaseLoader.java`: Loader that imports stores from a backing DB. → Rust: [gds/src/applications/graph_store_catalog/loaders/graph_store_from_database_loader.rs](gds/src/applications/graph_store_catalog/loaders/graph_store_from_database_loader.rs)
- `GraphStoreLoader.java`: Loader interface (contract for loaders used by applications). → Rust: [gds/src/applications/graph_store_catalog/loaders/graph_store_loader.rs](gds/src/applications/graph_store_catalog/loaders/graph_store_loader.rs)
- `GraphStoreValidationService.java`: Validates store invariants and structural integrity. → Rust: [gds/src/applications/graph_store_catalog/services/graph_store_validation_service.rs](gds/src/applications/graph_store_catalog/services/graph_store_validation_service.rs)
- `GraphStreamGraphPropertiesConfig.java`: Config for streaming graph properties. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_stream_graph_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_stream_graph_properties_config.rs)
- `GraphStreamNodePropertiesConfig.java`: Config for streaming node properties. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_stream_node_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_stream_node_properties_config.rs)
- `GraphStreamNodePropertyOrPropertiesResultProducer.java`: Produces streamed node-property results. → Rust: [gds/src/applications/graph_store_catalog/services/result_producers/graph_stream_node_property_result_producer.rs](gds/src/applications/graph_store_catalog/services/result_producers/graph_stream_node_property_result_producer.rs)
- `GraphStreamRelationshipPropertiesConfig.java`: Config for streaming relationship properties. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_stream_relationship_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_stream_relationship_properties_config.rs)
- `GraphStreamRelationshipPropertyOrPropertiesResultProducer.java`: Produces streamed relationship-property results. → Rust: [gds/src/applications/graph_store_catalog/services/result_producers/graph_stream_relationship_property_result_producer.rs](gds/src/applications/graph_store_catalog/services/result_producers/graph_stream_relationship_property_result_producer.rs)
- `GraphStreamRelationshipsConfig.java`: Config for streaming relationships. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_stream_relationships_config.rs](gds/src/applications/graph_store_catalog/configs/graph_stream_relationships_config.rs)
- `GraphWriteNodePropertiesConfig.java`: Config for node-property writes. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_write_node_properties_config.rs](gds/src/applications/graph_store_catalog/configs/graph_write_node_properties_config.rs)
- `GraphWriteRelationshipConfig.java`: Config for relationship-property writes. → Rust: [gds/src/applications/graph_store_catalog/configs/graph_write_relationship_config.rs](gds/src/applications/graph_store_catalog/configs/graph_write_relationship_config.rs)
- `ListGraphApplication.java`: Application that returns graph listings (JSON/TSJSON surface). → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/list_graph_application.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/list_graph_application.rs)
- `MemoryUsageValidator.java`: Validator for memory usage thresholds and sanity checks. → Rust: [gds/src/applications/graph_store_catalog/services/memory_usage_validator.rs](gds/src/applications/graph_store_catalog/services/memory_usage_validator.rs)
- `MutateLabelConfig.java`: Config object for label mutation operations. → Rust: [gds/src/applications/graph_store_catalog/configs/mutate_label_config.rs](gds/src/applications/graph_store_catalog/configs/mutate_label_config.rs)
- `NativeProjectApplication.java`: Native projection implementation bindings. → Rust: [gds/src/applications/graph_store_catalog/applications/project_applications/native_project_application.rs](gds/src/applications/graph_store_catalog/applications/project_applications/native_project_application.rs)
- `NodeFilterParser.java`: Parses Cypher-like node filter expressions (lightweight parser). → Rust: [gds/src/applications/graph_store_catalog/applications/simple_applications/node_filter_parser.rs](gds/src/applications/graph_store_catalog/applications/simple_applications/node_filter_parser.rs)
- `NodeLabelMutatorApplication.java`: Adds/removes/mutates node labels on stores. → Rust: [gds/src/applications/graph_store_catalog/applications/simple_applications/node_label_mutator_application.rs](gds/src/applications/graph_store_catalog/applications/simple_applications/node_label_mutator_application.rs)
- `PreconditionsService.java`: Centralized input validation and precondition checks. → Rust: [gds/src/applications/graph_store_catalog/services/preconditions_service.rs](gds/src/applications/graph_store_catalog/services/preconditions_service.rs)
- `ProgressTrackerFactory.java`: Factory for creating progress tracking/logging registries. → Rust: [gds/src/applications/graph_store_catalog/services/progress_tracker_factory.rs](gds/src/applications/graph_store_catalog/services/progress_tracker_factory.rs)
- `RandomWalkWithRestartsConfiguration.java`: Configuration DTO for RWR sampling variants. → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/random_walk_with_restarts_configuration.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/random_walk_with_restarts_configuration.rs)
- `SamplerProvider.java`: Sampling helpers and deterministic sampler provisioning. → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/sampler_provider.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/sampler_provider.rs)
- `StreamNodePropertiesApplication.java`: Streams node properties to consumers. → Rust: [gds/src/applications/graph_store_catalog/applications/stream_applications/stream_node_properties_application.rs](gds/src/applications/graph_store_catalog/applications/stream_applications/stream_node_properties_application.rs)
- `StreamRelationshipPropertiesApplication.java`: Streams relationship properties to consumers. → Rust: [gds/src/applications/graph_store_catalog/applications/stream_applications/stream_relationship_properties_application.rs](gds/src/applications/graph_store_catalog/applications/stream_applications/stream_relationship_properties_application.rs)
- `StreamRelationshipsApplication.java`: Streams relationships (edges) out of a store. → Rust: [gds/src/applications/graph_store_catalog/applications/stream_applications/stream_relationships_application.rs](gds/src/applications/graph_store_catalog/applications/stream_applications/stream_relationships_application.rs)
- `SubGraphProjectApplication.java`: Projects subgraphs using filters and sampling. → Rust: [gds/src/applications/graph_store_catalog/applications/complex_applications/sub_graph_project_application.rs](gds/src/applications/graph_store_catalog/applications/complex_applications/sub_graph_project_application.rs)
- `UserInputWriteProperties.java`: DTO for user-supplied write property requests. → Rust: [gds/src/applications/graph_store_catalog/services/user_input_write_properties.rs](gds/src/applications/graph_store_catalog/services/user_input_write_properties.rs)
- `WriteLabelConfig.java`: Configuration for write label operations. → Rust: [gds/src/applications/graph_store_catalog/configs/write_label_config.rs](gds/src/applications/graph_store_catalog/configs/write_label_config.rs)
- `WriteNodeLabelApplication.java`: Adds node labels as an application entrypoint. → Rust: [gds/src/applications/graph_store_catalog/applications/write_applications/write_node_label_application.rs](gds/src/applications/graph_store_catalog/applications/write_applications/write_node_label_application.rs)
- `WriteNodePropertiesApplication.java`: Writes node properties (bulk or streaming). → Rust: [gds/src/applications/graph_store_catalog/applications/write_applications/write_node_properties_application.rs](gds/src/applications/graph_store_catalog/applications/write_applications/write_node_properties_application.rs)
- `WriteRelationshipPropertiesApplication.java`: Config for relationship-property writes. → Rust: [gds/src/applications/graph_store_catalog/applications/write_applications/write_relationship_properties_application.rs](gds/src/applications/graph_store_catalog/applications/write_applications/write_relationship_properties_application.rs)
- `WriteRelationshipsApplication.java`: Adds relationships or relationship types to a graph store. → Rust: [gds/src/applications/graph_store_catalog/applications/write_applications/write_relationships_application.rs](gds/src/applications/graph_store_catalog/applications/write_applications/write_relationships_application.rs)

- Many Java classes are orchestration/facade layers that validate inputs, build the storage and computation runtimes, and delegate to algo/procedure code — follow the Procedure-First pattern from the arch-review.
- Several loader/writer classes are present but may be stubs or thin adapters; when wiring a TS/JS declarative layer, prefer emitting the canonical JSON consumed by the Java-side deserializer.

Next action: prototype a canonical JSON schema for GraphStore declarations and a tiny TypeScript JSX example that compiles to that schema.

Follow-up Plan (recorded progress)

1. Prototype canonical schema (TS validator)

- Status: in-progress
- Outcome: Zod/JSON-Schema file + small TS `compileGraphStore()` that emits canonical JSON for `generate`, `sample`, `export`.

2. JSX/TSX examples and compilation artifacts

- Status: not-started
- Outcome: three example snippets (`Generate`, `Sample`, `Export`) and their compiled canonical JSON outputs.

3. Java-side deserializer adapter

- Status: not-started
- Outcome: lightweight Java entrypoint that accepts canonical JSON and delegates to `GraphCatalogApplications` facade methods.

4. Tests and compatibility

- Status: not-started
- Outcome: integration tests that run `generate` and `sample` via compiled JSON and assert deterministic behavior (seed), and a compatibility transformer from old TS-JSON.

5. Improve applications error contract

- Status: not-started
- Outcome: define typed error shape (code/message/details) and migrate `GraphCatalogApplications` methods to return typed errors instead of `String` where practical.

6. Catalog wiring & small stubs completion

- Status: in-progress
- Outcome: complete remaining small Rust/Java stubs (loaders, writers) and wire `SamplerProvider`/`DegreeDistributionApplier` into sampling/listing flows.

Next immediate step I will take: implement the TS prototype (Zod schema + `compileGraphStore`) for `generate`, `sample`, and `export` as a small package in `gds/tools/graphstore-jsx-prototype`.
