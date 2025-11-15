use std::{fs, path::Path};
use anyhow::{Result, Context};
use chardetng::EncodingDetector;

// Function for reading a text file
pub fn read_txt(path: &Path) -> Result<String> {
    // Reading the file bytes
    let bytes = fs::read(path)
        .with_context(|| format!("Reading text file {}", path.display()))?;
    
    // Detecting encoding using chardetng
    let mut detector = EncodingDetector::new();
    detector.feed(&bytes, true);
    let encoding = detector.guess(None, true);

    // Decoding bytes to string using detected encoding
    let (content_cow, _, _) = encoding.decode(&bytes);

    Ok(content_cow.into_owned())
}