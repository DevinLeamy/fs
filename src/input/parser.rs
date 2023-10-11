use crate::core::*;
use clap::*;

#[derive(Parser)]
enum Command {
    Open(Open),
    Edit(Edit),
    Goto(Goto),
}

#[derive(Parser)]
struct Open {}

#[derive(Parser)]
struct Edit {}

#[derive(Parser)]
struct Goto {}

#[derive(Parser)]
pub struct Arguments {
    #[command(subcommand)]
    subcommand: Command,
}

impl Into<Input> for Arguments {
    fn into(self) -> Input {
        // todo!()
        Input::Open(OpenCommand {
            text: String::from(""),
        })
    }
}

pub struct InputHandler {}

impl InputHandler {
    /// Generates a well formed input from command line input.
    pub fn parse_arguments() -> Input {
        Arguments::parse().into()
    }
}
