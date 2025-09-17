use anyhow::{Context, Result};
use walkdir::WalkDir;
use std::{fs, path::Path};
use tantivy::{doc, ReloadPolicy};

use crate::search::{open::open_or_create_index, schema::build_schema};


pub fn run(corpus_dir: &Path, index_dir: &Path) -> Result<()>{
    let schema = build_schema();
    let index = open_or_create_index(index_dir, schema)?;

    let mut writer = index.writer(50_000_000)?;
    let sch = index.schema();

    let id_f = sch.get_field("id").unwrap();
    let title_f    = sch.get_field("title").unwrap();
    let author_f   = sch.get_field("author").unwrap();
    let authorx_f  = sch.get_field("author_exact").unwrap();
    let lang_f     = sch.get_field("language").unwrap();
    let year_f     = sch.get_field("year").unwrap();
    let content_f  = sch.get_field("content").unwrap();
    // add tag field later

    let mut count = 0usize;

    for entry in WalkDir::new(corpus_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let is_txt = path.is_file()
            && path.extension()
                .and_then(|s| s.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("txt"))
                .unwrap_or(false);

        if !is_txt{
            continue;
        } 

        let bytes = fs::read(path)
            .with_context(|| format!("Reading {}", path.display()))?;
        let content = String::from_utf8_lossy(&bytes).into_owned(); // maybe add smart decoding later
        
        let title = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled");

        let id = path.to_string_lossy().to_string();

        writer.add_document(doc!(
            id_f      => id,
            title_f   => title.to_string(),
            author_f  => "",     // fill later
            authorx_f => "",     // fill later
            lang_f    => "",     // fill later
            year_f    => 0i64,   // fill later
            content_f => content
        ))
            .with_context(|| format!("Indexing: {}", path.display()))?;
        count += 1;

        writer.commit()?;

        // reload readers so subsequent searches see new docs
        index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommitWithDelay)
            .try_into()?
            .reload()?;

        println!("Indexed {} document(s).", count);
        

    }
    Ok(())
}