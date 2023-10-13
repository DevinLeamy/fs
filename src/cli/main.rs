mod input;

use fs::{prelude::*, shared::Save};
use input::*;

fn main() -> Result<()> {
    let input = InputHandler::parse_arguments();
    let database = VectorDatabase::restore();
    // let index = index_factory(EMBEDDING_SIZE, "Flat", MetricType::L2).unwrap();
    // let database VectorDatabase::from_index(index);
    let mut engine = QueryEngineBuilder::new()
        .embedder(DefaultEmbeddingModel::from_remote()?)
        .database(database)
        .build()?;
    engine.load_test_files(fetch_files());

    let mut fs = FileSystem::from_config(FileSystemConfig { engine });

    let response = fs.handle_input(input)?;
    println!("File[{:?}]", response);

    fs.save();
    Ok(())
}

/// Fetch a list of files to load.
fn fetch_files() -> Vec<PathBuf> {
    let raw_paths: Vec<&str> = vec![
        // "/Users/Devin/.zshrc",
        // "/Users/Devin/Desktop/Github/DevinLeamy/fs/src/cli/main.rs",
        // "/Users/Devin/Desktop/Carina Nebula.png",
    ];
    raw_paths
        .into_iter()
        .map(|path| PathBuf::from(path))
        .collect()
}
