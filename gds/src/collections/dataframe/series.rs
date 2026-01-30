//! Series builders and helpers.

use polars::prelude::{NamedFrom, Series};

pub fn series_i64(name: &str, values: &[i64]) -> Series {
    Series::new(name.into(), values)
}

pub fn series_i32(name: &str, values: &[i32]) -> Series {
    Series::new(name.into(), values)
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

pub fn series_string(name: &str, values: &[String]) -> Series {
    Series::new(name.into(), values)
}
