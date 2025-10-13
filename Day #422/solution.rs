// Day 422: Merge two binary trees recursively (value = sum), O(n) time, O(h) space.
// Then print merged tree by level-order traversal (skipping null children).
use std::collections::VecDeque;

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(v: i32) -> Box<Node> {
        Box::new(Node { val: v, left: None, right: None })
    }
}

fn merge(a: Option<Box<Node>>, b: Option<Box<Node>>) -> Option<Box<Node>> {
    match (a, b) {
        (None, b) => b,
        (a, None) => a,
        (Some(mut x), Some(y)) => {
            x.val += y.val;
            x.left = merge(x.left.take(), y.left);
            x.right = merge(x.right.take(), y.right);
            Some(x)
        }
    }
}

fn main() {
    let mut t1 = Node::new(1);
    let mut l1 = Node::new(3);
    l1.left = Some(Node::new(5));
    t1.left = Some(l1);
    t1.right = Some(Node::new(2));

    let mut t2 = Node::new(2);
    let mut l2 = Node::new(1);
    l2.right = Some(Node::new(4));
    t2.left = Some(l2);
    let mut r2 = Node::new(3);
    r2.right = Some(Node::new(7));
    t2.right = Some(r2);

    let m = merge(Some(t1), Some(t2));

    let mut q: VecDeque<&Box<Node>> = VecDeque::new();
    let mut out: Vec<String> = Vec::new();
    if let Some(ref root) = m {
        q.push_back(root);
    }
    while let Some(c) = q.pop_front() {
        out.push(c.val.to_string());
        if let Some(ref l) = c.left {
            q.push_back(l);
        }
        if let Some(ref r) = c.right {
            q.push_back(r);
        }
    }
    println!("{}", out.join(" "));
}
