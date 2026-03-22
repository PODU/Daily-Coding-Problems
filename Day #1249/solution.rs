// Validate BST with inclusive bounds (left <= node <= right) via recursive range check. O(n) time, O(h) space.

struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i64) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn is_valid(node: &Option<Box<Node>>, lo: i64, hi: i64) -> bool {
    match node {
        None => true,
        Some(n) => {
            if n.val < lo || n.val > hi {
                return false;
            }
            is_valid(&n.left, lo, n.val) && is_valid(&n.right, n.val, hi)
        }
    }
}

fn is_valid_bst(root: &Option<Box<Node>>) -> bool {
    is_valid(root, i64::MIN, i64::MAX)
}

fn main() {
    // Valid tree: root 5, left 3 (2,5), right 8 (5,12)
    let mut root = Node::new(5);
    let mut left = Node::new(3);
    left.left = Some(Node::new(2));
    left.right = Some(Node::new(5));
    let mut right = Node::new(8);
    right.left = Some(Node::new(5));
    right.right = Some(Node::new(12));
    root.left = Some(left);
    root.right = Some(right);
    let root_opt = Some(root);
    println!("{}", if is_valid_bst(&root_opt) { "true" } else { "false" });

    // Invalid tree: root 5, left 6 (6 > 5 violates)
    let mut bad = Node::new(5);
    bad.left = Some(Node::new(6));
    bad.right = Some(Node::new(8));
    let bad_opt = Some(bad);
    println!("{}", if is_valid_bst(&bad_opt) { "true" } else { "false" });
}
