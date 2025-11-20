// Day 633: Sort a k-sorted (nearly sorted) array.
// Approach: min-heap of size k+1; pop smallest as we slide the window.
// Time: O(N log k), Space: O(k).
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_k_sorted(arr: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut res = Vec::with_capacity(arr.len());
    for &x in arr {
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
    let arr = vec![2, 1, 4, 3, 5]; // k = 1
    println!("{:?}", sort_k_sorted(&arr, 1));
}
