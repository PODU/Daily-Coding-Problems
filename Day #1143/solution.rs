// Day 1143: Merge k sorted linked lists.
// Min-heap (BinaryHeap+Reverse) of values. Time O(N log k), Space O(N).
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Node { val: i32, next: Option<Box<Node>> }

fn build(vals: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn merge_k(lists: Vec<Option<Box<Node>>>) -> Option<Box<Node>> {
    // Drain all values into a min-heap, then rebuild (O(N log N)).
    let mut heap = BinaryHeap::new();
    for mut l in lists {
        while let Some(node) = l {
            heap.push(Reverse(node.val));
            l = node.next;
        }
    }
    let mut head: Option<Box<Node>> = None;
    let mut vals: Vec<i32> = Vec::new();
    while let Some(Reverse(v)) = heap.pop() { vals.push(v); }
    for &x in vals.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn main() {
    let lists = vec![build(&[1, 4, 7]), build(&[2, 5, 8]), build(&[3, 6, 9])];
    let mut out = Vec::new();
    let mut cur = merge_k(lists);
    while let Some(node) = cur {
        out.push(node.val.to_string());
        cur = node.next;
    }
    println!("{}", out.join(" -> ")); // 1 -> 2 -> ... -> 9
}
