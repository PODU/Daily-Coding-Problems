// Range Sum of BST. Pruned DFS using BST property. Time O(n) worst, Space O(h).
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
            let mut s = 0;
            if n.val >= a && n.val <= b {
                s += n.val;
            }
            if n.val > a {
                s += range_sum(&n.left, a, b);
            }
            if n.val < b {
                s += range_sum(&n.right, a, b);
            }
            s
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
    let root = Some(root);
    println!("{}", range_sum(&root, 4, 9));
}
