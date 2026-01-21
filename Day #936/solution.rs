// Day 936: Sum of BST values within inclusive range [a,b], pruning branches out of range.
// Time O(n) worst, O(h + k) typical, Space O(h).

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn leaf(val: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { val, left: None, right: None }))
}

fn range_sum(n: &Option<Box<Node>>, a: i32, b: i32) -> i32 {
    match n {
        None => 0,
        Some(node) => {
            if node.val < a {
                range_sum(&node.right, a, b)
            } else if node.val > b {
                range_sum(&node.left, a, b)
            } else {
                node.val + range_sum(&node.left, a, b) + range_sum(&node.right, a, b)
            }
        }
    }
}

fn main() {
    let root = Some(Box::new(Node {
        val: 5,
        left: Some(Box::new(Node { val: 3, left: leaf(2), right: leaf(4) })),
        right: Some(Box::new(Node { val: 8, left: leaf(6), right: leaf(10) })),
    }));
    println!("{}", range_sum(&root, 4, 9)); // 23
}
