mod array_map;
mod default_map;
mod hash_map;
pub mod indexed;

use crate::array_map::{ArrayPositionMap, ArrayPriorityMap};
use crate::default_map::DefaultMap;
use crate::hash_map::IndexedHashMap;
use crate::indexed::Indexed;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

pub type HashMapIPQ<Index, Priority> =
    IndexedPriorityQueue<Index, IndexedHashMap<Index, Priority>, IndexedHashMap<Index, usize>>;

pub type DefaultMapIPQ<Index, Priority> =
    IndexedPriorityQueue<Index, DefaultMap<Index, Priority>, IndexedHashMap<Index, usize>>;

pub type ArrayMapIPQ<Priority> =
    IndexedPriorityQueue<usize, ArrayPriorityMap<Priority>, ArrayPositionMap>;

/// Indexed Priority Queue.
pub struct IndexedPriorityQueue<Index, Priorities, Positions>
where
    Index: Copy,
    Priorities: Indexed<Index = Index, Output: Ord + Clone>,
    Positions: Indexed<Index = Index, Output = usize>,
{
    /// The priorities associated with indexes on the heap.
    priorities: Priorities,
    /// The positions associated with indexes on the heap.
    positions: Positions,
    /// The underlying vec storing the indexes in order of priority.
    heap: Vec<Index>,
}

