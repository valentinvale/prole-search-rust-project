use tantivy::schema::*;

pub fn build_schema() -> Schema {
    let mut sb = Schema::builder();
    sb.add_text_field("id", STRING | STORED);
    sb.add_text_field("title", TEXT | STORED);
    sb.add_text_field("author", TEXT | STORED);
    sb.add_text_field("language", STRING | STORED);
    sb.add_text_field("author_exact", STRING | STORED);
    sb.add_i64_field("year", FAST | STORED);
    sb.add_facet_field("tags", FacetOptions::default());
    sb.add_text_field("content", TEXT | STORED);
    sb.build()
}