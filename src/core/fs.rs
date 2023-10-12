use crate::prelude::*;

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

pub struct FileSystemConfig<I: NativeIndex> {
    pub query_engine: QueryEngine<I>,
}

pub struct FileSystem<I: NativeIndex> {
    query_engine: QueryEngine<I>,
}

impl<I: NativeIndex> FileSystem<I> {
    pub fn from_config(config: FileSystemConfig<I>) -> Self {
        Self {
            query_engine: config.query_engine,
        }
    }
}

impl<I: NativeIndex> FileSystem<I> {
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

