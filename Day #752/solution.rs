// Day 752: Level-order (BFS) traversal of a binary tree.
// Time: O(n), Space: O(w) where w is the max width of the tree.
use std::collections::VecDeque;

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn level_order(root: &Link) -> Vec<i32> {
    let mut out = Vec::new();
    let mut q: VecDeque<&Node> = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while let Some(n) = q.pop_front() {
        out.push(n.val);
        if let Some(l) = &n.left {
            q.push_back(l);
        }
        if let Some(r) = &n.right {
            q.push_back(r);
        }
    }
    out
}

fn main() {
    let root = node(1,
        node(2, None, None),
        node(3, node(4, None, None), node(5, None, None)));

    let res = level_order(&root);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(", "));
}
