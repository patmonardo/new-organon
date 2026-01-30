//! Series builders and helpers.

use polars::prelude::{
    IntoSeries, NamedFrom, NewChunkedArray, Series, UInt16Chunked, UInt8Chunked,
};

pub fn series_i64(name: &str, values: &[i64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i32(name: &str, values: &[i32]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i16(name: &str, values: &[i16]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i8(name: &str, values: &[i8]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u64(name: &str, values: &[u64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u32(name: &str, values: &[u32]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_u16(name: &str, values: &[u16]) -> Series {
    UInt16Chunked::from_slice(name.into(), values).into_series()
}

pub fn series_u8(name: &str, values: &[u8]) -> Series {
    UInt8Chunked::from_slice(name.into(), values).into_series()
}

pub fn series_f64(name: &str, values: &[f64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_f32(name: &str, values: &[f32]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_bool(name: &str, values: &[bool]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_str(name: &str, values: &[&str]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_string(name: &str, values: &[String]) -> Series {
    Series::new(name.into(), values)
}
