use clap::*;
use fs::prelude::*;

#[derive(Parser, Debug)]
pub enum Command {
    Open(Open),
    Edit(Edit),
    Goto(Goto),
    Load,
}

#[derive(Parser, Debug)]
pub struct Open {
    text: String,
}

#[derive(Parser, Debug)]
pub struct Edit {
    text: String,
}

#[derive(Parser, Debug)]
pub struct Goto {
    text: String,
}

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    pub subcommand: Command,
}

impl From<Arguments> for Input {
    fn from(val: Arguments) -> Self {
        // This conversion from Arguments to Input to pretty useless now,
        // but the idea that when commands come with options, or there are
        // commands that aren't inputs (e.g. check the version), you don't
        // have to modify Input.
        match val.subcommand {
            Command::Open(Open { text }) => Input::Open(OpenCommand { text }),
            Command::Edit(Edit { text }) => Input::Edit(EditCommand { text }),
            Command::Goto(Goto { text }) => Input::Goto(GotoCommand { text }),
            _ => panic!("invalid option"),
        }
    }
}

pub struct InputHandler {}

impl InputHandler {
    /// Generates a well formed input from command line input.
    pub fn parse(arguments: Arguments) -> Input {
        arguments.into()
    }
}
