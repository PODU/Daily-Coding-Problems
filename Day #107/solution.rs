// Day 107: Level-order (BFS) traversal of a binary tree. O(n) time, O(n) space.
use std::collections::VecDeque;

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn level_order(root: &Node) -> Vec<i32> {
    let mut out = Vec::new();
    let mut q: VecDeque<&Node> = VecDeque::new();
    q.push_back(root);
    while let Some(n) = q.pop_front() {
        out.push(n.val);
        if let Some(ref l) = n.left {
            q.push_back(l);
        }
        if let Some(ref r) = n.right {
            q.push_back(r);
        }
    }
    out
}

fn main() {
    let root = Node::new(
        1,
        Some(Node::new(2, None, None)),
        Some(Node::new(
            3,
            Some(Node::new(4, None, None)),
            Some(Node::new(5, None, None)),
        )),
    );
    let strs: Vec<String> = level_order(&root).iter().map(|x| x.to_string()).collect();
    println!("{}", strs.join(", "));
}
