//! Dataset download helpers.

use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use reqwest::blocking::Response;

use crate::collections::datasets::error::DatasetIoError;

#[derive(Debug, Clone)]
pub struct DownloadReport {
    pub source: String,
    pub destination: PathBuf,
    pub bytes: u64,
}

pub fn download_url(
    url: &str,
    destination: impl AsRef<Path>,
) -> Result<DownloadReport, DatasetIoError> {
    let destination = destination.as_ref().to_path_buf();
    let mut response =
        reqwest::blocking::get(url).map_err(|e| DatasetIoError::Http(e.to_string()))?;
    ensure_success(&response)?;
    write_response(&mut response, &destination, url)
}

pub fn download_if_missing(
    url: &str,
    destination: impl AsRef<Path>,
) -> Result<DownloadReport, DatasetIoError> {
    let destination = destination.as_ref().to_path_buf();
    if destination.exists() {
        let bytes = destination
            .metadata()
            .map_err(|e| DatasetIoError::Io(e.to_string()))?
            .len();
        return Ok(DownloadReport {
            source: url.to_string(),
            destination,
            bytes,
        });
    }
    download_url(url, &destination)
}

pub fn copy_local(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<DownloadReport, DatasetIoError> {
    let source = source.as_ref();
    let destination = destination.as_ref().to_path_buf();
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    }
    std::fs::copy(source, &destination).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let bytes = destination
        .metadata()
        .map_err(|e| DatasetIoError::Io(e.to_string()))?
        .len();
    Ok(DownloadReport {
        source: source.display().to_string(),
        destination,
        bytes,
    })
}

pub fn download_to_dir(
    url: &str,
    directory: impl AsRef<Path>,
) -> Result<DownloadReport, DatasetIoError> {
    let directory = directory.as_ref();
    std::fs::create_dir_all(directory).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let file_name = filename_from_url(url).unwrap_or_else(|| "download.bin".to_string());
    let destination = directory.join(file_name);
    download_url(url, destination)
}

fn filename_from_url(url: &str) -> Option<String> {
    let trimmed = url.split('?').next().unwrap_or(url);
    let name = trimmed.rsplit('/').next()?;
    if name.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn ensure_success(response: &Response) -> Result<(), DatasetIoError> {
    if !response.status().is_success() {
        return Err(DatasetIoError::Http(format!(
            "HTTP {} for {}",
            response.status(),
            response.url()
        )));
    }
    Ok(())
}

fn write_response(
    response: &mut Response,
    destination: &Path,
    source: &str,
) -> Result<DownloadReport, DatasetIoError> {
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    }

    let mut file = File::create(destination).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let mut buffer = Vec::new();
    response
        .read_to_end(&mut buffer)
        .map_err(|e| DatasetIoError::Io(e.to_string()))?;
    file.write_all(&buffer)
        .map_err(|e| DatasetIoError::Io(e.to_string()))?;

    Ok(DownloadReport {
        source: source.to_string(),
        destination: destination.to_path_buf(),
        bytes: buffer.len() as u64,
    })
}

pub fn stream_to_writer(
    mut reader: impl Read,
    mut writer: impl Write,
) -> Result<u64, DatasetIoError> {
    io::copy(&mut reader, &mut writer).map_err(|e| DatasetIoError::Io(e.to_string()))
}
