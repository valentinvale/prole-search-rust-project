use anyhow::{Result, Context};
use std::{fs, path::Path};

pub fn ensure_empty_dir(dir: &Path) -> Result<()> {
    if dir.exists() {

        if fs::metadata(dir)?.is_file() {
            anyhow::bail!("Path is a file, not a directory: {}", dir.display());
        }

        let mut entries = fs::read_dir(dir)
            .with_context(|| format!("Reading directory: {}", dir.display()))?;
        if entries.next().is_some() {
            anyhow::bail!("Directory is not empty: {}", dir.display());
        }
    }
    else{
        fs::create_dir_all(dir)
            .with_context(|| format!("Creating directory: {}", dir.display()))?;
        
    }
    Ok(())
}