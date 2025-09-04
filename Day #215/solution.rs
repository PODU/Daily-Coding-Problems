// Day 215: Bottom view of a binary tree.
// Approach: BFS tracking horizontal distance; overwrite map[hd] so last (deepest) node wins. Time O(n log n), Space O(n).
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
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

fn bottom_view(root: &Link) -> Vec<i32> {
    let mut hd_map: BTreeMap<i32, i32> = BTreeMap::new();
    let mut q: VecDeque<(Rc<RefCell<Node>>, i32)> = VecDeque::new();
    if let Some(r) = root {
        q.push_back((r.clone(), 0));
    }
    while let Some((n, hd)) = q.pop_front() {
        let nb = n.borrow();
        hd_map.insert(hd, nb.val);
        if let Some(l) = &nb.left {
            q.push_back((l.clone(), hd - 1));
        }
        if let Some(rt) = &nb.right {
            q.push_back((rt.clone(), hd + 1));
        }
    }
    hd_map.values().cloned().collect()
}

fn main() {
    let root = node(5,
        node(3, node(1, node(0, None, None), None), node(4, None, None)),
        node(7, node(6, None, None), node(9, node(8, None, None), None)));
    println!("{:?}", bottom_view(&root));
}
