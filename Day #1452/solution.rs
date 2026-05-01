// Day 1452: Sort a k-sorted array (each element <= k from its place) using a
// min-heap of size k+1. Time O(N log k), Space O(k).
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn sort_k_sorted(a: &[i32], k: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::new(); // max-heap; Reverse -> min-heap
    let mut out = Vec::with_capacity(a.len());
    for &x in a {
        heap.push(Reverse(x));
        if heap.len() > k {
            out.push(heap.pop().unwrap().0);
        }
    }
    while let Some(Reverse(v)) = heap.pop() {
        out.push(v);
    }
    out
}

fn main() {
    let a = [2, 6, 3, 12, 56, 8];
    println!("{:?}", sort_k_sorted(&a, 3)); // [2, 3, 6, 8, 12, 56]
}
