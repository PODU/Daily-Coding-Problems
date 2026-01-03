// Day 842: invert (mirror) a binary tree by swapping children at every node.
// Recursive DFS. O(n) time, O(h) stack space.
use std::collections::VecDeque;

type Link = Option<Box<Node>>;

struct Node {
    v: char,
    l: Link,
    r: Link,
}

impl Node {
    fn new(v: char) -> Box<Node> {
        Box::new(Node { v, l: None, r: None })
    }
}

fn invert(root: &mut Link) {
    if let Some(n) = root {
        std::mem::swap(&mut n.l, &mut n.r);
        invert(&mut n.l);
        invert(&mut n.r);
    }
}

fn level_order(root: &Link) -> String {
    let mut out = Vec::new();
    let mut q: VecDeque<&Box<Node>> = VecDeque::new();
    if let Some(n) = root {
        q.push_back(n);
    }
    while let Some(n) = q.pop_front() {
        out.push(n.v);
        if let Some(l) = &n.l {
            q.push_back(l);
        }
        if let Some(r) = &n.r {
            q.push_back(r);
        }
    }
    out.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let mut a = Node::new('a');
    let mut b = Node::new('b');
    let c = Node::new('c'); // mut not needed after building below via temp
    let d = Node::new('d');
    let e = Node::new('e');
    let f = Node::new('f');

    let mut b = b;
    b.l = Some(d);
    b.r = Some(e);
    let mut c = c;
    c.l = Some(f);
    a.l = Some(b);
    a.r = Some(c);

    let mut root: Link = Some(a);
    invert(&mut root);
    println!("{}", level_order(&root)); // a c b f e d
}
