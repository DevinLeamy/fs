use crate::{prelude::*, shared::Save};

pub struct DatabaseLoader<I: NativeIndex> {
    database: VectorDatabase<I>,
    embedder: DefaultEmbeddingModel,
}

impl<I: NativeIndex> DatabaseLoader<I> {
    pub fn new(database: VectorDatabase<I>, embedder: DefaultEmbeddingModel) -> Self {
        Self { database, embedder }
    }
}

impl DatabaseLoader<IndexImpl> {
    pub fn load_files(&mut self, files: Vec<PathBuf>) {
        for file_path in files {
            let embedding = self.embedder.embed_file(file_path.clone()).unwrap();
            self.database.add(embedding, Item::File { path: file_path });
        }
        self.save();
    }

    pub fn load_file(&mut self, file_path: PathBuf) {
        let embedding = self.embedder.embed_file(file_path.clone()).unwrap();
        self.database.add(embedding, Item::File { path: file_path });
        self.save();
    }
}

impl DatabaseLoader<IndexImpl> {
    fn save(&self) {
        self.database.save();
    }
}
