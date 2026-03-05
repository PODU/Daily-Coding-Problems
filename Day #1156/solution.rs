// Level-order (BFS) traversal of a binary tree using a queue. O(n) time, O(n) space.
use std::collections::VecDeque;

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

fn level_order(root: &Option<Box<Node>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut queue: VecDeque<&Node> = VecDeque::new();
    if let Some(r) = root {
        queue.push_back(r);
    }
    while let Some(cur) = queue.pop_front() {
        res.push(cur.val);
        if let Some(l) = &cur.left {
            queue.push_back(l);
        }
        if let Some(r) = &cur.right {
            queue.push_back(r);
        }
    }
    res
}

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Node::new(2));
    let mut three = Node::new(3);
    three.left = Some(Node::new(4));
    three.right = Some(Node::new(5));
    root.right = Some(three);

    let root = Some(root);
    let vals = level_order(&root);
    let strs: Vec<String> = vals.iter().map(|v| v.to_string()).collect();
    println!("{}", strs.join(", "));
}
