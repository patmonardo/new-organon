//! Projection type-system walkthrough.
//!
//! A compact, runnable tour of the low-level Projection surface:
//! - Inspect the canonical `ValueType` table
//! - Build typed `PropertyMappings` with defaults and aggregations
#![allow(clippy::all)]
//! - Assemble node/relationship projections and serialize them
//!
//! Run with:
//!   cargo run -p gds --example projection_type_walkthrough

mod enabled {
    use gds::projection::{
        Aggregation, NodeLabel, NodeProjection, Orientation, PropertyMappingBuilder,
        PropertyMappings, RelationshipProjection, RelationshipType,
    };
    use gds::types::DefaultValue;
    use gds::value_type_table;

    pub fn main() {
        if let Err(err) = run() {
            eprintln!("projection_type_walkthrough failed: {err}");
            std::process::exit(1);
        }
    }

    fn run() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("=== Projection type-system walkthrough ===\n");

        print_value_type_table();
        println!();

        let stats_properties = build_stat_properties()?;
        let relationship_properties = build_relationship_properties()?;

        let person_projection = NodeProjection::builder()
            .label(NodeLabel::of("Person"))
            .properties(stats_properties.clone())
            .build();

        let knows_projection = RelationshipProjection::builder()
            .rel_type(RelationshipType::of("KNOWS"))
            .orientation(Orientation::Undirected)
            .aggregation(Aggregation::None) // preserve parallel edges
            .properties(relationship_properties.clone())
            .build()?;

        println!(
            "NodeProjection → label={} props={}",
            person_projection.label().name(),
            stats_properties.size()
        );
        println!(
            "RelationshipProjection → type={} orientation={} props={} aggregation={}",
            knows_projection.rel_type().name(),
            knows_projection.orientation().as_str(),
            relationship_properties.size(),
            knows_projection.aggregation().as_str()
        );
        println!();

        serialize_projection_slice(&stats_properties)?;
        demonstrate_validation_guard()?;

        Ok(())
    }

    fn print_value_type_table() {
        println!("Available ValueTypes (macro-driven):");
        println!(
            "{:<16} {:<20} {:<18} {}",
            "ValueType", "RustType", "Category", "Default"
        );

        macro_rules! describe_type {
            ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {
                let default: $rt = $dv;
                println!(
                    "{:<16} {:<20} {:<18} {:?}",
                    stringify!($vt),
                    stringify!($rt),
                    stringify!($cat),
                    default
                );
            };
        }

        // Use the crate-local macro export to dump the ValueType table.
        value_type_table!(describe_type);
    }

    fn build_stat_properties() -> Result<PropertyMappings, String> {
        // Default aggregation of SUM applied to everything unless overridden.
        Ok(PropertyMappings::builder()
            .with_default_aggregation(Aggregation::Sum)
            .add_mapping(
                PropertyMappingBuilder::new("pagerank")
                    .neo_property_key("gds.pagerank")
                    .aggregation(Aggregation::Max) // override the builder default
                    .build()?,
            )
            .add_mapping(
                PropertyMappingBuilder::new("community_id")
                    .default_value(DefaultValue::long(0))
                    .build()?,
            )
            .add_mapping(
                PropertyMappingBuilder::new("score")
                    .default_value(DefaultValue::double(0.0))
                    .build()?,
            )
            .build())
    }

    fn build_relationship_properties() -> Result<PropertyMappings, String> {
        // Relationship slice that preserves all parallel edges (Aggregation::None everywhere).
        Ok(PropertyMappings::builder()
            .with_default_aggregation(Aggregation::None)
            .add_mapping(
                PropertyMappingBuilder::new("weight")
                    .default_value(DefaultValue::double(1.0))
                    .build()?,
            )
            .add_mapping(PropertyMappingBuilder::new("timestamp").build()?)
            .build())
    }

    fn serialize_projection_slice(mappings: &PropertyMappings) -> Result<(), String> {
        let json = mappings.to_object(true)?;
        println!("Serialized PropertyMappings with aggregation metadata:");
        for (key, value) in json.iter() {
            println!("  {key}: {}", value);
        }
        println!();
        Ok(())
    }

    fn demonstrate_validation_guard() -> Result<(), String> {
        // Mixing NONE with other aggregations is rejected to keep relationship projections sane.
        let bad = PropertyMappings::new(vec![
            PropertyMappingBuilder::new("a")
                .aggregation(Aggregation::None)
                .build()?,
            PropertyMappingBuilder::new("b")
                .aggregation(Aggregation::Sum)
                .build()?,
        ]);

        if let Err(err) = bad {
            println!("Validation guard caught mixed NONE aggregation: {err}");
        }

        Ok(())
    }
}

fn main() {
    enabled::main();
}
