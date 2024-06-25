use crate::indexed::Indexed;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct IndexedHashMap<K, V>(HashMap<K, V>);

impl<K: Eq + Hash, V> Indexed for IndexedHashMap<K, V> {
    type Index = K;
    type Output = V;

    fn get(&self, index: Self::Index) -> Option<&Self::Output> {
        self.0.get(&index)
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        self.0.get_mut(&index)
    }

    fn insert(&mut self, index: Self::Index, value: Self::Output) -> Option<Self::Output> {
        self.0.insert(index, value)
    }

    fn remove(&mut self, index: Self::Index) -> Option<Self::Output> {
        self.0.remove(&index)
    }

    fn clear(&mut self) {
        self.0.clear()
    }
}

impl<K, V> From<HashMap<K, V>> for IndexedHashMap<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}
