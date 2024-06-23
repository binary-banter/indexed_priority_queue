use indexed_priority_queue::IndexedPriorityQueue;
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::ops::{Index, IndexMut};

pub struct Boxed<T>(Box<[T]>);

impl<T: Default + Clone> Boxed<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Boxed(vec![T::default(); capacity].into_boxed_slice())
    }
}

impl<T> Index<usize> for Boxed<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index - 1]
    }
}

impl<T> IndexMut<usize> for Boxed<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index - 1]
    }
}

pub type VSIDS =
    IndexedPriorityQueue<usize, Boxed<Reverse<OrderedFloat<f32>>>, Boxed<Option<usize>>>;

// Creates a Variable State Independent Decay Sum (VSIDS) datastructure for use in SAT-solving.
// It must be able to remove, restore and update entries by index and pop the entry with the largest value.
// Since the number of variables is constant for most SAT-solvers, we use a fixed size array for `VS` and `PS`.
pub fn main() {
    // Capacity for 4 variables.
    let capacity = 4;
    let mut vsids = VSIDS::with_capacity(
        Boxed::with_capacity(capacity),
        Boxed::with_capacity(capacity),
        capacity,
    );

    // Initialize the counters.
    for variable in 1..=4 {
        vsids.push(variable, Reverse(0.0.into()));
    }

    // Assign variables 1, 2 and 3.
    vsids.remove(1);
    vsids.remove(2);
    vsids.remove(3);

    // Conflict on variable 3.
    vsids.update_down(3).0 .0 += 1.0;
    vsids.restore(3);

    // During branching, we will now select variable 3.
    assert_eq!(vsids.pop(), Some(3));

    // Assign variable 4.
    vsids.remove(4);

    // Conflict on variables 3 and 4.
    vsids.update_down(4).0 .0 += 1.0;
    vsids.restore(4);
    vsids.update_down(3).0 .0 += 1.0;
    vsids.restore(3);

    // During branching, we will now select variable 3 followed by 4.
    assert_eq!(vsids.pop(), Some(3));
    assert_eq!(vsids.pop(), Some(4));

    // No conflict and no more variables to select, so SAT!
    assert_eq!(vsids.pop(), None);
}
