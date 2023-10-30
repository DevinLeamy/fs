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
        loader.load_files(fetch_files("/Users/Devin/Desktop/Github/DevinLeamy"))?;
    } else {
        // Handle commands.
        let input = InputHandler::parse(arguments);
        let engine = QueryEngineBuilder::new()
            .embedder(embedder)
            .database(database)
            .build()?;

        let mut fs = FileSystem::from_config(FileSystemConfig { engine });

        let response = fs.handle_input(input)?;
        println!("{:?}", response);
    }

    Ok(())
}

/// Fetch a list of files to load.
fn fetch_files(directory: impl AsRef<Path>) -> Vec<PathBuf> {
    let directory_path: &Path = directory.as_ref();
    if directory_path.ends_with(".git") {
        return vec![];
    }
    if directory_path.ends_with("build") {
        return vec![];
    }
    if directory_path.ends_with("target") {
        return vec![];
    }
    if directory_path.ends_with("node_modules") {
        return vec![];
    }
    if directory_path.ends_with("lib") {
        return vec![];
    }
    let mut paths = Vec::<PathBuf>::new();
    for path in std::fs::read_dir(directory).unwrap() {
        if let Ok(entry) = path {
            if entry.path().is_file() {
                paths.push(entry.path());
            } else if entry.path().is_dir() {
                paths.extend(fetch_files(entry.path()));
            }
        }
    }

    paths
}
