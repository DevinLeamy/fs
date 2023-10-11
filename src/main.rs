mod core;
mod input;

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
        embedding_model.embed(String::from("Home directory"))
    );
}
