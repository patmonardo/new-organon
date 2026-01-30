//! Dataset registry for managing dataset families (e.g. pytorch-geometric).

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Common dataset splits.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatasetSplit {
    Train,
    Validation,
    Test,
    All,
    Custom(String),
}

/// Minimal metadata for a dataset entry.
#[derive(Debug, Clone, Default)]
pub struct DatasetMetadata {
    pub name: String,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub tags: Vec<String>,
}

/// Resolved dataset artifact (path + split).
#[derive(Debug, Clone)]
pub struct DatasetArtifact {
    pub split: DatasetSplit,
    pub path: PathBuf,
    pub format_hint: Option<String>,
}

/// In-memory registry for dataset metadata and roots.
#[derive(Debug, Clone)]
pub struct DatasetRegistry {
    root: PathBuf,
    entries: HashMap<String, DatasetMetadata>,
}

impl DatasetRegistry {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: HashMap::new(),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn register(&mut self, metadata: DatasetMetadata) {
        self.entries.insert(metadata.name.clone(), metadata);
    }

    pub fn register_names(&mut self, names: &[&str]) {
        for name in names {
            self.register(DatasetMetadata {
                name: name.to_string(),
                ..Default::default()
            });
        }
    }

    pub fn list(&self) -> Vec<&DatasetMetadata> {
        let mut values: Vec<&DatasetMetadata> = self.entries.values().collect();
        values.sort_by(|a, b| a.name.cmp(&b.name));
        values
    }

    pub fn get(&self, name: &str) -> Option<&DatasetMetadata> {
        self.entries.get(name)
    }

    pub fn dataset_root(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }

    pub fn dataset_artifact(&self, name: &str, split: DatasetSplit) -> DatasetArtifact {
        DatasetArtifact {
            split,
            path: self.dataset_root(name),
            format_hint: None,
        }
    }
}
