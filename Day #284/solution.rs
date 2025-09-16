// Day 284: Find all cousins of a node (same level, different parent) via BFS.
// Time O(N), Space O(N).
use std::cell::RefCell;
use std::collections::HashSet;
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

fn cousins(root: &Link, target: i32) -> Vec<i32> {
    let mut q: Vec<Rc<RefCell<Node>>> = Vec::new();
    if let Some(r) = root {
        q.push(r.clone());
    }
    while !q.is_empty() {
        let mut next = Vec::new();
        let mut level = Vec::new();
        let mut target_parent: Option<Rc<RefCell<Node>>> = None;
        for n in &q {
            let nb = n.borrow();
            for c in [&nb.left, &nb.right] {
                if let Some(cc) = c {
                    level.push(cc.borrow().val);
                    if cc.borrow().val == target {
                        target_parent = Some(n.clone());
                    }
                    next.push(cc.clone());
                }
            }
        }
        if let Some(tp) = target_parent {
            let tpb = tp.borrow();
            let mut sib = HashSet::new();
            if let Some(l) = &tpb.left {
                sib.insert(l.borrow().val);
            }
            if let Some(r) = &tpb.right {
                sib.insert(r.borrow().val);
            }
            return level.into_iter().filter(|v| !sib.contains(v)).collect();
        }
        q = next;
    }
    Vec::new()
}

fn main() {
    let root = node(
        1,
        node(2, node(4, None, None), node(5, None, None)),
        node(3, None, node(6, None, None)),
    );
    println!("{:?}", cousins(&root, 4)); // [6]
}
