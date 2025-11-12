use anyhow::{Context, Result};
use std::path::Path;
use tantivy::{
    Index,
    schema::{Field}
};

use crate::search::schema;

pub fn run(query: &str, index_dir: &std::path::PathBuf, limit: usize, fields_csv: &str, offset: usize) -> anyhow::Result<()> {
    
    // Open index + schema
    let index = Index::open_in_dir(index_dir)
        .with_context(|| format!("Opening index at {}", index_dir.display()))?;
    let schema = index.schema(); 

    // Get fields from CSV (fallback to title, body, author)
    let default_names: Vec<&str> = fields_csv.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let mut default_fields: Vec<Field> = Vec::new();

    for name in default_names {
        if let Ok(f) = schema.get_field(name) {
            default_fields.push(f);
        }
    }

    if default_fields.is_empty() {
        for name in ["title", "body", "author"] {
            if let Ok(f) = schema.get_field(name) {
                default_fields.push(f);
            }
        }
    }

    // Build query


    Ok(())
}