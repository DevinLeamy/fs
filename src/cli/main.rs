mod input;

use clap::Parser;
use fs::{prelude::*, shared::Save};
use input::*;

fn main() -> Result<()> {
    let arguments = Arguments::parse();
    let database = if VectorDatabase::exists() {
        VectorDatabase::restore()
    } else {
        let index = index_factory(EMBEDDING_SIZE, "Flat", MetricType::L2).unwrap();
        VectorDatabase::from_index(index)
    };

    let embedder = DefaultEmbeddingModel::from_remote()?;
    if matches!(arguments.subcommand, Command::Load) {
        // Load files into the index.
        let mut loader = DatabaseLoader::new(database, embedder);
        loader.load_files(fetch_files())?;
    } else {
        // Handle commands.
        let input = InputHandler::parse(arguments);
        let engine = QueryEngineBuilder::new()
            .embedder(embedder)
            .database(database)
            .build()?;

        let mut fs = FileSystem::from_config(FileSystemConfig { engine });

        let response = fs.handle_input(input)?;
        println!("File[{:?}]", response);
    }

    Ok(())
}

/// Fetch a list of files to load.
fn fetch_files() -> Vec<PathBuf> {
    let raw_paths: Vec<&str> = vec![
        "/Users/Devin/.zshrc",
        "/Users/Devin/Desktop/Github/DevinLeamy/fs/src/cli/main.rs",
        "/Users/Devin/Desktop/Carina Nebula.png",
    ];
    raw_paths.into_iter().map(PathBuf::from).collect()
}
