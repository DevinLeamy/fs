use std::path::PathBuf;

use rust_bert::pipelines::sentence_embeddings::Embedding;

pub enum Item {
    File { path: PathBuf },
}

/// Vector database backed by faiss-rs
pub struct VectorDatabase {}

impl VectorDatabase {
    /// Query an item using an embedding.
    pub fn query(&self, embedding: Embedding) -> Item {
        todo!()
    }

    /// Associate and embedding with an item.
    pub fn add(&self, embedding: Embedding, item: Item) {
        todo!()
    }
}
