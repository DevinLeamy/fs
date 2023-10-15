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
    pub fn load_files(&mut self, files: Vec<PathBuf>) -> Result<()> {
        for file_path in files {
            println!("Loading file: {:?}", file_path);
            let embedding = self.embedder.embed_file(file_path.clone())?;
            self.database
                .add(embedding, Item::File { path: file_path })?;
        }
        self.save();

        Ok(())
    }

    pub fn load_file(&mut self, file_path: PathBuf) -> Result<()> {
        let embedding = self.embedder.embed_file(file_path.clone())?;
        self.database
            .add(embedding, Item::File { path: file_path })?;
        self.save();

        Ok(())
    }
}

impl DatabaseLoader<IndexImpl> {
    fn save(&self) {
        self.database.save();
    }
}
