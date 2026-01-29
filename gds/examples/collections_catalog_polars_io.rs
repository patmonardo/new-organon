//! Collections catalog Polars IO example (CSV in, Parquet out).
//!
//! Run with:
//!   cargo run -p gds --example collections_catalog_polars_io

use std::path::PathBuf;

use gds::collections::backends::vec::VecLong;
use gds::collections::catalog::disk::CollectionsCatalogDisk;
use gds::collections::catalog::types::{
    CollectionsCatalogDiskEntry, CollectionsIoFormat, CollectionsIoPolicy,
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

    // --- CSV in ---
    let csv_name = "sample_ids_csv";
    let csv_entry = CollectionsCatalogDiskEntry {
        name: csv_name.to_string(),
        value_type: ValueType::Long,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Csv,
            ..Default::default()
        },
        data_path: catalog.default_data_path(csv_name, CollectionsIoFormat::Csv),
    };

    let values = vec![1_i64, 2, 3, 5, 8, 13, 21];
    let csv_collection = VecLong::from(values);
    let csv_entry = register_or_replace(&mut catalog, csv_entry)?;
    catalog.write_collection::<i64, _>(&csv_entry, &csv_collection)?;

    // --- Parquet out ---
    let parquet_name = "sample_ids_parquet";
    let parquet_entry = CollectionsCatalogDiskEntry {
        name: parquet_name.to_string(),
        value_type: ValueType::Long,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Parquet,
            ..Default::default()
        },
        data_path: catalog.default_data_path(parquet_name, CollectionsIoFormat::Parquet),
    };

    let round_trip_values = catalog.read_collection::<i64>(&csv_entry)?;
    let parquet_collection = VecLong::from(round_trip_values);
    let parquet_entry = register_or_replace(&mut catalog, parquet_entry)?;
    catalog.write_collection::<i64, _>(&parquet_entry, &parquet_collection)?;

    // --- Process Parquet and write a second Parquet ---
    let processed_name = "sample_ids_parquet_processed";
    let processed_parquet_entry = CollectionsCatalogDiskEntry {
        name: processed_name.to_string(),
        value_type: ValueType::Long,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Parquet,
            ..Default::default()
        },
        data_path: catalog.default_data_path(processed_name, CollectionsIoFormat::Parquet),
    };

    let parquet_values = catalog.read_collection::<i64>(&parquet_entry)?;
    let processed_values: Vec<i64> = parquet_values.into_iter().map(|v| v * 2).collect();
    let processed_collection = VecLong::from(processed_values);
    let processed_parquet_entry = register_or_replace(&mut catalog, processed_parquet_entry)?;
    catalog.write_collection::<i64, _>(&processed_parquet_entry, &processed_collection)?;

    // --- Export processed Parquet to CSV ---
    let processed_csv_name = "sample_ids_parquet_processed_csv";
    let processed_csv_entry = CollectionsCatalogDiskEntry {
        name: processed_csv_name.to_string(),
        value_type: ValueType::Long,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format: CollectionsIoFormat::Csv,
            ..Default::default()
        },
        data_path: catalog.default_data_path(processed_csv_name, CollectionsIoFormat::Csv),
    };

    let processed_round_trip = catalog.read_collection::<i64>(&processed_parquet_entry)?;
    let processed_csv_collection = VecLong::from(processed_round_trip);
    let processed_csv_entry = register_or_replace(&mut catalog, processed_csv_entry)?;
    catalog.write_collection::<i64, _>(&processed_csv_entry, &processed_csv_collection)?;

    catalog.save()?;

    println!("Catalog entries:");
    for entry in catalog.list() {
        println!("- {} ({:?})", entry.name, entry.io_policy.format);
    }

    Ok(())
}
