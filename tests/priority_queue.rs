// Tests adapted from https://github.com/garro95/priority-queue/blob/4eecbd7ffa182a1def25d520dc78acd561032d11/tests/priority_queue.rs.
// todo: add license

use indexed_priority_queue::HashMapIPQ;
use std::cmp::Reverse;

#[test]
fn init() {
    let pq = HashMapIPQ::<&str, Reverse<i32>>::default();
    println!("{:?}", pq);
}

#[test]
fn push_len() {
    let mut pq = HashMapIPQ::<&str, Reverse<i32>>::default();
    pq.push("a", Reverse(1));
    pq.push("b", Reverse(2));
    println!("{:?}", pq);
    assert_eq!(pq.len(), 2);
}

#[test]
fn push_pop() {
    let mut pq = HashMapIPQ::<&str, Reverse<i32>>::default();
    assert_eq!(pq.min(), None);
    pq.push("a", Reverse(1));
    pq.push("b", Reverse(2));
    pq.push("f", Reverse(7));
    pq.push("g", Reverse(4));
    pq.push("h", Reverse(3));
    println!("{:?}", pq);
    assert_eq!(pq.pop(), Some("f"));
    println!("{:?}", pq);
    assert_eq!(pq.min(), Some(&"g"));
    assert_eq!(pq.pop(), Some("g"));
    assert_eq!(pq.len(), 3);
}

#[test]
fn push_update() {
    let mut pq = HashMapIPQ::<&str, Reverse<i32>>::default();
    pq.push("a", Reverse(9));
    pq.push("b", Reverse(8));
    pq.push("c", Reverse(7));
    pq.push("d", Reverse(6));
    pq.push("e", Reverse(5));
    pq.push("f", Reverse(4));
    pq.push("g", Reverse(10));
    pq.push("k", Reverse(11));

    // todo: push doesn't resort indices already in the queue. `update_down` would be better here
    pq.push("d", Reverse(20));

    assert_eq!(pq.min(), Some(&"d"));
    assert_eq!(pq.pop(), Some("d"));
}

// #[test]
// fn push_increase() {
//     let mut pq= HashMapIPQ::<&str, Reverse<i32>>::default();
//     pq.push("Processor", Reverse(1));
//     pq.push("Mainboard", Reverse(2));
//     pq.push("RAM", Reverse(5));
//     pq.push("GPU", Reverse(4));
//     pq.push("Disk", Reverse(3));
//
//     let processor_priority = |pq: &HashMapIPQ::<&str, Reverse<i32>>| {
//         *pq.iter()
//             .find_map(|(i, p)| if *i == "Processor" { Some(p) } else { None })
//             .unwrap()
//     };
//
//     pq.push_increase("Processor", 3);
//     assert_eq!(processor_priority(&pq), 3);
//
//     pq.push_increase("Processor", 1);
//     assert_eq!(processor_priority(&pq), 3);
//
//     pq.push_increase("Processor", 6);
//     assert_eq!(pq.peek(), Some((&"Processor", &6)));
// }

#[test]
fn change_priority() {
    let mut pq = HashMapIPQ::<&str, Reverse<i32>>::default();
    pq.push("Processor", Reverse(1));
    pq.push("Mainboard", Reverse(2));
    pq.push("RAM", Reverse(5));
    pq.push("GPU", Reverse(4));
    pq.push("Disk", Reverse(3));
    *pq.update_dyn("Processor") = Reverse(10);
    assert_eq!(pq.min(), Some(&"Processor"));

    *pq.update_dyn("RAM") = Reverse(11);
    assert_eq!(pq.min(), Some(&"RAM"));
}

// Ironically this test is not reversed.
#[test]
fn reversed_order() {
    let mut pq = HashMapIPQ::<&str, i32>::default();
    pq.push("a", 1);
    pq.push("b", 2);
    assert_eq!(pq.pop(), Some("a"));
}

// #[test]
// fn from_vec() {
//     let v = vec![("a", 1), ("b", 2), ("f", 7)];
//     let mut pq: PriorityQueue<_, _> = PriorityQueue::from(v);
//     assert_eq!(pq.pop(), Some(("f", 7)));
//     assert_eq!(pq.len(), 2);
// }

