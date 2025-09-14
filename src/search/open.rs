use anyhow::{Result, Context};
use tantivy::{Index, schema::Schema};
use std::path::Path;

pub fn create_empty_index(dir: &Path, schema: Schema) -> Result<Index> {
    let index = Index::create_in_dir(dir, schema)
        .with_context(|| format!("Creating index in directory: {}", dir.display()))?;
    Ok(index)
}