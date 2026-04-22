// BFS level-order: sum each level, track the level (1-indexed) with min sum.
// Time O(n), Space O(width).
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;
struct Node {
    val: i64,
    l: Link,
    r: Link,
}

fn node(val: i64, l: Link, r: Link) -> Link {
    Some(Rc::new(RefCell::new(Node { val, l, r })))
}

fn min_sum_level(root: &Link) -> (i32, i64) {
    if root.is_none() {
        return (-1, 0);
    }
    let mut q = vec![root.clone().unwrap()];
    let mut level = 0;
    let mut best_level = 1;
    let mut best_sum = i64::MAX;
    while !q.is_empty() {
        let mut next = Vec::new();
        let mut sum = 0i64;
        level += 1;
        for n in &q {
            let nb = n.borrow();
            sum += nb.val;
            if let Some(l) = &nb.l {
                next.push(l.clone());
            }
            if let Some(r) = &nb.r {
                next.push(r.clone());
            }
        }
        if sum < best_sum {
            best_sum = sum;
            best_level = level;
        }
        q = next;
    }
    (best_level, best_sum)
}

fn main() {
    let root = node(10,
        node(2, node(4, None, None), node(5, None, None)),
        node(3, None, None));
    let (lvl, sum) = min_sum_level(&root);
    println!("Level with minimum sum: {} (sum = {})", lvl, sum);
}
