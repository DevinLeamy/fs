mod core;
mod input;

use std::path::PathBuf;

use crate::core::*;
use crate::input::*;

fn main() {
    let input = InputHandler::parse_arguments();
    let mut fs = FileSystem::from_config(FileSystemConfig::default());
    let response = fs.handle_input(input);

    println!("{:?}", response);

    let embedding_model = DefaultEmbeddingModel::from_remote();
    println!(
        "{:?}",
        embedding_model.embed_sentence(String::from("Home directory"))
    );
    println!(
        "{:?}",
        embedding_model.embed_file(PathBuf::from("~/Desktop/IMG_3390.jpeg"))
    );
}
