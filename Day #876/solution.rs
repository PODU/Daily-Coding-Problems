// Sort a k-sorted array with a min-heap of size k+1. Time O(N log k), Space O(k).
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_k_sorted(a: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut res = Vec::with_capacity(a.len());
    for &x in a {
        heap.push(Reverse(x));
        if heap.len() > k {
            res.push(heap.pop().unwrap().0);
        }
    }
    while let Some(Reverse(v)) = heap.pop() {
        res.push(v);
    }
    res
}

fn main() {
    let a = vec![6, 5, 3, 2, 8, 10, 9];
    println!("{:?}", sort_k_sorted(&a, 3));
}
