use std::{collections::HashSet, fs, path::Path};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct IndexedStore {
    files: HashSet<String>,
}

impl IndexedStore {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self { files: HashSet::new() });
        }
        let data = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)

    }

    pub fn save(&self, path: &Path) -> Result<()> {
        fs::write(path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    pub fn is_indexed(&self, path: &Path) -> bool {
        self.files.contains(&path.to_string_lossy().to_string())
    }

    pub fn mark_as_indexed(&mut self, path: &Path) {
        self.files.insert(path.to_string_lossy().to_string());
    }
}