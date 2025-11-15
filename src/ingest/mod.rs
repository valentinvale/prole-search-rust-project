use std::path::Path;

use anyhow::Result;

pub mod text;
pub mod pdf;

// Enum for different ingested document types, add metadata fields later
pub enum IngestedDoc {
    Text{
        content: String,
        // add more metadata later
    },

    Pdf {
        content: String,
        // page count: usize,
        // title: Option<String>,
        // author: Option<String>,
        // creation_date: Option<String>

    }
}

// Ingest a document based on its file extension
pub fn ingest_path(path: &Path) -> Result<Option<IngestedDoc>> {
    let ext = path.extension().and_then(|s| s.to_str()).map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("txt") => {
            let content = text::read_txt(path)?;
            Ok(Some(IngestedDoc::Text { content }))
        }
        Some("pdf") => {
            let content = pdf::read_pdf(path)?;
            Ok(Some(IngestedDoc::Pdf { content }))
        }
        _ => Ok(None),
    }
}
