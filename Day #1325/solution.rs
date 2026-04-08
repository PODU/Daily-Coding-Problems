// Day 1325: Sorted array -> height-balanced BST.
// Recursively pick the middle element as the root so both halves differ in height by <=1. O(n) time, O(log n) stack.

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(a: &[i32], lo: i64, hi: i64) -> Option<Box<Node>> {
    if lo > hi {
        return None;
    }
    let mid = lo + (hi - lo) / 2;
    Some(Box::new(Node {
        val: a[mid as usize],
        left: build(a, lo, mid - 1),
        right: build(a, mid + 1, hi),
    }))
}

fn preorder(node: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        out.push(n.val);
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let a = vec![1, 2, 3, 4, 5, 6, 7];
    let root = build(&a, 0, a.len() as i64 - 1);
    let mut out = Vec::new();
    preorder(&root, &mut out);
    println!("preorder: {:?}", out); // preorder: [4, 2, 1, 3, 6, 5, 7]
}
