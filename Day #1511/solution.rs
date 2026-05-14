// Range Sum of BST via DFS with BST pruning (skip left if node<a, right if node>b).
// O(n) worst-case time, O(h) recursion space.

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn range_sum(root: &Option<Box<Node>>, a: i32, b: i32) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            if n.val < a {
                range_sum(&n.right, a, b)
            } else if n.val > b {
                range_sum(&n.left, a, b)
            } else {
                n.val + range_sum(&n.left, a, b) + range_sum(&n.right, a, b)
            }
        }
    }
}

fn main() {
    let mut root = Node::new(5);
    let mut left = Node::new(3);
    left.left = Some(Node::new(2));
    left.right = Some(Node::new(4));
    let mut right = Node::new(8);
    right.left = Some(Node::new(6));
    right.right = Some(Node::new(10));
    root.left = Some(left);
    root.right = Some(right);
    let tree = Some(root);
    println!("{}", range_sum(&tree, 4, 9));
}
