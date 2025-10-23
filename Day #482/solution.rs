// Day 482: BST range sum [a,b] inclusive via DFS with pruning.
// Skip left subtree if node<a, skip right if node>b. Time O(n) worst, O(k+h) typical; Space O(h).
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
    fn leaf(val: i32) -> Option<Box<Node>> {
        Some(Node::new(val, None, None))
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
    let root = Some(Node::new(
        5,
        Some(Node::new(3, Node::leaf(2), Node::leaf(4))),
        Some(Node::new(8, Node::leaf(6), Node::leaf(10))),
    ));
    println!("{}", range_sum(&root, 4, 9)); // 23
}
