use super::ml::*;

pub enum Input {
    Open(OpenCommand),
    Edit(EditCommand),
    Goto(GotoCommand),
}

pub struct OpenCommand {
    pub text: String,
}

pub struct EditCommand {
    pub text: String,
}

pub struct GotoCommand {
    pub text: String,
}

#[derive(Default)]
pub struct FileSystemConfig {}

pub struct FileSystem;

impl FileSystem {
    pub fn init() -> Self {
        Self {}
    }

    pub fn from_config(config: FileSystemConfig) -> Self {
        Self {}
    }
}

impl FileSystem {
    pub fn handle_input(&mut self, input: Input) {
        match input {
            Input::Edit(cmd) => self.edit(cmd),
            Input::Open(cmd) => self.open(cmd),
            Input::Goto(cmd) => self.goto(cmd),
        };
    }

    pub fn edit(&mut self, command: EditCommand) {
        todo!()
    }

    pub fn open(&mut self, command: OpenCommand) {
        // todo!()
    }

    pub fn goto(&mut self, command: GotoCommand) {
        todo!()
    }
}
