// Sorted array -> height-balanced BST: pick lower-middle as root, recurse. Print preorder.
// mid = (lo+hi)/2 (lower-middle). Time O(n), Space O(log n) recursion.

struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(a: &[i64], lo: i64, hi: i64) -> Option<Box<Node>> {
    if lo > hi {
        return None;
    }
    let mid = (lo + hi) / 2;
    Some(Box::new(Node {
        val: a[mid as usize],
        left: build(a, lo, mid - 1),
        right: build(a, mid + 1, hi),
    }))
}

fn preorder(node: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(n) = node {
        out.push(n.val);
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let a: Vec<i64> = vec![-10, -3, 0, 5, 9];
    let root = build(&a, 0, a.len() as i64 - 1);
    let mut out: Vec<i64> = Vec::new();
    preorder(&root, &mut out);
    let parts: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
