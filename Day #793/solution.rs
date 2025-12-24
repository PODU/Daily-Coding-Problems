// Prune binary tree to a full binary tree: post-order recursion; if a node has
// exactly one child, replace it with its (already pruned) child. O(n) time, O(h) space.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None }))
    }
}

fn prune(node: Link) -> Link {
    let n = match node {
        Some(n) => n,
        None => return None,
    };
    let l = prune(n.borrow_mut().left.take());
    let r = prune(n.borrow_mut().right.take());
    match (l, r) {
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (l, r) => {
            n.borrow_mut().left = l;
            n.borrow_mut().right = r;
            Some(n)
        }
    }
}

fn main() {
    let n: Vec<Rc<RefCell<Node>>> = (0..8).map(Node::new).collect();
    n[0].borrow_mut().left = Some(n[1].clone());
    n[0].borrow_mut().right = Some(n[2].clone());
    n[1].borrow_mut().left = Some(n[3].clone());
    n[2].borrow_mut().right = Some(n[4].clone());
    n[3].borrow_mut().right = Some(n[5].clone());
    n[4].borrow_mut().left = Some(n[6].clone());
    n[4].borrow_mut().right = Some(n[7].clone());

    let root = prune(Some(n[0].clone()));

    let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while !q.is_empty() {
        let sz = q.len();
        let mut level: Vec<String> = Vec::new();
        for _ in 0..sz {
            let cur = q.pop_front().unwrap();
            level.push(cur.borrow().val.to_string());
            if let Some(l) = cur.borrow().left.clone() {
                q.push_back(l);
            }
            if let Some(r) = cur.borrow().right.clone() {
                q.push_back(r);
            }
        }
        println!("{}", level.join(" "));
    }
}
