// Day 664: Maximum path sum between any two nodes in a binary tree.
// Post-order DFS: each node returns best downward gain; track best "bridge". Time O(n), Space O(h).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;
struct Node {
    val: i32,
    l: Link,
    r: Link,
}

fn node(val: i32, l: Link, r: Link) -> Link {
    Some(Rc::new(RefCell::new(Node { val, l, r })))
}

fn gain(n: &Link, best: &mut i32) -> i32 {
    match n {
        None => 0,
        Some(rc) => {
            let b = rc.borrow();
            let lg = gain(&b.l, best).max(0);
            let rg = gain(&b.r, best).max(0);
            *best = (*best).max(b.val + lg + rg);
            b.val + lg.max(rg)
        }
    }
}

fn max_path_sum(root: &Link) -> i32 {
    let mut best = i32::MIN;
    gain(root, &mut best);
    best
}

fn main() {
    let root = node(
        -10,
        node(9, None, None),
        node(20, node(15, None, None), node(7, None, None)),
    );
    println!("{}", max_path_sum(&root)); // 42
}
