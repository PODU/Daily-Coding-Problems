// Sort a k-sorted array (each element <= k from its sorted position).
// Min-heap of size k+1: pop the min as the next sorted element. O(N log k) time, O(k) space.
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn sort_k_sorted(arr: &[i32], k: usize) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut result = Vec::with_capacity(arr.len());
    let mut i = 0usize;
    while i <= k && i < arr.len() {
        heap.push(Reverse(arr[i]));
        i += 1;
    }
    while i < arr.len() {
        let Reverse(v) = heap.pop().unwrap();
        result.push(v);
        heap.push(Reverse(arr[i]));
        i += 1;
    }
    while let Some(Reverse(v)) = heap.pop() {
        result.push(v);
    }
    result
}

fn main() {
    let arr = vec![2, 1, 4, 3, 6, 5]; // k-sorted with k = 2
    let sorted = sort_k_sorted(&arr, 2);
    let parts: Vec<String> = sorted.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
