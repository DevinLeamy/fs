use crate::prelude::*;

pub struct QueryEngine<I: NativeIndex> {
    // TODO: This is to be replaced by a trait bound once the right way to define the trait is clear.
    embedder: DefaultEmbeddingModel,
    database: VectorDatabase<I>,
}

impl<I: NativeIndex> QueryEngine<I> {
    /// Temporary helper to manually load files into the vector store, before we setup a better
    /// solution.
    pub fn load_test_files(&mut self, paths: Vec<PathBuf>) {
        for file_path in paths {
            if !file_path.is_file() {
                println!("{:?} is not a file", file_path);
                continue;
            }
            let embedding = self.embedder.embed_file(file_path.clone()).unwrap();
            self.database.add(embedding, Item::File { path: file_path });
        }
    }

    pub fn search(&self, query: String) -> Result<Item> {
        todo!()
    }
}

pub struct QueryEngineBuilder<I: NativeIndex> {
    embedder: Option<DefaultEmbeddingModel>,
    database: Option<VectorDatabase<I>>,
}

impl<I: NativeIndex> QueryEngineBuilder<I> {
    pub fn new() -> Self {
        Self {
            embedder: None,
            database: None,
        }
    }

    pub fn embedder(mut self, embedder: DefaultEmbeddingModel) -> Self {
        self.embedder = Some(embedder);
        self
    }

    pub fn database(mut self, database: VectorDatabase<I>) -> Self {
        self.database = Some(database);
        self
    }

    pub fn build(self) -> Result<QueryEngine<I>> {
        if let (Some(embedder), Some(database)) = (self.embedder, self.database) {
            return Ok(QueryEngine { embedder, database });
        }

        Err(FSError::from_str(
            "internal error: query engine builder missing fields",
        ))
    }
}
