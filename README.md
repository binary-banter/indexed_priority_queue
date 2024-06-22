# Indexed Priority Queue

[![github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/binary-banter/indexed_priority_queue)
&ensp;[![crates-io](https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust)](https://crates.io/crates/indexed_priority_queue)
&ensp;[![docs-rs](https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs)](https://docs.rs/indexed_priority_queue)

An indexed priority queue with index-based removals, restores and value updates.

## About

A priority queue datastructure with the following operations:

|             Operation              | Time Complexity | Description                                                                                  |
|:----------------------------------:|:---------------:|----------------------------------------------------------------------------------------------|
|        `push(Index, Value)`        |   `O(log n)`    | Inserts an index-value pair into the queue.                                                  |
|      `pop() -> Option<Index>`      |   `O(log n)`    | Removes and returns the index with the smallest value from the priority queue.               |
|          `remove(Index)`           |   `O(log n)`    | Deletes the given index from the priority queue.                                             |
|          `restore(Index)`          |   `O(log n)`    | Reinserts a previously removed index into the priority queue with its last associated value. |
|      `min() -> Option<Index>`      |     `O(1)`      | Retrieves the index with the smallest value without removing it from the priority queue.     |
|       `get(Index) -> Value`        |     `O(1)`      | Returns the value associated with the given index. Panics if the index is not present.       |
| `update_dyn(Index) -> &mut Value`  |   `O(log n)`    | Modifies the value associated with the given index.                                          |
|  `update_up(Index) -> &mut Value`  |   `O(log n)`    | Increases the value associated with the given index. More efficient than `update_dyn`.       |
| `update_down(Index) -> &mut Value` |   `O(log n)`    | Decreases the value associated with the given index. More efficient than `update_dyn`.       |

## Examples

There are examples available at:
https://github.com/binary-banter/indexed_priority_queue/tree/main/examples