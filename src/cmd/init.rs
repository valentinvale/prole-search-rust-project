use anyhow::{Result, Context};
use std::path::Path;
use crate::search::{schema::build_schema, open::create_empty_index};
use crate::utils::fsx::ensure_empty_dir;

pub fn run(index_dir: &Path) -> Result<()> {
    // Schema
    let schema = build_schema();

    // Ensure directory is empty
    ensure_empty_dir(index_dir)
        .with_context(|| format!("Preparing: {}", index_dir.display()))?;

    // Create Tantivy index
    create_empty_index(index_dir, schema)?;

    println!("Initialized empty index at {}", index_dir.display());

    Ok(())
}