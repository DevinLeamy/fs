use std::path::PathBuf;

/// Trait to be implemented for all model-specific code.
pub trait SentenceEmbeddingModel {
    /// Generates an embedding from a sentence.
    fn embed_sentence(&self, s: String) -> Vec<f32>;
}

pub trait FileEmbeddingModel {
    /// Generate an embedding from a file.
    fn embed_file(&self, path: PathBuf) -> Vec<f32>;
}
