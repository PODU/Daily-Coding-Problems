// Day 83: Invert (mirror) a binary tree by swapping children recursively.
// Time O(n), Space O(h).
use std::collections::VecDeque;

struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn invert(node: &mut Option<Box<Node>>) {
    if let Some(n) = node {
        std::mem::swap(&mut n.left, &mut n.right);
        invert(&mut n.left);
        invert(&mut n.right);
    }
}

fn level_order(root: &Option<Box<Node>>) -> String {
    let mut out = Vec::new();
    let mut q: VecDeque<&Box<Node>> = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while let Some(n) = q.pop_front() {
        out.push(n.val.to_string());
        if let Some(l) = &n.left {
            q.push_back(l);
        }
        if let Some(r) = &n.right {
            q.push_back(r);
        }
    }
    out.join(" ")
}

fn leaf(v: char) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn main() {
    let mut a = Some(Box::new(Node {
        val: 'a',
        left: Some(Box::new(Node { val: 'b', left: leaf('d'), right: leaf('e') })),
        right: Some(Box::new(Node { val: 'c', left: leaf('f'), right: None })),
    }));
    println!("before: {}", level_order(&a)); // a b c d e f
    invert(&mut a);
    println!("after:  {}", level_order(&a)); // a c b f e d
}
