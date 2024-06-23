use std::cmp::Ordering;
use std::ops::{Deref, DerefMut, IndexMut, Not};

/// Indexed Priority Queue
///
/// I is the type of the indices.
/// VS is the datastructure that stores the values of each index.
/// PS is the datastructure that stores the positions of each index in the heap.
pub struct IndexedPriorityQueue<I, VS, PS> {
    /// The value for each index.
    value: VS,
    /// The position of the index on the heap.
    position: PS,
    /// The underlying vec storing the values in order.
    heap: Vec<I>,
}

impl<I, VS, PS> IndexedPriorityQueue<I, VS, PS>
where
    I: Copy,
    VS: IndexMut<I, Output: Ord + Sized + Clone>,
    PS: IndexMut<I, Output = Option<usize>>,
{
    pub fn new(value: VS, position: PS) -> Self {
        Self::with_capacity(value, position, 0)
    }

    pub fn with_capacity(value: VS, position: PS, capacity: usize) -> Self {
        Self {
            value,
            position,
            heap: Vec::with_capacity(capacity),
        }
    }

    /// Returns the size of the heap.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Checkk if the heap is empty
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Returns the index with the smallest value.
    pub fn min(&self) -> Option<I> {
        self.heap.is_empty().not().then_some(self.heap[0])
    }

    pub fn get(&self, index: I) -> &VS::Output {
        &self.value[index]
    }

    /// (Re-)store the given index on the heap.
    pub fn restore(&mut self, index: I) {
        if self.position[index].is_none() {
            let position = self.len();
            self.heap.push(index);
            self.position[index] = Some(position);
            self.up_heap(position);
        }
    }

    /// Remove the given index from the heap.
    /// Should not be called with indices off the heap.
    pub fn remove(&mut self, index: I) {
        let position = self.position[index]
            .take()
            .expect("Should not be called with indices off the heap.");
        self.heap.swap_remove(position);
        if position < self.len() {
            self.position[self.heap[position]] = Some(position);
            self.down_heap(position);
        }
    }

    /// Sets the value of the index and pushes it onto the heap.
    /// Should not be called with indices on the heap.
    pub fn push(&mut self, index: I, value: VS::Output) {
        self.value[index] = value;
        self.restore(index);
    }

    /// Pops the index with the smallest value.
    pub fn pop(&mut self) -> Option<I> {
        if self.is_empty() {
            return None;
        }
        let popped_index = self.heap.swap_remove(0);
        self.position[popped_index] = None;
        if !self.is_empty() {
            self.position[self.heap[0]] = Some(0);
        }
        self.down_heap(0);
        Some(popped_index)
    }

    fn parent(n: usize) -> usize {
        (n - 1) / 2
    }

    fn left(&self, n: usize) -> Option<usize> {
        let index = 2 * n + 1;
        (index < self.len()).then_some(index)
    }

    fn right(&self, n: usize) -> Option<usize> {
        let index = 2 * n + 2;
        (index < self.len()).then_some(index)
    }

    /// Swaps the positions of two indices on the heap by heap index.
    fn swap(&mut self, n: usize, m: usize) {
        self.position[self.heap[n]] = Some(m);
        self.position[self.heap[m]] = Some(n);
        self.heap.swap(n, m);
    }

    /// Compares two values of indices on the heap by heap index.
    fn compare(&self, n: usize, m: usize) -> Ordering {
        self.value[self.heap[n]].cmp(&self.value[self.heap[m]])
    }

    /// Performs up-heap bubbling from the given heap index.
    fn up_heap(&mut self, mut n: usize) {
        while n > 0 {
            let parent = Self::parent(n);
            if self.compare(parent, n).is_le() {
                break;
            }
            self.swap(n, parent);
            n = parent;
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
            if self.compare(n, smallest_child_index).is_le() {
                break;
            }
            self.swap(n, smallest_child_index);
            n = smallest_child_index;
        }
    }
}

macro_rules! generate_get_mut {
    ($struct_name:ident, $function_name:ident $(, $cfg_condition:tt)*) => {
        impl<I, VS, PS> IndexedPriorityQueue<I, VS, PS>
        where
            I: Copy,
            VS: IndexMut<I, Output: Ord + Sized + Clone>,
            PS: IndexMut<I, Output = Option<usize>>,
        {
            pub fn $function_name(&mut self, index: I) -> $struct_name<I, VS, PS> {
                $struct_name {
                    $(#[cfg($cfg_condition)])*
                    old_value: self.value[index].clone(),
                    heap: self,
                    index
                }
            }
        }

        pub struct $struct_name<'a, I, VS, PS>
        where
            I: Copy,
            VS: IndexMut<I, Output: Ord + Sized + Clone>,
            PS: IndexMut<I, Output = Option<usize>>,
        {
            $(#[cfg($cfg_condition)])*
            old_value: VS::Output,
            heap: &'a mut IndexedPriorityQueue<I, VS, PS>,
            index: I,
        }

        impl<'a, I, VS, PS> Deref for $struct_name<'a, I, VS, PS>
        where
            I: Copy,
            VS: IndexMut<I, Output: Ord + Sized + Clone>,
            PS: IndexMut<I, Output = Option<usize>>,
        {
            type Target = VS::Output;

            fn deref(&self) -> &Self::Target {
                &self.heap.value[self.index]
            }
        }

        impl<'a, I, VS, PS> DerefMut for $struct_name<'a, I, VS, PS>
        where
            I: Copy,
            VS: IndexMut<I, Output: Ord + Sized + Clone>,
            PS: IndexMut<I, Output = Option<usize>>,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.heap.value[self.index]
            }
        }
    };
}

generate_get_mut!(IndexedPriorityQueueRefUp, update_up, debug_assertions);
generate_get_mut!(IndexedPriorityQueueRefDown, update_down, debug_assertions);
generate_get_mut!(IndexedPriorityQueueRefDyn, update_dyn);

impl<'a, I, VS, PS> Drop for IndexedPriorityQueueRefUp<'a, I, VS, PS>
where
    I: Copy,
    VS: IndexMut<I, Output: Ord + Sized + Clone>,
    PS: IndexMut<I, Output = Option<usize>>,
{
    fn drop(&mut self) {
        debug_assert!(self.heap.value[self.index] >= self.old_value);
        if let Some(position) = self.heap.position[self.index] {
            self.heap.down_heap(position);
        }
    }
}

impl<'a, I, VS, PS> Drop for IndexedPriorityQueueRefDown<'a, I, VS, PS>
where
    I: Copy,
    VS: IndexMut<I, Output: Ord + Sized + Clone>,
    PS: IndexMut<I, Output = Option<usize>>,
{
    fn drop(&mut self) {
        debug_assert!(self.heap.value[self.index] <= self.old_value);
        if let Some(position) = self.heap.position[self.index] {
            self.heap.up_heap(position);
        }
    }
}

impl<'a, I, VS, PS> Drop for IndexedPriorityQueueRefDyn<'a, I, VS, PS>
where
    I: Copy,
    VS: IndexMut<I, Output: Ord + Sized + Clone>,
    PS: IndexMut<I, Output = Option<usize>>,
{
    fn drop(&mut self) {
        if let Some(position) = self.heap.position[self.index] {
            match self.heap.value[self.index].cmp(&self.old_value) {
                Ordering::Greater => self.heap.down_heap(position),
                Ordering::Less => self.heap.up_heap(position),
                Ordering::Equal => {}
            }
        }
    }
}
