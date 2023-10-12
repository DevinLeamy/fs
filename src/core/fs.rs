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
    engine: QueryEngine<I>,
}

impl<I: NativeIndex> FileSystem<I> {
    pub fn from_config(config: FileSystemConfig<I>) -> Self {
        Self {
            engine: config.query_engine,
        }
    }
}

impl<I: NativeIndex> FileSystem<I> {
    pub fn handle_input(&mut self, input: Input) -> Result<String> {
        match input {
            Input::Edit(cmd) => self.edit(cmd),
            Input::Open(cmd) => self.open(cmd),
            Input::Goto(cmd) => self.goto(cmd),
        }
    }

    pub fn edit(&mut self, command: EditCommand) -> Result<String> {
        todo!()
    }

    pub fn open(&mut self, command: OpenCommand) -> Result<String> {
        todo!()
    }

    pub fn goto(&mut self, command: GotoCommand) -> Result<String> {
        let item = self.engine.search(command.text)?;
        match item {
            Item::File { path } => Ok(path.to_str().unwrap().to_string()),
            _ => Err(FSError::from_str("query did not return a file path")),
        }
    }
}