impl<Index, Priorities, Positions> IndexedPriorityQueue<Index, Priorities, Positions>
where
    Index: Copy,
    Priorities: Indexed<Index = Index, Output: Ord + Clone>,
    Positions: Indexed<Index = Index, Output = usize>,
{
    /// Constructs a new, empty `IndexedPriorityQueue`.
    pub fn new(priorities: impl Into<Priorities>, positions: impl Into<Positions>) -> Self {
        Self::with_capacity(priorities.into(), positions.into(), 0)
    }

    /// Constructs a new, empty `IndexedPriorityQueue` with at least the specified capacity.
    pub fn with_capacity(
        priorities: impl Into<Priorities>,
        positions: impl Into<Positions>,
        capacity: usize,
    ) -> Self {
        Self {
            priorities: priorities.into(),
            positions: positions.into(),
            heap: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of indices in the indexed priority queue.
    ///
    /// Time complexity: `O(1)`
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Returns `true` if the priority queue contains no indices.
    ///
    /// Time complexity: `O(1)`
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns `true` if the priority queue contains the specified index.
    ///
    /// Time complexity: `O(1)`
    pub fn contains(&self, index: Index) -> bool {
        self.positions.contains(index)
    }

    /// Returns the smallest priority in the queue, or `None`, if it is empty.
    ///
    /// Time complexity: `O(1)`
    pub fn min_priority(&self) -> Option<&Priorities::Output> {
        self.heap
            .first()
            .and_then(|index| self.priorities.get(*index))
    }

    /// Returns the priority associated with the specified index, or `None`, if the index is not in the queue.
    ///
    /// Time complexity: `O(1)`
    pub fn get_priority(&self, index: Index) -> Option<&Priorities::Output> {
        self.priorities.get(index)
    }

    /// Reinserts a previously removed index into the queue with its last associated value.
    ///
    /// Time complexity: `O(log n)`
    pub fn restore_index(&mut self, index: Index) {
        if !self.positions.contains(index) {
            let position = self.len();
            self.heap.push(index);
            self.positions.insert(index, position);
            self.up_heap(position);
        }
    }

    /// Removes the specified index and its associated priority from the queue.
    /// This method should not be called with indices that are not present in the queue.
    /// Returns the removed priority.
    ///
    /// Time complexity: `O(log n)`
    pub fn remove(&mut self, index: Index) -> Priorities::Output {
        self.remove_index(index);
        self.priorities.remove(index).unwrap()
    }

    /// Removes the specified index from the queue, retaining its associated priority.
    /// This method should not be called with indices that are not present in the queue.
    ///
    /// Time complexity: `O(log n)`
    pub fn remove_index(&mut self, index: Index) {
        let position = self
            .positions
            .remove(index)
            .expect("Index was not present in the queue.");
        self.heap.swap_remove(position);
        if position < self.len() {
            *self.positions.index_mut(self.heap[position]) = position;
            self.down_heap(position);
        }
    }

    /// Clears all indices and their priorities from the queue.
    pub fn clear(&mut self) {
        self.clear_indices();
        self.priorities.clear();
    }

    /// Clears all indices from the queue.
    pub fn clear_indices(&mut self) {
        self.positions.clear();
        self.heap.clear();
    }

    /// Inserts an index-priority pair into the priority queue.
    /// Returns the previous priority associated with the index, if it existed.
    ///
    /// Time complexity: `O(log n)`
    pub fn push(&mut self, index: Index, value: Priorities::Output) -> Option<Priorities::Output> {
        let old_priority = self.priorities.insert(index, value);
        self.restore_index(index);
        old_priority
    }

    /// Returns the index associated with the smallest priority in the queue, or `None` if it is empty.
    ///
    /// Time complexity: `O(1)`
    pub fn min(&self) -> Option<&Index> {
        self.heap.first()
    }

    /// Removes and returns the index associated with the smallest priority in the queue, or `None` if it is empty.
    ///
    /// Time complexity: `O(log n)`
    pub fn pop(&mut self) -> Option<Index> {
        if self.is_empty() {
            return None;
        }

        // Removes the index associated with the smallest priority in the queue.
        let popped_index = self.heap.swap_remove(0);

        // Update positions.
        self.positions.remove(popped_index);

        // Update the position for the new root and restore the heap property.
        if let Some(new_root) = self.heap.first() {
            *self.positions.index_mut(*new_root) = 0;
            self.down_heap(0);
        }

        Some(popped_index)
    }

    /// Returns the index of the parent node in the heap for the given index `n`.
    fn parent(n: usize) -> usize {
        (n - 1) / 2
    }

    /// Returns the index of the left child node in the heap for the given index `n`.
    fn left(&self, n: usize) -> Option<usize> {
        let index = 2 * n + 1;
        (index < self.len()).then_some(index)
    }

    /// Returns the index of the right child node in the heap for the given index `n`.
    fn right(&self, n: usize) -> Option<usize> {
        let index = 2 * n + 2;
        (index < self.len()).then_some(index)
    }

    /// Swaps the positions of two nodes in the heap by their indices.
    fn swap(&mut self, n: usize, m: usize) {
        *self.positions.index_mut(self.heap[n]) = m;
        *self.positions.index_mut(self.heap[m]) = n;
        self.heap.swap(n, m);
    }

    /// Compares the priorities of two nodes in the heap.
    fn compare(&self, n: usize, m: usize) -> Ordering {
        self.priorities
            .index(self.heap[n])
            .cmp(self.priorities.index(self.heap[m]))
    }

    /// Performs up-heap bubbling from the given heap index.
    fn up_heap(&mut self, mut n: usize) {
        while n > 0 {
            let parent = Self::parent(n);

            if self.compare(parent, n).is_gt() {
                self.swap(parent, n);
                n = parent;
            } else {
                break;
            }
        }
    }

    /// Performs down-heap bubbling from the given heap index.
    fn down_heap(&mut self, mut n: usize) {
        while let Some(left_index) = self.left(n) {
            let smallest_child_index = match self.right(n) {
                None => left_index,
                Some(right_index) => match self.compare(left_index, right_index) {
                    Ordering::Less | Ordering::Equal => left_index,
                    Ordering::Greater => right_index,
                },
            };

            if self.compare(smallest_child_index, n).is_lt() {
                self.swap(smallest_child_index, n);
                n = smallest_child_index;
            } else {
                break;
            }
        }
    }
}

impl<Index, Priorities, Positions> Default for IndexedPriorityQueue<Index, Priorities, Positions>
where
    Index: Copy,
    Priorities: Indexed<Index = Index, Output: Ord + Clone> + Default,
    Positions: Indexed<Index = Index, Output = usize> + Default,
{
    fn default() -> Self {
        Self::new(Priorities::default(), Positions::default())
    }
}

macro_rules! generate_get_mut {
    ($struct_name:ident, $function_name:ident $(, $cfg_condition:tt)*) => {
        impl<Index, Priorities, Positions> IndexedPriorityQueue<Index, Priorities, Positions>
        where
            Index: Copy,
            Priorities: Indexed<Index=Index, Output: Ord + Clone>,
            Positions: Indexed<Index=Index, Output = usize>,
        {
            pub fn $function_name(&mut self, index: Index) -> $struct_name<Index, Priorities, Positions> {
                $struct_name {
                    $(#[cfg($cfg_condition)])*
                    old_value: self.priorities.index(index).clone(),
                    heap: self,
                    index
                }
            }
        }

        pub struct $struct_name<'a, Index, Priorities, Positions>
        where
            Index: Copy,
            Priorities: Indexed<Index=Index, Output: Ord + Clone>,
            Positions: Indexed<Index=Index, Output = usize>,
        {
            $(#[cfg($cfg_condition)])*
            old_value: Priorities::Output,
            heap: &'a mut IndexedPriorityQueue<Index, Priorities, Positions>,
            index: Index,
        }

        impl<'a, Index, Priorities, Positions> Deref for $struct_name<'a, Index, Priorities, Positions>
        where
            Index: Copy,
            Priorities: Indexed<Index=Index, Output: Ord + Clone>,
            Positions: Indexed<Index=Index, Output = usize>,
        {
            type Target = Priorities::Output;

            fn deref(&self) -> &Self::Target {
                self.heap.priorities.index(self.index)
            }
        }

        impl<'a, Index, Priorities, Positions> DerefMut for $struct_name<'a, Index, Priorities, Positions>
        where
            Index: Copy,
            Priorities: Indexed<Index=Index, Output: Ord + Clone>,
            Positions: Indexed<Index=Index, Output = usize>,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.heap.priorities.index_mut(self.index)
            }
        }
    };
}

generate_get_mut!(IPQMutRefUp, update_up, debug_assertions);
generate_get_mut!(IPQMutRefDown, update_down, debug_assertions);
generate_get_mut!(IPQMutRefDyn, update_dyn);

impl<'a, Index, Priorities, Positions> Drop for IPQMutRefUp<'a, Index, Priorities, Positions>
where
    Index: Copy,
    Priorities: Indexed<Index = Index, Output: Ord + Clone>,
    Positions: Indexed<Index = Index, Output = usize>,
{
    fn drop(&mut self) {
        debug_assert!(*self.heap.priorities.index(self.index) >= self.old_value);
        if let Some(position) = self.heap.positions.get(self.index) {
            self.heap.down_heap(*position);
        }
    }
}

impl<'a, Index, Priorities, Positions> Drop for IPQMutRefDown<'a, Index, Priorities, Positions>
where
    Index: Copy,
    Priorities: Indexed<Index = Index, Output: Ord + Clone>,
    Positions: Indexed<Index = Index, Output = usize>,
{
    fn drop(&mut self) {
        debug_assert!(*self.heap.priorities.index(self.index) <= self.old_value);
        if let Some(position) = self.heap.positions.get(self.index) {
            self.heap.up_heap(*position);
        }
    }
}

impl<'a, Index, Priorities, Positions> Drop for IPQMutRefDyn<'a, Index, Priorities, Positions>
where
    Index: Copy,
    Priorities: Indexed<Index = Index, Output: Ord + Clone>,
    Positions: Indexed<Index = Index, Output = usize>,
{
    fn drop(&mut self) {
        if let Some(position) = self.heap.positions.get(self.index) {
            match self.heap.priorities.index(self.index).cmp(&self.old_value) {
                Ordering::Greater => self.heap.down_heap(*position),
                Ordering::Less => self.heap.up_heap(*position),
                Ordering::Equal => {}
            }
        }
    }
}
