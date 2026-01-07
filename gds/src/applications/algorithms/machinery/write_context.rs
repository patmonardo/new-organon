//! WriteContext (Java parity scaffold).
//!
//! Java reference: `WriteContext`.
//! In Java this bundles Neo4j exporter builders, which are procedure-specific.
//!
//! In Rust we don't currently have those APIs, so we keep a thin, typed container
//! with provider traits that can later be backed by real exporters.

/// Marker trait for exporter context.
///
/// Java: `ExporterContext`.
pub trait ExporterContext: Send + Sync {}

pub trait NodeLabelExporterBuilder: Send + Sync {}
pub trait NodePropertyExporterBuilder: Send + Sync {}
pub trait RelationshipExporterBuilder: Send + Sync {}
pub trait RelationshipPropertiesExporterBuilder: Send + Sync {}
pub trait RelationshipStreamExporterBuilder: Send + Sync {}

/// Provider for exporter builders.
///
/// Java: `ExportBuildersProvider`.
pub trait ExportBuildersProvider: Send + Sync {
    fn node_label_exporter_builder(
        &self,
        exporter_context: &dyn ExporterContext,
    ) -> Box<dyn NodeLabelExporterBuilder>;

    fn node_property_exporter_builder(
        &self,
        exporter_context: &dyn ExporterContext,
    ) -> Box<dyn NodePropertyExporterBuilder>;

    fn relationship_exporter_builder(
        &self,
        exporter_context: &dyn ExporterContext,
    ) -> Box<dyn RelationshipExporterBuilder>;

    fn relationship_properties_exporter_builder(
        &self,
        exporter_context: &dyn ExporterContext,
    ) -> Box<dyn RelationshipPropertiesExporterBuilder>;

    fn relationship_stream_exporter_builder(
        &self,
        exporter_context: &dyn ExporterContext,
    ) -> Box<dyn RelationshipStreamExporterBuilder>;
}

/// The parameter object for all things write.
///
/// Java: `record WriteContext(...)`.
pub struct WriteContext {
    pub node_label_exporter_builder: Box<dyn NodeLabelExporterBuilder>,
    pub node_property_exporter_builder: Box<dyn NodePropertyExporterBuilder>,
    pub relationship_exporter_builder: Box<dyn RelationshipExporterBuilder>,
    pub relationship_properties_exporter_builder: Box<dyn RelationshipPropertiesExporterBuilder>,
    pub relationship_stream_exporter_builder: Box<dyn RelationshipStreamExporterBuilder>,
}

impl WriteContext {
    pub fn create(
        export_builders_provider: &dyn ExportBuildersProvider,
        exporter_context: &dyn ExporterContext,
    ) -> Self {
        let node_label_exporter_builder =
            export_builders_provider.node_label_exporter_builder(exporter_context);
        let node_property_exporter_builder =
            export_builders_provider.node_property_exporter_builder(exporter_context);
        let relationship_exporter_builder =
            export_builders_provider.relationship_exporter_builder(exporter_context);
        let relationship_properties_exporter_builder =
            export_builders_provider.relationship_properties_exporter_builder(exporter_context);
        let relationship_stream_exporter_builder =
            export_builders_provider.relationship_stream_exporter_builder(exporter_context);

        Self {
            node_label_exporter_builder,
            node_property_exporter_builder,
            relationship_exporter_builder,
            relationship_properties_exporter_builder,
            relationship_stream_exporter_builder,
        }
    }

    pub fn builder() -> WriteContextBuilder {
        WriteContextBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteContextBuilder {
    node_label_exporter_builder: Option<Box<dyn NodeLabelExporterBuilder>>,
    node_property_exporter_builder: Option<Box<dyn NodePropertyExporterBuilder>>,
    relationship_exporter_builder: Option<Box<dyn RelationshipExporterBuilder>>,
    relationship_properties_exporter_builder:
        Option<Box<dyn RelationshipPropertiesExporterBuilder>>,
    relationship_stream_exporter_builder: Option<Box<dyn RelationshipStreamExporterBuilder>>,
}

impl WriteContextBuilder {
    pub fn with_node_label_exporter_builder(
        mut self,
        builder: Box<dyn NodeLabelExporterBuilder>,
    ) -> Self {
        self.node_label_exporter_builder = Some(builder);
        self
    }

    pub fn with_node_property_exporter_builder(
        mut self,
        builder: Box<dyn NodePropertyExporterBuilder>,
    ) -> Self {
        self.node_property_exporter_builder = Some(builder);
        self
    }

    pub fn with_relationship_exporter_builder(
        mut self,
        builder: Box<dyn RelationshipExporterBuilder>,
    ) -> Self {
        self.relationship_exporter_builder = Some(builder);
        self
    }

    pub fn with_relationship_properties_exporter_builder(
        mut self,
        builder: Box<dyn RelationshipPropertiesExporterBuilder>,
    ) -> Self {
        self.relationship_properties_exporter_builder = Some(builder);
        self
    }

    pub fn with_relationship_stream_exporter_builder(
        mut self,
        builder: Box<dyn RelationshipStreamExporterBuilder>,
    ) -> Self {
        self.relationship_stream_exporter_builder = Some(builder);
        self
    }

    pub fn build(self) -> WriteContext {
        WriteContext {
            node_label_exporter_builder: self
                .node_label_exporter_builder
                .expect("node_label_exporter_builder is required"),
            node_property_exporter_builder: self
                .node_property_exporter_builder
                .expect("node_property_exporter_builder is required"),
            relationship_exporter_builder: self
                .relationship_exporter_builder
                .expect("relationship_exporter_builder is required"),
            relationship_properties_exporter_builder: self
                .relationship_properties_exporter_builder
                .expect("relationship_properties_exporter_builder is required"),
            relationship_stream_exporter_builder: self
                .relationship_stream_exporter_builder
                .expect("relationship_stream_exporter_builder is required"),
        }
    }
}
