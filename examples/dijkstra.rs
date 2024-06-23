use indexed_priority_queue::{Indexed, IndexedPriorityQueue};
use std::collections::HashMap;
use std::hash::Hash;

// A distance is a `usize` with as default `usize::MAX`.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Distance(usize);

impl Default for Distance {
    fn default() -> Self {
        Distance(usize::MAX)
    }
}

impl Default for &Distance {
    fn default() -> Self {
        &Distance(usize::MAX)
    }
}

#[derive(Default)]
struct DefaultMap<K, V>(HashMap<K, V>);

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

#[derive(Default)]
struct Map<K, V>(HashMap<K, V>);

impl<K: Eq + Hash, V> Indexed for Map<K, V> {
    type Index = K;
    type Output = V;

    fn get(&self, index: K) -> Option<&Self::Output> {
        self.0.get(&index)
    }

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Output> {
        self.0.get_mut(&index)
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

type Queue = IndexedPriorityQueue<usize, DefaultMap<usize, Distance>, Map<usize, usize>>;

pub fn main() {
    // Graph from https://www.geeksforgeeks.org/introduction-to-dijkstras-shortest-path-algorithm/
    // The graph is represented as pairs of (neighbour, length)
    let graph = vec![
        vec![(1, 2), (2, 6)],
        vec![(0, 2), (3, 5)],
        vec![(0, 6), (3, 8)],
        vec![(2, 8), (1, 5), (4, 10), (5, 15)],
        vec![(3, 10), (6, 2)],
        vec![(3, 15), (6, 6)],
        vec![(4, 2), (5, 6)],
    ];

    // The start and end node id of the graph
    let (start, end) = (0, 6);

    // Queue of nodes and the best path to them so far
    let mut queue = Queue::new(DefaultMap::default(), Map::default());
    queue.push(start, Distance(0));

    // While there are nodes to process
    while let Some(node) = queue.pop() {
        let node_best_distance = queue.get_priority(node).unwrap().0;

        // If we found the end, check if the result is correct
        if node == end {
            assert_eq!(node_best_distance, 19);
            return;
        }

        for (neighbor, distance_to_neighbor) in &graph[node] {
            let mut neighbor_best_distance = queue.update_down(*neighbor);
            let alternative_distance = node_best_distance + *distance_to_neighbor;
            if alternative_distance < neighbor_best_distance.0 {
                *neighbor_best_distance = Distance(alternative_distance);
                drop(neighbor_best_distance);
                queue.restore_index(*neighbor);
            }
        }
    }

    unreachable!();
}
