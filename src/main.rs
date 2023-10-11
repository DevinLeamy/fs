mod core;
mod input;

use std::path::PathBuf;

use crate::core::*;
use crate::input::*;

fn main() {
    let input = InputHandler::parse_arguments();
    let mut fs = FileSystem::from_config(FileSystemConfig::default());
    let mut database = VectorDatabase::empty();
    let embedder = DefaultEmbeddingModel::from_remote();
    load_files(&mut database, &embedder);

    let response = fs.handle_input(input);

    println!("{:?}", response);

    println!(
        "{:?}",
        embedder.embed_sentence(String::from("Home directory"))
    );
    println!(
        "{:?}",
        embedder.embed_file(PathBuf::from("~/Desktop/IMG_3390.jpeg"))
    );
}

/// Temporary helper to manually load files into the vector store, before we setup a better
/// solution.
fn load_files(database: &mut VectorDatabase, embedder: &DefaultEmbeddingModel) {
    for file_path in fetch_files() {
        if !file_path.is_file() {
            println!("{:?} is not a file", file_path);
            continue;
        }
        load_file(database, embedder, file_path);
    }
}
fn load_file(database: &mut VectorDatabase, embedder: &DefaultEmbeddingModel, path: PathBuf) {
    let embedding = embedder.embed_file(path.clone());
    // database.add(embedding, Item::File { path });
}

/// Fetch a list of files to load.
fn fetch_files() -> Vec<PathBuf> {
    let raw_paths = vec![
        "/Users/Devin/.zshrc",
        "/Users/Devin/Desktop/Github/DevinLeamy/fs/src/main.rs",
        "/Users/Devin/Desktop/Carina Nebula.png",
    ];
    raw_paths
        .into_iter()
        .map(|path| PathBuf::from(path))
        .collect()
}
