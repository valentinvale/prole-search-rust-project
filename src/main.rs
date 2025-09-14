use anyhow::Result;
use clap::{Parser, Subcommand};


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
    InitIndex {
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
        }
    }

    Ok(())

}
