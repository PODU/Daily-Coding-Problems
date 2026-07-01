// Approach: recursive generation of all BSTs; root choice + Cartesian product of left/right.
// Time/Space O(Catalan(N)*N).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn build(lo: i32, hi: i32) -> Vec<Link> {
    if lo > hi {
        return vec![None];
    }
    let mut res: Vec<Link> = Vec::new();
    for r in lo..=hi {
        let lefts = build(lo, r - 1);
        let rights = build(r + 1, hi);
        for l in &lefts {
            for ri in &rights {
                let root = Rc::new(RefCell::new(Node {
                    val: r,
                    left: l.clone(),
                    right: ri.clone(),
                }));
                res.push(Some(root));
            }
        }
    }
    res
}

fn preorder(node: &Link, out: &mut Vec<String>) {
    if let Some(n) = node {
        let n = n.borrow();
        out.push(n.val.to_string());
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let n = 3;
    let trees = build(1, n);
    println!("{}", trees.len());
    for t in &trees {
        let mut out: Vec<String> = Vec::new();
        preorder(t, &mut out);
        println!("{}", out.join(","));
    }
}
