use std::path::PathBuf;

use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};

use crate::{prelude::*, shared::unsafe_path_string};

pub const EMBEDDING_SIZE: u32 = 384;

pub struct DefaultEmbeddingModel {
    model: SentenceEmbeddingsModel,
}

impl DefaultEmbeddingModel {
    pub fn from_path(_path: PathBuf) -> Result<Self> {
        todo!()
    }

    pub fn from_remote() -> Result<Self> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .map_err(|_| FSError::from_str("failed to create remote model"))?;

        Ok(Self { model })
    }
}

impl SentenceEmbeddingModel for DefaultEmbeddingModel {
    fn embed_sentence(&self, s: String) -> Result<Vec<f32>> {
        let sentences = [&s];
        let result = self
            .model
            .encode(&sentences)
            .map_err(|_| FSError::from_str("failed to embed sentence"))?;
        Ok(result[0].clone())
    }
}

impl FileEmbeddingModel for DefaultEmbeddingModel {
    fn embed_file(&self, path: PathBuf) -> Result<Vec<f32>> {
        let file_sentence = format!(
            "{:?}\n{:?}",
            unsafe_path_string(&path),
            self.file_to_string(&path)
        );
        self.embed_sentence(file_sentence)
    }
}

impl DefaultEmbeddingModel {
    fn file_to_string(&self, path: &PathBuf) -> String {
        if let Ok(s) = std::fs::read_to_string(path) {
            s
        } else {
            String::from("")
        }
    }
}
