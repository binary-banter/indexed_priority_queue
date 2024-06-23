use indexed_priority_queue::{Indexed, IndexedPriorityQueue};
use ordered_float::OrderedFloat;
use std::cmp::Reverse;
use std::mem;

struct Priorities(Box<[Reverse<OrderedFloat<f32>>]>);
struct Positions(Box<[Option<usize>]>);
type VSIDS = IndexedPriorityQueue<usize, Priorities, Positions>;

impl Priorities {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(vec![Default::default(); capacity].into_boxed_slice())
    }
}

impl Positions {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(vec![Default::default(); capacity].into_boxed_slice())
    }
}

impl Indexed for Priorities {
    type Index = usize;
    type Output = Reverse<OrderedFloat<f32>>;

    fn get(&self, index: Self::Index) -> Option<&Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        Some(&self.0[index])
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        Some(&mut self.0[index])
    }

    fn insert(&mut self, index: Self::Index, mut value: Self::Output) -> Option<Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        let old_value = &mut self.0[index];
        mem::swap(old_value, &mut value);
        Some(value)
    }

    fn remove(&mut self, index: Self::Index) -> Option<Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        Some(self.0[index])
    }

    fn clear(&mut self) {}
}

impl Indexed for Positions {
    type Index = usize;
    type Output = usize;

    fn get(&self, index: Self::Index) -> Option<&Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        self.0[index].as_ref()
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        self.0[index].as_mut()
    }

    fn insert(&mut self, index: Self::Index, value: Self::Output) -> Option<Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        let old_value = self.0[index];
        self.0[index] = Some(value);
        old_value
    }

    fn remove(&mut self, index: Self::Index) -> Option<Self::Output> {
        let index = index - 1;
        debug_assert!(index < self.0.len());

        self.0[index].take()
    }

    fn clear(&mut self) {}
}

// Creates a Variable State Independent Decay Sum (VSIDS) datastructure for use in SAT-solving.
// It must be able to remove, restore and update entries by index and pop the entry with the largest value.
// Since the number of variables is constant for most SAT-solvers, we use a fixed size array for `VS` and `PS`.
pub fn main() {
    // Capacity for 4 variables.
    let capacity = 4;
    let mut vsids = VSIDS::with_capacity(
        Priorities::with_capacity(capacity),
        Positions::with_capacity(capacity),
        capacity,
    );

    // Initialize the counters.
    for variable in 1..=4 {
        vsids.push(variable, Reverse(0.0.into()));
    }

    // Assign variables 1, 2 and 3.
    vsids.remove_index(1);
    vsids.remove_index(2);
    vsids.remove_index(3);

    // Conflict on variable 3.
    vsids.update_down(3).0 .0 += 1.0;
    vsids.restore_index(3);

    // During branching, we will now select variable 3.
    assert_eq!(vsids.pop(), Some(3));

    // Assign variable 4.
    vsids.remove_index(4);

    // Conflict on variables 3 and 4.
    vsids.update_down(4).0 .0 += 1.0;
    vsids.restore_index(4);
    vsids.update_down(3).0 .0 += 1.0;
    vsids.restore_index(3);

    // During branching, we will now select variable 3 followed by 4.
    assert_eq!(vsids.pop(), Some(3));
    assert_eq!(vsids.pop(), Some(4));

    // No conflict and no more variables to select, so SAT!
    assert_eq!(vsids.pop(), None);
}
