// Day 994: Print binary tree nodes level by level (BFS).
// Use a queue; dequeue a node, emit it, enqueue its children. O(n) time/space.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Rc::new(RefCell::new(Node { val, left, right })))
}

fn level_order(root: Link) -> Vec<i32> {
    let mut out = Vec::new();
    let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while let Some(n) = q.pop_front() {
        let n = n.borrow();
        out.push(n.val);
        if let Some(l) = n.left.clone() {
            q.push_back(l);
        }
        if let Some(r) = n.right.clone() {
            q.push_back(r);
        }
    }
    out
}

fn main() {
    let root = node(1, node(2, None, None), node(3, node(4, None, None), node(5, None, None)));
    let vals: Vec<String> = level_order(root).iter().map(|v| v.to_string()).collect();
    println!("{}", vals.join(", ")); // 1, 2, 3, 4, 5
}
