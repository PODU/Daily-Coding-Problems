// Deepest node in a binary tree via BFS level order; last visited node is deepest.
// Time O(N), Space O(N).
use std::collections::VecDeque;

struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: char) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn deepest(root: &Node) -> char {
    let mut q: VecDeque<&Node> = VecDeque::new();
    q.push_back(root);
    let mut last = root.val;
    while let Some(n) = q.pop_front() {
        last = n.val;
        if let Some(ref l) = n.left {
            q.push_back(l);
        }
        if let Some(ref r) = n.right {
            q.push_back(r);
        }
    }
    last
}

fn main() {
    let mut a = Node::new('a');
    let mut b = Node::new('b');
    let c = Node::new('c');
    let d = Node::new('d');
    b.left = Some(d);
    a.left = Some(b);
    a.right = Some(c);
    println!("{}", deepest(&a)); // d
}
