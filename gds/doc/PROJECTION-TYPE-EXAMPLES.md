# Projection + Type System Examples

Short cookbook-style snippets showing how the Projection surface (projections, property mappings) ties into the type system (value table, defaults).  
Runnable companions:  
- `cargo run -p gds --example projection_type_walkthrough` (Projection + types)  
- `cargo run -p gds --example values_system_walkthrough` (Values + DefaultValue + runtime factory)
- `cargo run -p gds --example default_graph_store_basics` (Small, Vec-backed DefaultGraphStore tour)

## 1) Inspect the available property types

```rust
use gds::projection::value_type_table;

// Prints the canonical ValueType → Rust type mapping used by codegen.
macro_rules! describe_type {
    ($vt:ident, $rt:ty, $cat:ident, $dv:expr) => {
        println!(
            "{:<16} -> {:<18} {:<18} default: {:?}",
            stringify!($vt),
            stringify!($rt),
            stringify!($cat),
            $dv
        );
    };
}

fn dump_projection_types() {
    value_type_table!(describe_type);
}
```

`value_type_table!` lives at `gds/src/projection/codegen/values/value_type_table.rs` and is the single source of truth for supported value kinds.

## 2) Build node + relationship projections with typed defaults

```rust
use gds::projection::{
    Aggregation, NodeLabel, NodeProjection, Orientation, PropertyMappingBuilder, PropertyMappings,
    RelationshipProjection, RelationshipType,
};
use gds::types::DefaultValue;

// Shared property slice: pagerank (MAX) + community_id (default 0)
let properties = PropertyMappings::builder()
    .with_default_aggregation(Aggregation::Sum)
    .add_mapping(
        PropertyMappingBuilder::new("pagerank")
            .neo_property_key("gds.pagerank")
            .aggregation(Aggregation::Max)
            .build()?,
    )
    .add_mapping(
        PropertyMappingBuilder::new("community_id")
            .default_value(DefaultValue::long(0))
            .build()?,
    )
    .build();

let person_projection = NodeProjection::builder()
    .label(NodeLabel::of("Person"))
    .properties(properties.clone())
    .build();

let knows_projection = RelationshipProjection::builder()
    .rel_type(RelationshipType::of("KNOWS"))
    .orientation(Orientation::Undirected)
    .aggregation(Aggregation::None) // preserve parallel edges
    .properties(properties)
    .build()?;
```

`PropertyMappings` enforces aggregation rules (e.g., `NONE` cannot mix with other aggregations) so the examples stay valid at build time.

## 3) Serialize a projection slice for downstream consumers

```rust
use gds::projection::{PropertyMappingBuilder, PropertyMappings};
use gds::types::DefaultValue;

let mappings = PropertyMappings::builder()
    .add_mapping(
        PropertyMappingBuilder::new("score")
            .default_value(DefaultValue::double(0.0).expect("finite"))
            .build()?,
    )
    .add_mapping(PropertyMappingBuilder::new("labels").build()?)
    .build();

let json = mappings.to_object(true)?; // include aggregation metadata
assert_eq!(json["score"]["defaultValue"].as_f64(), Some(0.0));
```

The resulting JSON blob can be attached to `NodeProjection::to_config()` / `RelationshipProjection::to_config()` calls or shipped to TS clients that mirror the same projection schema.
