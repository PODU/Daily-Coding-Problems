// Day 487: Find all cousins of a target node (same level, different parent).
// BFS level by level tracking each node's parent id; on the target's level collect
// nodes whose parent differs from the target's parent. Time O(n), Space O(n).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }
}

fn cousins(root: &Link, target: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let r = match root {
        Some(r) => r.clone(),
        None => return res,
    };
    // Each level entry: (node, parent_id). Root has parent id 0 (no real parent).
    let mut level: Vec<(Rc<RefCell<Node>>, usize)> = vec![(r, 0)];
    while !level.is_empty() {
        let mut target_parent: Option<usize> = None;
        let mut next: Vec<(Rc<RefCell<Node>>, usize)> = Vec::new();
        for (node, pid) in &level {
            let id = Rc::as_ptr(node) as usize;
            let b = node.borrow();
            if b.val == target {
                target_parent = Some(*pid);
            }
            if let Some(l) = &b.left {
                next.push((l.clone(), id));
            }
            if let Some(rt) = &b.right {
                next.push((rt.clone(), id));
            }
        }
        if let Some(tp) = target_parent {
            for (node, pid) in &level {
                let v = node.borrow().val;
                if *pid != tp && v != target {
                    res.push(v);
                }
            }
            return res;
        }
        level = next;
    }
    res
}

fn main() {
    let root = Node::new(
        1,
        Node::new(2, Node::new(4, None, None), Node::new(5, None, None)),
        Node::new(3, None, Node::new(6, None, None)),
    );
    for v in cousins(&root, 4) {
        println!("{}", v); // 6
    }
}
