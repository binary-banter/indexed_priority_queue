pub trait Indexed {
    type Index;
    type Output;

    fn get(&self, index: Self::Index) -> Option<&Self::Output>;
    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output>;
    fn insert(&mut self, index: Self::Index, value: Self::Output) -> Option<Self::Output>;
    fn remove(&mut self, index: Self::Index) -> Option<Self::Output>;
    fn clear(&mut self);

    fn contains(&self, index: Self::Index) -> bool {
        self.get(index).is_some()
    }

    fn index(&self, index: Self::Index) -> &Self::Output {
        self.get(index).unwrap()
    }

    fn index_mut(&mut self, index: Self::Index) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
