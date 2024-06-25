use crate::indexed::Indexed;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct DefaultMap<K, V>(HashMap<K, V>, V);

impl<K: Eq + Hash, V: Default> Indexed for DefaultMap<K, V> {
    type Index = K;
    type Output = V;

    fn get(&self, index: K) -> Option<&Self::Output> {
        Some(self.0.get(&index).unwrap_or(&self.1))
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        Some(self.0.entry(index).or_default())
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

    fn iter(&mut self) -> impl Iterator<Item = &Self::Output> {
        self.0.values()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Output> {
        self.0.values_mut()
    }
}

impl<K, V: Default> From<HashMap<K, V>> for DefaultMap<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value, V::default())
    }
}
