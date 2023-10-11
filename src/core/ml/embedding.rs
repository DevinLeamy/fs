use std::path::PathBuf;

use rust_bert::pipelines::sentence_embeddings::Embedding;

pub trait SentenceEmbeddingModel {
    /// Generates an embedding from a sentence.
    fn embed_sentence(&self, s: String) -> Embedding;
}

pub trait FileEmbeddingModel {
    /// Generate an embedding from a file.
    fn embed_file(&self, path: PathBuf) -> Embedding;
}
