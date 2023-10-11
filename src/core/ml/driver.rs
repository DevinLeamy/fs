/// Trait to be implemented for all model-specific code.
pub trait EmbeddingModel {
    /// Generates an embedding from a sentence.
    fn embed(&self, s: String) -> Vec<f32>;
}
