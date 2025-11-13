use anyhow::{Context, Result};
use std::path::Path;
use tantivy::{
    Document, Index, SnippetGenerator, TantivyDocument, collector::{Count, TopDocs}, query::QueryParser, schema::{Field, Value}
};

use crate::search::schema;

pub fn run(query_str: &str, index_dir: &std::path::PathBuf, limit: usize, fields_csv: &str, offset: usize) -> anyhow::Result<()> {
    
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
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let parser = QueryParser::for_index(&index, default_fields.clone());

    let query = parser.parse_query(query_str)
        .with_context(|| format!("Parsing query: {query_str}"))?;


    // Collect top-docs + total hits for info
    let (top_docs, total) = searcher.search(&query, &(TopDocs::with_limit(limit).and_offset(offset), Count))?;


    // Prepare snippet generator for body field if it exists
    let body_field = schema.get_field("body");
    let mut snip: Option<SnippetGenerator> = match body_field {
        Ok(f) => Some(SnippetGenerator::create(&searcher, &query, f)?),
        Err(_e) => None,
    };

    // Display results
    println!("Total hits: {}", total);
    for (rank, (score, addr)) in top_docs.into_iter().enumerate() {
        let retrieved: TantivyDocument = searcher.doc(addr)?;
        let title = retrieved.get_first(schema.get_field("title").unwrap()).and_then(|v| v.as_str()).unwrap_or("untitled");
        //to do, maybe helper function for getting field value as string
    }


    Ok(())
}