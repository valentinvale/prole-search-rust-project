use anyhow::{Context};

use tantivy::{
    Index, SnippetGenerator, TantivyDocument, collector::{Count, TopDocs}, query::QueryParser, schema::{Field, Value}
};


pub fn get_string(doc: &TantivyDocument, f: Field) -> Option<&str>{
    doc.get_first(f).and_then(|v| v.as_str()).filter(|s| !s.trim().is_empty())
}

pub fn run(query_str: &str, index_dir: &std::path::PathBuf, limit: usize, fields_csv: &str, offset: usize) -> anyhow::Result<()> {
    
    // Open index + schema
    let index = Index::open_in_dir(index_dir)
        .with_context(|| format!("Opening index at {}", index_dir.display()))?;
    let schema = index.schema(); 

    // Get fields from CSV (fallback to title, content, author)
    let default_names: Vec<&str> = fields_csv.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let mut default_fields: Vec<Field> = Vec::new();

    for name in default_names {
        if let Ok(f) = schema.get_field(name) {
            default_fields.push(f);
        }
    }

    if default_fields.is_empty() {
        for name in ["title", "content", "author"] {
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


    // Prepare snippet generator for content field if it exists
    let content_field = schema.get_field("content");
    let mut snip: Option<SnippetGenerator> = match content_field {
        Ok(f) => Some(SnippetGenerator::create(&searcher, &query, f)?),
        Err(_e) => None,
    };

    // Display results
    println!("Total hits: {}", total);
    for (rank, (score, addr)) in top_docs.into_iter().enumerate() {
        let retrieved: TantivyDocument = searcher.doc(addr)?;
        let title = get_string(&retrieved, schema.get_field("title").unwrap()).unwrap_or("(untitled)");
        let id = get_string(&retrieved, schema.get_field("id").unwrap()).unwrap_or("");
        let author = get_string(&retrieved, schema.get_field("author").unwrap()).unwrap_or("(unknown)");
        let lang = get_string(&retrieved, schema.get_field("language").unwrap()).unwrap_or("(unknown)");

        let snippet = if let Some(ref mut gen_) = snip {
            let s = gen_.snippet_from_doc(&retrieved);
            if s.fragment().is_empty() {
                println!("fragment is empty");
                "".to_string()
            }
            else {
                s.to_html()
            }
        }
        else {
            "".to_string()
        };

        println!("— #{:>2} | score: {:.3} | {}", offset + rank + 1, score, title);
        println!("  author: {}", author);
        println!(" language: {}", lang);
        println!("  id: {}", id);
        if !snippet.is_empty() {
            println!(" snippet: {}", snippet);
        }

        println!();

    }


    Ok(())
}