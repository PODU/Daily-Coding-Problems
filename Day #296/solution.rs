// Sorted array -> height-balanced BST: pick lower-mid as root, recurse halves; print BFS level-order.
// Time O(n), Space O(log n) recursion.
use std::collections::VecDeque;

struct Node {
    val: i64,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

fn build(a: &[i64], lo: i64, hi: i64) -> Option<Box<Node>> {
    if lo > hi {
        return None;
    }
    let mid = (lo + hi) / 2;
    Some(Box::new(Node {
        val: a[mid as usize],
        l: build(a, lo, mid - 1),
        r: build(a, mid + 1, hi),
    }))
}

fn main() {
    let a = vec![-10i64, -3, 0, 5, 9];
    let root = build(&a, 0, a.len() as i64 - 1);
    let mut order: Vec<i64> = Vec::new();
    let mut q: VecDeque<&Option<Box<Node>>> = VecDeque::new();
    q.push_back(&root);
    while let Some(opt) = q.pop_front() {
        if let Some(n) = opt {
            order.push(n.val);
            q.push_back(&n.l);
            q.push_back(&n.r);
        }
    }
    let parts: Vec<String> = order.iter().map(|x| x.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
