use indexed_priority_queue::IndexedPriorityQueue;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Index, IndexMut};

#[derive(Default)]
pub struct DefaultMapped<K, V: Default>(HashMap<K, V>, V);

impl<K: Copy + Hash + Eq, V: Default> Index<K> for DefaultMapped<K, V> {
    type Output = V;

    fn index(&self, index: K) -> &Self::Output {
        self.0.get(&index).unwrap_or(&self.1)
    }
}

impl<K: Copy + Hash + Eq, V: Default> IndexMut<K> for DefaultMapped<K, V> {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        self.0.entry(index).or_default()
    }
}

// A distance is a `usize` with as default usize::MAX
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Distance(usize);

impl Default for Distance {
    fn default() -> Self {
        Distance(usize::MAX)
    }
}

pub type Queue = IndexedPriorityQueue<
    usize,
    DefaultMapped<usize, Distance>,
    DefaultMapped<usize, Option<usize>>,
>;

pub fn main() {
    // Graph from https://www.geeksforgeeks.org/introduction-to-dijkstras-shortest-path-algorithm/
    // The graph is represented as pairs of (neighbour, length)
    let graph: Vec<Vec<(usize, usize)>> = vec![
        vec![(1, 2), (2, 6)],
        vec![(0, 2), (3, 5)],
        vec![(0, 6), (3, 8)],
        vec![(2, 8), (1, 5), (4, 10), (5, 15)],
        vec![(3, 10), (6, 2)],
        vec![(3, 15), (6, 6)],
        vec![(4, 2), (5, 6)],
    ];

    // The start and end node id of the graph
    let start = 0;
    let end = 6;

    // Queue of nodes and the best path to them so far
    let mut queue = Queue::new(DefaultMapped::default(), DefaultMapped::default());
    queue.push(start, Distance(0));

    // While there are nodes to process
    while let Some(node) = queue.pop() {
        let node_best_distance = *queue.update_up(node);

        // If we found the end, check if the result is correct
        if node == end {
            assert_eq!(node_best_distance.0, 19);
            return;
        }

        for (neighbor, distance_to_neighbor) in &graph[node] {
            let mut neighbor_best_distance = queue.update_down(*neighbor);
            let alternative_distance = node_best_distance.0 + *distance_to_neighbor;
            if alternative_distance < neighbor_best_distance.0 {
                *neighbor_best_distance = Distance(alternative_distance);
                drop(neighbor_best_distance);
                queue.restore(*neighbor);
            }
        }
    }
    unreachable!();
}