// #[test]
// fn from_vec_with_repeated() {
//     let v = vec![("a", 1), ("b", 2), ("f", 7), ("a", 2)];
//     let mut pq: PriorityQueue<_, _> = v.into();
//     assert_eq!(pq.pop(), Some(("f", 7)));
//     assert_eq!(pq.len(), 2);
// }

// #[test]
// fn from_iter() {
//     use std::iter::FromIterator;
//
//     let v = vec![("a", 1), ("b", 2), ("f", 7)];
//     let mut pq: PriorityQueue<_, _> = PriorityQueue::from_iter(v);
//     assert_eq!(pq.pop(), Some(("f", 7)));
//     assert_eq!(pq.len(), 2);
// }

// #[test]
// fn heap_sort() {
//     type Pq<I, P> = PriorityQueue<I, P>;
//
//     let v = vec![("a", 2), ("b", 7), ("f", 1)];
//     let sorted = (Pq::from(v)).into_sorted_vec();
//     assert_eq!(sorted.as_slice(), &["b", "a", "f"]);
// }

// #[test]
// fn change_priority_by() {
//     use std::iter::FromIterator;
//
//     let v = vec![("a", 1), ("b", 2), ("f", 7), ("g", 6), ("h", 5)];
//     let mut pq: PriorityQueue<_, _> = PriorityQueue::from_iter(v);
//
//     assert!(!pq.change_priority_by("z", |z| *z += 8));
//     assert!(pq.change_priority_by("b", |b| *b += 8));
//     assert_eq!(pq.into_sorted_vec().as_slice(), &["b", "f", "g", "h", "a"]);
// }

#[test]
fn remove_empty() {
    let mut pq = HashMapIPQ::<&str, Reverse<i32>>::default();

    pq.remove_index(&"b");
    assert!(pq.is_empty());
}

#[test]
fn remove_one() {
    let mut pq = HashMapIPQ::<&str, Reverse<i32>>::default();

    pq.push("b", Reverse(21));

    assert_eq!(pq.remove(&"b"), Reverse(21));
    assert!(pq.is_empty());
}

// #[test]
// fn remove() {
//     use std::iter::FromIterator;
//     type Pq<I, P> = PriorityQueue<I, P>;
//
//     let v = vec![("a", 1), ("b", 2), ("f", 7), ("g", 6), ("h", 5)];
//     let mut pq = Pq::from_iter(v);
//
//     pq.remove(&"b").unwrap();
//     pq.push("b", 2);
//     pq.remove(&"b");
//     assert_eq!(["f", "g", "h", "a"], pq.into_sorted_vec().as_slice());
// }

// #[test]
// fn remove2() {
//     use std::collections::hash_map::RandomState;
//     let mut queue = PriorityQueue::<i32, i32, RandomState>::default();
//
//     for i in 0..7 {
//         queue.push(i, i);
//     }
//
//     queue.remove(&0);
//
//     let mut last_priority = *queue.peek().unwrap().1;
//     while let Some((_, priority)) = queue.pop() {
//         dbg!(priority);
//         assert!(last_priority >= priority);
//         last_priority = priority;
//     }
//
//     let mut queue: PriorityQueue<i32, i32, RandomState> =
//         [20, 7, 19, 5, 6, 15, 18, 1, 2, 3, 4, 13, 14, 16, 17]
//             .iter()
//             .map(|i| (*i, *i))
//             .collect();
//
//     queue.remove(&1);
//
//     let mut last_priority = *queue.peek().unwrap().1;
//     while let Some((_, priority)) = queue.pop() {
//         dbg!(priority);
//         assert!(last_priority >= priority);
//         last_priority = priority;
//     }
// }

// #[test]
// fn extend() {
//     let mut pq = PriorityQueue::new();
//     pq.push("a", 1);
//     pq.push("b", 2);
//     pq.push("f", 7);
//
//     let v = vec![("c", 4), ("d", 6), ("e", 3)];
//     pq.extend(v);
//     assert_eq!(pq.len(), 6);
//     assert_eq!(
//         pq.into_sorted_vec().as_slice(),
//         &["f", "d", "c", "e", "b", "a"]
//     );
// }

