// Prune binary tree: remove subtrees containing only 0s (no 1 descendant).
// Post-order recursion. O(n) time, O(h) recursion stack.
use std::collections::VecDeque;

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Box::new(Node { val, left, right }))
    }
}

fn prune(node: Link) -> Link {
    match node {
        None => None,
        Some(mut n) => {
            n.left = prune(n.left.take());
            n.right = prune(n.right.take());
            if n.val == 0 && n.left.is_none() && n.right.is_none() {
                None
            } else {
                Some(n)
            }
        }
    }
}

fn level_order(root: &Link) -> String {
    let mut q: VecDeque<&Link> = VecDeque::new();
    q.push_back(root);
    let mut out: Vec<String> = Vec::new();
    while let Some(cur) = q.pop_front() {
        match cur {
            None => out.push("null".to_string()),
            Some(n) => {
                out.push(n.val.to_string());
                q.push_back(&n.left);
                q.push_back(&n.right);
            }
        }
    }
    while out.last().map_or(false, |s| s == "null") {
        out.pop();
    }
    format!("[{}]", out.join(", "))
}

fn main() {
    let root = Node::new(
        0,
        Node::new(1, None, None),
        Node::new(
            0,
            Node::new(1, Node::new(0, None, None), Node::new(0, None, None)),
            Node::new(0, None, None),
        ),
    );
    let pruned = prune(root);
    println!("{}", level_order(&pruned)); // [0, 1, 0, null, null, 1]
}
