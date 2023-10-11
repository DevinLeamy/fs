use std::path::PathBuf;

use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

use super::driver::*;

pub struct DefaultEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl DefaultEmbeddingModel {
    pub fn from_path(path: PathBuf) -> Self {
        todo!()
    }

    pub fn from_remote() -> Self {
        // TODO: Once an error type is added (e.g. using "thiserror"), this returns a result.
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .unwrap();

        Self { model }
    }
}

impl EmbeddingModel for DefaultEmbeddingModel {
    fn embed(&self, s: String) -> Vec<f32> {
        // TODO: Should return result.
        let sentences = [&s];
        let result = self.model.encode(&sentences).unwrap();
        result[0].clone()
    }
}
