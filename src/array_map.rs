use crate::indexed::Indexed;
use std::mem;

#[derive(Debug)]
pub struct ArrayPriorityMap<Priority, const N: usize = 0>(Box<[Priority]>);

#[derive(Debug)]
pub struct ArrayPositionMap<const N: usize = 0>(Box<[usize]>);

impl<Priority: Clone, const OFFSET: usize> Indexed for ArrayPriorityMap<Priority, OFFSET> {
    type Index = usize;
    type Output = Priority;

    fn get(&self, index: Self::Index) -> Option<&Self::Output> {
        let index = index - OFFSET;

        Some(&self.0[index])
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        let index = index - OFFSET;

        Some(&mut self.0[index])
    }

    fn insert(&mut self, index: Self::Index, mut value: Self::Output) -> Option<Self::Output> {
        let index = index - OFFSET;

        let old_value = &mut self.0[index];
        mem::swap(old_value, &mut value);
        Some(value)
    }

    fn remove(&mut self, index: Self::Index) -> Option<Self::Output> {
        let index = index - OFFSET;

        Some(self.0[index].clone())
    }

    fn clear(&mut self) {}

    fn iter(&mut self) -> impl Iterator<Item = &Self::Output> {
        self.0.iter()
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Output> {
        self.0.iter_mut()
    }
}

impl<const OFFSET: usize> Indexed for ArrayPositionMap<OFFSET> {
    type Index = usize;
    type Output = usize;

    fn get(&self, index: Self::Index) -> Option<&Self::Output> {
        let index = index - OFFSET;

        match &self.0[index] {
            &usize::MAX => None,
            position => Some(position),
        }
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        let index = index - OFFSET;

        match &mut self.0[index] {
            &mut usize::MAX => None,
            position => Some(position),
        }
    }

    fn insert(&mut self, index: Self::Index, value: Self::Output) -> Option<Self::Output> {
        let index = index - OFFSET;

        match mem::replace(&mut self.0[index], value) {
            usize::MAX => None,
            position => Some(position),
        }
    }

    fn remove(&mut self, index: Self::Index) -> Option<Self::Output> {
        let index = index - OFFSET;

        match mem::replace(&mut self.0[index], usize::MAX) {
            usize::MAX => None,
            position => Some(position),
        }
    }

    fn clear(&mut self) {
        self.0.fill(usize::MAX);
    }

    fn iter(&mut self) -> impl Iterator<Item = &Self::Output> {
        self.0.iter().filter(|v| **v != usize::MAX)
    }

    fn iter_mut(&mut self) -> impl Iterator<Item = &mut Self::Output> {
        self.0.iter_mut().filter(|v| **v != usize::MAX)
    }
}

impl<Priority, const OFFSET: usize> From<Box<[Priority]>> for ArrayPriorityMap<Priority, OFFSET> {
    fn from(value: Box<[Priority]>) -> Self {
        Self(value)
    }
}

impl<const OFFSET: usize> From<Box<[usize]>> for ArrayPositionMap<OFFSET> {
    fn from(value: Box<[usize]>) -> Self {
        Self(value)
    }
}
