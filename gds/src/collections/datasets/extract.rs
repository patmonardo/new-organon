//! Dataset archive extraction helpers.

use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;
use zip::ZipArchive;

use crate::collections::datasets::error::DatasetIoError;

#[derive(Debug, Clone)]
pub struct ExtractReport {
    pub archive: PathBuf,
    pub destination: PathBuf,
    pub files: usize,
}

pub fn extract_archive(
    archive_path: impl AsRef<Path>,
    destination: impl AsRef<Path>,
) -> Result<ExtractReport, DatasetIoError> {
    let archive_path = archive_path.as_ref().to_path_buf();
    let destination = destination.as_ref().to_path_buf();
    std::fs::create_dir_all(&destination).map_err(|e| DatasetIoError::Io(e.to_string()))?;

    let extension = archive_extension(&archive_path);
    let files = match extension.as_deref() {
        Some("zip") => extract_zip(&archive_path, &destination)?,
        Some("tar") => extract_tar(&archive_path, &destination)?,
        Some("tar.gz") | Some("tgz") => extract_tar_gz(&archive_path, &destination)?,
        _ => {
            return Err(DatasetIoError::Unsupported(format!(
                "Unsupported archive format for {:?}",
                archive_path
            )))
        }
    };

    Ok(ExtractReport {
        archive: archive_path,
        destination,
        files,
    })
}

fn archive_extension(path: &Path) -> Option<String> {
    let file_name = path.file_name()?.to_string_lossy();
    if file_name.ends_with(".tar.gz") {
        return Some("tar.gz".to_string());
    }
    if file_name.ends_with(".tgz") {
        return Some("tgz".to_string());
    }
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_string())
}

fn extract_zip(archive_path: &Path, destination: &Path) -> Result<usize, DatasetIoError> {
    let file = File::open(archive_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let mut archive = ZipArchive::new(file).map_err(|e| DatasetIoError::Archive(e.to_string()))?;
    let mut count = 0usize;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| DatasetIoError::Archive(e.to_string()))?;
        let out_path = destination.join(entry.name());
        if entry.is_dir() {
            std::fs::create_dir_all(&out_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
            continue;
        }
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        }
        let mut out_file =
            File::create(&out_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        io::copy(&mut entry, &mut out_file).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        count += 1;
    }

    Ok(count)
}

fn extract_tar(archive_path: &Path, destination: &Path) -> Result<usize, DatasetIoError> {
    let file = File::open(archive_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let mut archive = Archive::new(file);
    unpack_tar(&mut archive, destination)
}

fn extract_tar_gz(archive_path: &Path, destination: &Path) -> Result<usize, DatasetIoError> {
    let file = File::open(archive_path).map_err(|e| DatasetIoError::Io(e.to_string()))?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    unpack_tar(&mut archive, destination)
}

fn unpack_tar<R: io::Read>(
    archive: &mut Archive<R>,
    destination: &Path,
) -> Result<usize, DatasetIoError> {
    let mut count = 0usize;
    for entry in archive
        .entries()
        .map_err(|e| DatasetIoError::Archive(e.to_string()))?
    {
        let mut entry = entry.map_err(|e| DatasetIoError::Archive(e.to_string()))?;
        let path = entry
            .path()
            .map_err(|e| DatasetIoError::Archive(e.to_string()))?
            .to_path_buf();
        let out_path = destination.join(path);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| DatasetIoError::Io(e.to_string()))?;
        }
        entry
            .unpack(&out_path)
            .map_err(|e| DatasetIoError::Archive(e.to_string()))?;
        count += 1;
    }
    Ok(count)
}
