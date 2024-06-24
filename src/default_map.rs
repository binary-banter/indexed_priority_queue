use crate::indexed::Indexed;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Default, Debug)]
pub struct DefaultMap<K, V>(HashMap<K, V>);

impl<K, V> Indexed for DefaultMap<K, V>
where
    K: Eq + Hash,
    V: Default,
    for<'a> &'a V: Default,
{
    type Index = K;
    type Output = V;

    fn get(&self, index: K) -> Option<&Self::Output> {
        Some(self.0.get(&index).unwrap_or_default())
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
}

impl<K, V> From<HashMap<K, V>> for DefaultMap<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}
