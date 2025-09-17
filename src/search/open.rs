use anyhow::{Result, Context};
use tantivy::{Index, schema::Schema};
use std::path::Path;

pub fn create_empty_index(dir: &Path, schema: Schema) -> Result<Index> {
    let index = Index::create_in_dir(dir, schema)
        .with_context(|| format!("Creating index in directory: {}", dir.display()))?;
    Ok(index)
}

pub fn open_existing_index(dir: &Path) -> Result<Index>{
    let index = Index::open_in_dir(dir)
        .with_context(|| format!("Opening index from directory: {}", dir.display()))?;
    Ok(index)
}

pub fn open_or_create_index(dir: &Path, schema: Schema) -> Result<Index>{
    if dir.exists() {
        Ok(open_existing_index(dir)?)
    }
    else {
        std::fs::create_dir_all(dir)
            .with_context(|| format!("Creating directory: {}", dir.display()))?;
        Ok(create_empty_index(dir, schema)?)
    }
}