// #[test]
// fn extend_empty() {
//     let mut pq = PriorityQueue::new();
//
//     let v = vec![("c", 4), ("d", 6), ("e", 3)];
//     pq.extend(v);
//     assert_eq!(pq.len(), 3);
//     assert_eq!(pq.into_sorted_vec().as_slice(), &["d", "c", "e"]);
// }

// #[test]
// fn iter() {
//     let mut pq = PriorityQueue::new();
//     pq.push("a", 1);
//     pq.push("b", 2);
//     pq.push("f", 7);
//
//     assert_eq!(pq.iter().count(), 3);
// }

// #[test]
// fn iter_mut() {
//     let mut pq = PriorityQueue::new();
//     pq.push("a", 1);
//     pq.push("b", 2);
//     pq.push("f", 7);
//     pq.push("g", 4);
//     pq.push("h", 3);
//
//     for (i, p) in &mut pq {
//         if *i < "f" {
//             *p += 18;
//         }
//     }
//
//     assert_eq!(pq.pop(), Some(("b", 20)));
//
//     /*
//     As expected, this does not compile
//     let iter_mut = pq.iter_mut();
//     iter_mut.for_each(|(_, p)| {*p += 2});
//
//     assert_eq!(pq.pop(), Some(("f", 9)));
//     */
// }

// #[test]
// fn into_sorted_iter() {
//     let mut pq = PriorityQueue::new();
//     pq.push("a", 1);
//     pq.push("b", 2);
//     pq.push("f", 7);
//
//     assert_eq!(
//         pq.into_sorted_iter().collect::<Vec<_>>(),
//         vec!(("f", 7), ("b", 2), ("a", 1))
//     );
// }
//
// #[test]
// fn iter_mut1() {
//     let mut queue: PriorityQueue<&'static str, i32> = Default::default();
//
//     queue.push("a", 0);
//     queue.push("b", 1);
//     assert_eq!(queue.peek().unwrap().0, &"b");
//
//     let iter_mut = queue.iter_mut();
//     for (k, v) in iter_mut {
//         if k == &"a" {
//             *v = 2;
//         }
//     }
//
//     assert_eq!(queue.peek().unwrap().0, &"a");
// }

// #[test]
// fn iter_mut_empty() {
//     let mut queue: PriorityQueue<&'static str, i32> = Default::default();
//
//     let iter_mut = queue.iter_mut();
//     for (k, v) in iter_mut {
//         if k == &"a" {
//             *v = 2;
//         }
//     }
// }

// #[test]
// fn eq() {
//     let mut a = HashMapIPQ::<&str, Reverse<i32>>::default();
//     let mut b = HashMapIPQ::<&str, Reverse<i32>>::default();
//     assert_eq!(a, b);
//
//     a.push("a", Reverse(1));
//     b.push("a", Reverse(1));
//     assert_eq!(a, b);
//
//     a.push("b", Reverse(2));
//     assert_ne!(a, b);
//
//     b.push("f", Reverse(7));
//     b.push("g", Reverse(4));
//     b.push("h", Reverse(3));
//     b.push("b", Reverse(2));
//
//     a.push("g", Reverse(4));
//     a.push("f", Reverse(7));
//     a.push("h", Reverse(3));
//     assert_eq!(a, b);
//     assert_eq!(b, a);
// }

// #[test]
// fn non_default_key() {
//     use std::time::*;
//     type PqType = PriorityQueue<i32, Instant>;
//     let _: PqType = PriorityQueue::default();
// }

// #[test]
// fn conversion() {
//     use priority_queue::DoublePriorityQueue;
//
//     let mut dpq = DoublePriorityQueue::new();
//
//     dpq.push('a', 3);
//     dpq.push('b', 5);
//     dpq.push('c', 1);
//
//     let mut pq: PriorityQueue<_, _> = dpq.into();
//
//     assert_eq!(pq.pop(), Some(('b', 5)));
// }
