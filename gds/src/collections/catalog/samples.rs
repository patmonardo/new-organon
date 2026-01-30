//! Sample datasets for Collections Catalog.

use crate::collections::backends::vec::{VecDouble, VecInt, VecLong};
use crate::collections::catalog::disk::CollectionsCatalogDisk;
use crate::collections::catalog::types::{
    CatalogError, CollectionsCatalogDiskEntry, CollectionsIoFormat, CollectionsIoPolicy,
};
use crate::collections::Collections;
use crate::config::CollectionsBackend;
use crate::types::ValueType;

fn build_entry(
    catalog: &CollectionsCatalogDisk,
    name: &str,
    value_type: ValueType,
    format: CollectionsIoFormat,
) -> CollectionsCatalogDiskEntry {
    CollectionsCatalogDiskEntry {
        name: name.to_string(),
        value_type,
        schema: None,
        backend: CollectionsBackend::Vec,
        extensions: Vec::new(),
        io_policy: CollectionsIoPolicy {
            format,
            ..Default::default()
        },
        data_path: catalog.default_data_path(name, format),
    }
}

/// Write a sample i64 series dataset to the catalog.
pub fn write_sample_i64_series(
    catalog: &mut CollectionsCatalogDisk,
    name: &str,
    values: &[i64],
    format: CollectionsIoFormat,
) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
    let entry = build_entry(catalog, name, ValueType::Long, format);
    catalog.register(entry.clone())?;
    let collection = VecLong::from(values.to_vec());
    catalog.write_collection::<i64, _>(&entry, &collection)?;
    Ok(entry)
}

/// Write a sample i32 series dataset to the catalog.
pub fn write_sample_i32_series(
    catalog: &mut CollectionsCatalogDisk,
    name: &str,
    values: &[i32],
    format: CollectionsIoFormat,
) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
    let entry = build_entry(catalog, name, ValueType::Int, format);
    catalog.register(entry.clone())?;
    let collection = VecInt::from(values.to_vec());
    catalog.write_collection::<i32, _>(&entry, &collection)?;
    Ok(entry)
}

/// Write a sample f64 series dataset to the catalog.
pub fn write_sample_f64_series(
    catalog: &mut CollectionsCatalogDisk,
    name: &str,
    values: &[f64],
    format: CollectionsIoFormat,
) -> Result<CollectionsCatalogDiskEntry, CatalogError> {
    let entry = build_entry(catalog, name, ValueType::Double, format);
    catalog.register(entry.clone())?;
    let collection = VecDouble::from(values.to_vec());
    catalog.write_collection::<f64, _>(&entry, &collection)?;
    Ok(entry)
}

/// Write a sample edge list dataset (src/dst) to the catalog.
pub fn write_sample_edge_list(
    catalog: &mut CollectionsCatalogDisk,
    name: &str,
    edges: &[(i64, i64)],
    format: CollectionsIoFormat,
) -> Result<(CollectionsCatalogDiskEntry, CollectionsCatalogDiskEntry), CatalogError> {
    let mut src_values = Vec::with_capacity(edges.len());
    let mut dst_values = Vec::with_capacity(edges.len());
    for (src, dst) in edges {
        src_values.push(*src);
        dst_values.push(*dst);
    }

    let src_name = format!("{name}_src");
    let dst_name = format!("{name}_dst");
    let src_entry = build_entry(catalog, &src_name, ValueType::Long, format);
    let dst_entry = build_entry(catalog, &dst_name, ValueType::Long, format);

    catalog.register(src_entry.clone())?;
    catalog.register(dst_entry.clone())?;

    let src_collection = VecLong::from(src_values);
    let dst_collection = VecLong::from(dst_values);
    catalog.write_collection::<i64, _>(&src_entry, &src_collection)?;
    catalog.write_collection::<i64, _>(&dst_entry, &dst_collection)?;

    Ok((src_entry, dst_entry))
}

/// Read a sample series from the catalog into a Vec.
pub fn read_sample_series<T>(
    catalog: &CollectionsCatalogDisk,
    entry: &CollectionsCatalogDiskEntry,
) -> Result<Vec<T>, CatalogError>
where
    T: crate::collections::catalog::polars_io::PolarsCollectionType,
{
    catalog.read_collection(entry)
}

#[allow(dead_code)]
fn _assert_collections_trait<T: Clone>(collection: &impl Collections<T>) {
    let _ = collection.len();
}
