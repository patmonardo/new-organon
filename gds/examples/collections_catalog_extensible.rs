//! Collections catalog extension example (facade usage).
//!
//! Run with:
//!   cargo run -p gds --example collections_catalog_extensible

use std::path::PathBuf;

use gds::collections::catalog::types::CollectionsIoFormat;
use gds::collections::dataframe::{expr_col, expr_gt, expr_lit_f64};
use gds::collections::dataframe::{scale_f64_column, TableBuilder};
use gds::collections::extensions::catalog::{CatalogExtension, CatalogExtensionConfig};

#[cfg(feature = "polars-lazy")]
use gds::collections::dataframe::{expr_col, expr_gt, expr_lit_f64};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("target/collections_catalog_extensible");

    let config = CatalogExtensionConfig {
        default_format: CollectionsIoFormat::Parquet,
        infer_schema_on_write: true,
        infer_schema_on_read: true,
        validate_on_read: false,
        eager: true,
        auto_save: true,
        ..Default::default()
    };

    let mut catalog = CatalogExtension::new(&root)?.with_config(config);
    run_pipeline(&mut catalog)?;
    print_catalog(catalog.catalog());

    Ok(())
}

fn run_pipeline(catalog: &mut CatalogExtension) -> Result<(), Box<dyn std::error::Error>> {
    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 5, 8, 13, 21])
        .with_f64_column("score", &[10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0])
        .build()?;

    catalog.write_table("sample_table", &table, None)?;

    let mut table = catalog.read_table("sample_table")?;
    println!("Raw table:\n{}", table.fmt_table());
    scale_f64_column(&mut table, "score", 2.0)?;
    println!("Scaled table:\n{}", table.fmt_table());
    catalog.write_table("sample_table_processed", &table, None)?;

    let selected = table.select_columns(&["id", "score"])?;
    println!("Selected columns (eager):\n{}", selected.fmt_table());

    let filtered = table.filter_expr(expr_gt(expr_col("score"), expr_lit_f64(30.0)))?;
    println!("Filtered rows (expr):\n{}", filtered.fmt_table());

    let grouped =
        table.group_by_columns(&["id"], &[expr_col("score").mean().alias("mean_score")])?;
    println!("Grouped table (expr):\n{}", grouped.fmt_table());

    Ok(())
}

fn print_catalog(catalog: &gds::collections::catalog::disk::CollectionsCatalogDisk) {
    println!("Catalog entries:");
    for entry in catalog.list() {
        println!("- {} ({:?})", entry.name, entry.io_policy.format);
        if let Some(schema) = &entry.schema {
            let fields = schema
                .fields
                .iter()
                .map(|field| {
                    let time = field
                        .time_unit
                        .map(|unit| format!("{:?}", unit))
                        .unwrap_or_else(|| "None".to_string());
                    format!(
                        "{}:{} nullable={} time_unit={}",
                        field.name,
                        field.value_type.name(),
                        field.nullable,
                        time
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            println!("  schema: {fields}");
        }
    }
}
