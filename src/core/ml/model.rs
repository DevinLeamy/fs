use std::path::PathBuf;

use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

use crate::prelude::*;

pub const EMBEDDING_SIZE: u32 = 384;

pub struct DefaultEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl DefaultEmbeddingModel {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        todo!()
    }

    pub fn from_remote() -> Result<Self> {
        // TODO: Once an error type is added (e.g. using "thiserror"), this returns a result.
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .map_err(|_| FSError::Uncategorized {
                message: "failed to create remote model",
            })?;

        Ok(Self { model })
    }
}

impl SentenceEmbeddingModel for DefaultEmbeddingModel {
    fn embed_sentence(&self, s: String) -> Vec<f32> {
        // TODO: Should return result.
        let sentences = [&s];
        let result = self.model.encode(&sentences).unwrap();
        result[0].clone()
    }
}

impl FileEmbeddingModel for DefaultEmbeddingModel {
    fn embed_file(&self, path: PathBuf) -> Vec<f32> {
        self.embed_sentence(path.to_str().unwrap().to_string())
    }
}
