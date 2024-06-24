use indexed_priority_queue::ArrayMapIPQ;
use ordered_float::OrderedFloat;
use std::cmp::Reverse;

// Creates a Variable State Independent Decay Sum (VSIDS) datastructure for use in SAT-solving.
// It must be able to remove, restore and update entries by index and pop the entry with the largest value.
// Since the number of variables is constant for most SAT-solvers, we use a fixed size array for `VS` and `PS`.
pub fn main() {
    // Capacity for 4 variables.
    let capacity = 4;

    // Initialize the queue.
    let mut vsids = ArrayMapIPQ::with_capacity(
        vec![Reverse(OrderedFloat(0.)); capacity].into_boxed_slice(),
        vec![None; capacity].into_boxed_slice(),
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
