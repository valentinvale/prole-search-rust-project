use pdf_extract::extract_text;
use anyhow::{Context, Ok, Result};
use std::path::Path;

pub fn read_pdf(path: &Path) -> Result<String> {
    let text = extract_text(path)
        .with_context(|| format!("Extracting text from PDF {}", path.display()))?;
    Ok(text)
}