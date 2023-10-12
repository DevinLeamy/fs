use std::path::PathBuf;

use crate::prelude::*;
use rust_bert::pipelines::sentence_embeddings::Embedding;

pub trait SentenceEmbeddingModel {
    /// Generates an embedding from a sentence.
    fn embed_sentence(&self, s: String) -> Result<Embedding>;
}

pub trait FileEmbeddingModel {
    /// Generate an embedding from a file.
    fn embed_file(&self, path: PathBuf) -> Result<Embedding>;
}
