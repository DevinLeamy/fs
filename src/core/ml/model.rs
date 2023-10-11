use std::path::PathBuf;

use super::driver::*;

/// Initial LLM driver.
pub struct DefactoEmbeddingModel {}

impl DefactoEmbeddingModel {
    fn from_path(path: PathBuf) -> Self {
        Self {}
    }
}

impl EmbeddingModel for DefactoEmbeddingModel {
    fn embed(&self, s: String) -> &[f32] {
        todo!()
    }
}
