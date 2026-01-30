//! Catalog schema types and Polars mappings.

use polars::prelude::{DataFrame, DataType, Field, PlSmallStr, Schema, TimeUnit};

use crate::types::ValueType;

/// A single schema field.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionsField {
    pub name: String,
    pub value_type: ValueType,
    pub nullable: bool,
    pub time_unit: Option<CollectionsTimeUnit>,
}

/// Time precision for datetime fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CollectionsTimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
}

/// Schema for a catalog entry (table or series).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionsSchema {
    pub fields: Vec<CollectionsField>,
}

impl CollectionsSchema {
    pub fn from_columns(columns: Vec<(&str, ValueType)>) -> Self {
        let fields = columns
            .into_iter()
            .map(|(name, value_type)| CollectionsField {
                name: name.to_string(),
                value_type,
                nullable: false,
                time_unit: None,
            })
            .collect();
        Self { fields }
    }

    pub fn from_polars(df: &DataFrame) -> Self {
        let fields = df
            .get_columns()
            .iter()
            .map(|column| CollectionsField {
                name: column.name().to_string(),
                value_type: polars_dtype_to_value_type(column.dtype()),
                nullable: column.null_count() > 0,
                time_unit: polars_dtype_to_time_unit(column.dtype()),
            })
            .collect();
        Self { fields }
    }

    pub fn to_polars_schema(&self) -> Schema {
        let mut entries: Vec<(PlSmallStr, DataType)> = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            if let Some(dtype) = value_type_to_polars_dtype(field.value_type, field.time_unit) {
                let polars_field = Field::new(field.name.as_str().into(), dtype);
                entries.push((polars_field.name.clone(), polars_field.dtype.clone()));
            }
        }
        Schema::from_iter(entries)
    }
}

pub fn value_type_to_polars_dtype(
    value_type: ValueType,
    time_unit: Option<CollectionsTimeUnit>,
) -> Option<DataType> {
    match value_type {
        ValueType::Byte => Some(DataType::Int8),
        ValueType::Short => Some(DataType::Int16),
        ValueType::Int => Some(DataType::Int32),
        ValueType::Long => Some(DataType::Int64),
        ValueType::Float => Some(DataType::Float32),
        ValueType::Double => Some(DataType::Float64),
        ValueType::Boolean => Some(DataType::Boolean),
        ValueType::String | ValueType::Char => Some(DataType::String),
        ValueType::Date => Some(DataType::Date),
        ValueType::DateTime => Some(DataType::Datetime(
            time_unit
                .map(collections_time_unit_to_polars)
                .unwrap_or(TimeUnit::Milliseconds),
            None,
        )),
        ValueType::ByteArray => Some(DataType::List(Box::new(DataType::Int8))),
        ValueType::ShortArray => Some(DataType::List(Box::new(DataType::Int16))),
        ValueType::IntArray => Some(DataType::List(Box::new(DataType::Int32))),
        ValueType::LongArray => Some(DataType::List(Box::new(DataType::Int64))),
        ValueType::FloatArray => Some(DataType::List(Box::new(DataType::Float32))),
        ValueType::DoubleArray => Some(DataType::List(Box::new(DataType::Float64))),
        ValueType::BooleanArray => Some(DataType::List(Box::new(DataType::Boolean))),
        ValueType::StringArray | ValueType::CharArray => {
            Some(DataType::List(Box::new(DataType::String)))
        }
        ValueType::StringMap
        | ValueType::LongMap
        | ValueType::DoubleMap
        | ValueType::BooleanMap
        | ValueType::StringMapArray
        | ValueType::LongMapArray
        | ValueType::DoubleMapArray
        | ValueType::BooleanMapArray => None,
        _ => None,
    }
}

pub fn polars_dtype_to_value_type(dtype: &DataType) -> ValueType {
    match dtype {
        DataType::Int8 => ValueType::Byte,
        DataType::Int16 => ValueType::Short,
        DataType::Int32 => ValueType::Int,
        DataType::Int64 => ValueType::Long,
        DataType::Float32 => ValueType::Float,
        DataType::Float64 => ValueType::Double,
        DataType::Boolean => ValueType::Boolean,
        DataType::String => ValueType::String,
        DataType::Date => ValueType::Date,
        DataType::Datetime(_, _) => ValueType::DateTime,
        DataType::List(inner) => match inner.as_ref() {
            DataType::Int8 => ValueType::ByteArray,
            DataType::Int16 => ValueType::ShortArray,
            DataType::Int32 => ValueType::IntArray,
            DataType::Int64 => ValueType::LongArray,
            DataType::Float32 => ValueType::FloatArray,
            DataType::Float64 => ValueType::DoubleArray,
            DataType::Boolean => ValueType::BooleanArray,
            DataType::String => ValueType::StringArray,
            _ => ValueType::UntypedArray,
        },
        _ => ValueType::Unknown,
    }
}

pub fn polars_dtype_to_time_unit(dtype: &DataType) -> Option<CollectionsTimeUnit> {
    match dtype {
        DataType::Datetime(unit, _) => Some(polars_time_unit_to_collections(unit)),
        _ => None,
    }
}

fn collections_time_unit_to_polars(unit: CollectionsTimeUnit) -> TimeUnit {
    match unit {
        CollectionsTimeUnit::Nanoseconds => TimeUnit::Nanoseconds,
        CollectionsTimeUnit::Microseconds => TimeUnit::Microseconds,
        CollectionsTimeUnit::Milliseconds => TimeUnit::Milliseconds,
    }
}

fn polars_time_unit_to_collections(unit: &TimeUnit) -> CollectionsTimeUnit {
    match unit {
        TimeUnit::Nanoseconds => CollectionsTimeUnit::Nanoseconds,
        TimeUnit::Microseconds => CollectionsTimeUnit::Microseconds,
        TimeUnit::Milliseconds => CollectionsTimeUnit::Milliseconds,
    }
}
