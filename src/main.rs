mod core;
mod input;

use crate::core::*;
use crate::input::*;

fn main() {
    let input = InputHandler::parse_arguments();
    let mut fs = FileSystem::from_config(FileSystemConfig::default());
    let response = fs.handle_input(input);

    println!("{:?}", response);
}
