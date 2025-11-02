// Day 540: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(n), Space O(n).

use std::cell::RefCell;
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

fn boustrophedon(root: &Link) -> Vec<i32> {
    let mut out = Vec::new();
    let mut queue: Vec<Rc<RefCell<Node>>> = match root {
        Some(r) => vec![r.clone()],
        None => return out,
    };
    let mut left_to_right = true;
    while !queue.is_empty() {
        let mut next = Vec::new();
        let mut level: Vec<i32> = Vec::with_capacity(queue.len());
        for n in &queue {
            let nb = n.borrow();
            level.push(nb.val);
            if let Some(l) = &nb.left {
                next.push(l.clone());
            }
            if let Some(r) = &nb.right {
                next.push(r.clone());
            }
        }
        if !left_to_right {
            level.reverse();
        }
        out.extend(level);
        left_to_right = !left_to_right;
        queue = next;
    }
    out
}

fn main() {
    let root = node(1,
        node(2, node(4, None, None), node(5, None, None)),
        node(3, node(6, None, None), node(7, None, None)));
    println!("{:?}", boustrophedon(&root));
}
