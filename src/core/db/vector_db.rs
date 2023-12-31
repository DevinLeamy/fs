use std::{collections::HashMap, path::PathBuf};

use crate::{
    prelude::*,
    shared::{utils::unsafe_path_string, Save},
};
use faiss::{index::NativeIndex, *};
use rust_bert::pipelines::sentence_embeddings::Embedding;
use std::fmt;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Item {
    File { path: PathBuf },
}

type ItemId = u64;

#[derive(Serialize, Deserialize)]
struct SerializedVectorDatabase(PathBuf, HashMap<u64, Item>);

struct SearchResult {
    items: Vec<Item>,
    distances: Vec<f32>,
}

impl SearchResult {
    fn len(&self) -> usize {
        self.items.len()
    }

    fn lowest_distance_item(&self) -> Option<Item> {
        self.items.first().cloned()
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.len() {
            writeln!(f, "[{:.2?}] {:?} ", self.distances[i], self.items[i])?;
        }

        Ok(())
    }
}

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
    pub fn query(&mut self, embedding: Embedding) -> Result<Item> {
        let result = self.query_k(&embedding, 5)?;
        let Some(closest) = result.lowest_distance_item() else {
            return Err(FSError::from_str("no results from query"));
        };
        println!("{result}");
        Ok(closest)
    }

    /// Associate and embedding with an item.
    pub fn add(&mut self, embedding: Embedding, item: Item) -> Result<()> {
        if self.contains(&embedding)? {
            println!("VectorDatabase::add - duplicate embedding ignored");
            return Ok(());
        }
        let item_id = self.generate_item_id();
        self.items.insert(item_id, item);

        // Here we're naively assuming that IDs are assigned sequentially, starting from 0.
        self.index
            .add(&embedding)
            .map_err(|_| FSError::from_str("failed to add new embedding to vector store"))?;

        Ok(())
    }

    /// Number of items in the database.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if the database already contains an embedding.
    pub fn contains(&mut self, embedding: &Embedding) -> Result<bool> {
        let result = self.query_k(embedding, 1)?;
        Ok(result.len() > 0 && result.distances[0] == 0.0)
    }
}

impl<I: NativeIndex> VectorDatabase<I> {
    /// Query the index for up to `k` results.
    fn query_k(&mut self, embedding: &Embedding, k: usize) -> Result<SearchResult> {
        let raw_result = self
            .index
            .search(embedding, k)
            .map_err(|_| FSError::from_str("failed to query vector store"))?;

        let mut items = Vec::<Item>::new();
        let mut distances = Vec::<f32>::new();
        for (i, index) in raw_result.labels.iter().enumerate() {
            let Some(id) = index.get() else {
                // ith entry was found.
                continue;
            };
            if !self.items.contains_key(&id) {
                // The given embedding has not been stored.
                continue;
            }
            items.push(self.items.get(&id).unwrap().clone());
            distances.push(raw_result.distances[i])
        }

        Ok(SearchResult { items, distances })
    }

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
