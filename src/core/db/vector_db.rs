use std::{collections::HashMap, path::PathBuf};

use faiss::{index::NativeIndex, *};
use rust_bert::pipelines::sentence_embeddings::Embedding;

#[derive(Clone, Debug)]
pub enum Item {
    File { path: PathBuf },
}

type ItemId = u64;

/// Vector database backed by faiss-rs.
///
/// https://github.com/Enet4/faiss-rs
pub struct VectorDatabase<I: NativeIndex> {
    index: IdMap<I>,
    items: HashMap<ItemId, Item>,
}

impl<I: NativeIndex> VectorDatabase<I> {
    pub fn from_index(index: I) -> Self {
        Self {
            index: IdMap::new(index).unwrap(),
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
        self.index
            .add_with_ids(&embedding, &[Idx::new(item_id)])
            .unwrap();
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
}
