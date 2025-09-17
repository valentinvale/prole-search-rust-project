use anyhow::Result;
use clap::{Command, Parser, Subcommand}; // maybe use Command later for more customization


mod utils;
mod search;
mod cmd;


#[derive(Parser)]
#[command(name = "mls", version, about = "Marxist Literature Search - Prole Search")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Initialize a new empty index
    #[command(about = "Initialize a new empty Tantivy index (default directory: ./index)")]
    InitIndex {
        #[arg(default_value = "index")]
        index_dir: std::path::PathBuf,
    },

    // Ingest documents from a directory into the index
    #[command(about = "Ingest documents from a directory into the index (default directory: ./corpus, default index directory: ./index)")]
    Index {
        #[arg(default_value = "corpus")]
        corpus_dir: std::path::PathBuf,

        #[arg(default_value = "index")]
        index_dir: std::path::PathBuf,
    }
}


fn main() -> Result<()> {
    utils::logging::init();

    let cli = Cli::parse();
    match cli.cmd {
        Commands::InitIndex { index_dir } => {
            cmd::init::run(&index_dir)?;
        },
        Commands::Index { corpus_dir, index_dir } => {
            cmd::index::run(&corpus_dir, &index_dir)?;
        }
    }

    Ok(())

}
