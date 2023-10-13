use crate::prelude::*;

pub struct QueryEngine<I: NativeIndex> {
    // TODO: This is to be replaced by a trait bound once the right way to define the trait is clear.
    embedder: DefaultEmbeddingModel,
    database: VectorDatabase<I>,
}

impl<I: NativeIndex> QueryEngine<I> {
    pub fn search(&mut self, query: String) -> Result<Item> {
        let embedding = self.embedder.embed_sentence(query)?;
        Ok(self.database.query(embedding))
    }
}

pub struct QueryEngineBuilder<I: NativeIndex> {
    embedder: Option<DefaultEmbeddingModel>,
    database: Option<VectorDatabase<I>>,
}

impl<I: NativeIndex> Default for QueryEngineBuilder<I> {
    fn default() -> Self {
        Self::new()
    }
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
