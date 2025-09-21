// Day 306: Sort a k-sorted array with a min-heap of size k+1. O(N log k).
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_k(arr: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut res = Vec::new();
    let mut i = 0;
    while i < arr.len() && i <= k { heap.push(Reverse(arr[i])); i += 1; }
    while i < arr.len() {
        res.push(heap.pop().unwrap().0);
        heap.push(Reverse(arr[i]));
        i += 1;
    }
    while let Some(Reverse(v)) = heap.pop() { res.push(v); }
    res
}

fn main() {
    let r = sort_k(&[6, 5, 3, 2, 8, 10, 9], 3);
    println!("{}", r.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ")); // 2 3 5 6 8 9 10
}
