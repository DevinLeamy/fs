use clap::*;
use fs::prelude::*;

#[derive(Parser, Debug)]
enum Command {
    Open(Open),
    Edit(Edit),
    Goto(Goto),
}

#[derive(Parser, Debug)]
struct Open {
    text: String,
}

#[derive(Parser, Debug)]
struct Edit {
    text: String,
}

#[derive(Parser, Debug)]
struct Goto {
    text: String,
}

#[derive(Parser, Debug)]
pub struct Arguments {
    #[command(subcommand)]
    subcommand: Command,
}

impl Into<Input> for Arguments {
    fn into(self) -> Input {
        // This conversion from Arguments to Input to pretty useless now,
        // but the idea that when commands come with options, or there are
        // commands that aren't inputs (e.g. check the version), you don't
        // have to modify Input.
        match self.subcommand {
            Command::Open(Open { text }) => Input::Open(OpenCommand { text }),
            Command::Edit(Edit { text }) => Input::Edit(EditCommand { text }),
            Command::Goto(Goto { text }) => Input::Goto(GotoCommand { text }),
        }
    }
}

pub struct InputHandler {}

impl InputHandler {
    /// Generates a well formed input from command line input.
    pub fn parse_arguments() -> Input {
        Arguments::parse().into()
    }
}
