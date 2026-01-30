//! Collections catalog Polars IO example (CSV in, Parquet out).
//!
//! Run with:
//!   cargo run -p gds --example collections_catalog_polars_io

use std::path::PathBuf;

use gds::collections::catalog::disk::CollectionsCatalogDisk;
use gds::collections::catalog::types::{
    CollectionsCatalogDiskEntry, CollectionsIoFormat, CollectionsIoPolicy,
};
use gds::collections::dataframe::{
    read_table_parquet, scale_f64_column, write_table_csv, write_table_parquet, TableBuilder,
};
use gds::config::CollectionsBackend;
use gds::types::ValueType;

fn register_or_replace(
    catalog: &mut CollectionsCatalogDisk,
    entry: CollectionsCatalogDiskEntry,
) -> Result<CollectionsCatalogDiskEntry, Box<dyn std::error::Error>> {
    let _ = catalog.remove(&entry.name);
    catalog.register(entry.clone())?;
    Ok(entry)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from("target/collections_catalog_example");
    let mut catalog = CollectionsCatalogDisk::load(&root)?;

    // --- CSV in (multi-column table) ---
    let table_csv_name = "sample_table_csv";
    let table_csv_entry = CollectionsCatalogDiskEntry {
        name: table_csv_name.to_string(),
        value_type: ValueType::Unknown,
        schema: None,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Csv,
            ..Default::default()
        },
        data_path: catalog.default_data_path(table_csv_name, CollectionsIoFormat::Csv),
    };

    let table = TableBuilder::new()
        .with_i64_column("id", &[1, 2, 3, 5, 8, 13, 21])
        .with_f64_column("score", &[10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0])
        .build()?;

    let table_csv_entry = register_or_replace(&mut catalog, table_csv_entry)?;
    let table_csv_path = root.join(&table_csv_entry.data_path);
    if let Some(parent) = table_csv_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    write_table_csv(&table_csv_path, &table)?;

    // --- Parquet out (multi-column table) ---
    let table_parquet_name = "sample_table_parquet";
    let table_parquet_entry = CollectionsCatalogDiskEntry {
        name: table_parquet_name.to_string(),
        value_type: ValueType::Unknown,
        schema: None,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Parquet,
            ..Default::default()
        },
        data_path: catalog.default_data_path(table_parquet_name, CollectionsIoFormat::Parquet),
    };

    let table_parquet_entry = register_or_replace(&mut catalog, table_parquet_entry)?;
    let table_parquet_path = root.join(&table_parquet_entry.data_path);
    if let Some(parent) = table_parquet_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    write_table_parquet(&table_parquet_path, &table)?;

    // --- Process Parquet table and write a second Parquet ---
    let processed_parquet_name = "sample_table_parquet_processed";
    let processed_parquet_entry = CollectionsCatalogDiskEntry {
        name: processed_parquet_name.to_string(),
        value_type: ValueType::Unknown,
        schema: None,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Parquet,
            ..Default::default()
        },
        data_path: catalog.default_data_path(processed_parquet_name, CollectionsIoFormat::Parquet),
    };

    let mut table_df = read_table_parquet(&table_parquet_path)?;
    println!("Raw table (parquet):\n{}", table_df.fmt_table());
    scale_f64_column(&mut table_df, "score", 2.0)?;
    println!("Scaled table (parquet):\n{}", table_df.fmt_table());

    let processed_parquet_entry = register_or_replace(&mut catalog, processed_parquet_entry)?;
    let processed_parquet_path = root.join(&processed_parquet_entry.data_path);
    if let Some(parent) = processed_parquet_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    write_table_parquet(&processed_parquet_path, &table_df)?;

    // --- Export processed Parquet table to CSV ---
    let processed_csv_name = "sample_table_parquet_processed_csv";
    let processed_csv_entry = CollectionsCatalogDiskEntry {
        name: processed_csv_name.to_string(),
        value_type: ValueType::Unknown,
        schema: None,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Csv,
            ..Default::default()
        },
        data_path: catalog.default_data_path(processed_csv_name, CollectionsIoFormat::Csv),
    };

    let processed_csv_entry = register_or_replace(&mut catalog, processed_csv_entry)?;
    let processed_csv_path = root.join(&processed_csv_entry.data_path);
    if let Some(parent) = processed_csv_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    write_table_csv(&processed_csv_path, &table_df)?;

    catalog.refresh_schema(table_csv_name)?;
    catalog.refresh_schema(table_parquet_name)?;
    catalog.refresh_schema(processed_parquet_name)?;
    catalog.refresh_schema(processed_csv_name)?;

    catalog.save()?;

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

    Ok(())
}
