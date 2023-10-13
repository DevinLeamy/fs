use std::{collections::HashMap, path::PathBuf};

use crate::{
    prelude::*,
    shared::{utils::unsafe_path_string, Save},
};
use faiss::{index::NativeIndex, *};
use rust_bert::pipelines::sentence_embeddings::Embedding;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Item {
    File { path: PathBuf },
}

type ItemId = u64;

#[derive(Serialize, Deserialize)]
struct SerializedVectorDatabase(PathBuf, HashMap<u64, Item>);

/// Vector database backed by faiss-rs.
///
/// https://github.com/Enet4/faiss-rs
pub struct VectorDatabase<I: NativeIndex> {
    index: I,
    items: HashMap<ItemId, Item>,
}

impl<I: NativeIndex> VectorDatabase<I> {
    pub fn from_index(index: I) -> Self {
        Self {
            index,
            items: HashMap::new(),
        }
    }
}

impl<I: NativeIndex> VectorDatabase<I> {
    /// Query an item using an embedding.
    pub fn query(&mut self, embedding: Embedding) -> Item {
        let result = self.index.search(&embedding, 3).unwrap();
        println!("{:?}", result);
        let index = result.labels[0];
        let id = index.get().unwrap();

        self.items.get(&id).unwrap().clone()
    }

    /// Associate and embedding with an item.
    pub fn add(&mut self, embedding: Embedding, item: Item) {
        let item_id = self.generate_item_id();
        self.items.insert(item_id, item);
        // Here we're naively assuming that IDs are assigned sequentially, starting from 0.
        self.index.add(&embedding).unwrap();
    }

    /// Number of items in the database.
    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<I: NativeIndex> VectorDatabase<I> {
    /// Generate a new item id.
    fn generate_item_id(&self) -> ItemId {
        // Note: Not a long-term solution.
        self.items.len() as u64
    }

    /// Serialized index database path.
    fn index_path(&self) -> PathBuf {
        // Note: Eventually this will be some ~/.local/fs/db/index
        PathBuf::from("./assets/db/index")
    }
}

impl Serialize for VectorDatabase<IndexImpl> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let items = &self.items;
        let index_path = self.index_path();
        write_index(&self.index, unsafe_path_string(&index_path))
            .map_err(serde::ser::Error::custom)?;
        SerializedVectorDatabase(index_path, items.clone()).serialize(serializer)
    }
}

impl<'d> Deserialize<'d> for VectorDatabase<IndexImpl> {
    fn deserialize<D: Deserializer<'d>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let SerializedVectorDatabase(index_path, items) = Deserialize::deserialize(deserializer)?;
        let index = read_index(unsafe_path_string(&index_path)).map_err(de::Error::custom)?;
        Ok(Self { index, items })
    }
}

impl<'d> Save<'d> for VectorDatabase<IndexImpl> {
    fn path() -> PathBuf {
        // Note: Eventually this will be some ~/.local/fs/db/index
        PathBuf::from("./assets/db/index.json")
    }

    fn restore() -> Self {
        let file_contents = std::fs::read_to_string(Self::path()).unwrap();
        serde_json::from_str(&file_contents).expect("failed to read database")
    }

    fn save(&self) {
        let serialized_db = serde_json::to_string(self).unwrap();
        std::fs::write(Self::path(), serialized_db).expect("failed to save database");
    }
}
