// Day 278: Generate all structurally distinct BSTs with N nodes (values 1..N).
// Recursive divide on root choice. Count = Catalan(N). Time O(Catalan(N)*N).
use std::rc::Rc;

struct Node {
    val: i32,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

fn build(lo: i32, hi: i32) -> Vec<Option<Rc<Node>>> {
    if lo > hi {
        return vec![None];
    }
    let mut res = Vec::new();
    for r in lo..=hi {
        for l in build(lo, r - 1) {
            for rt in build(r + 1, hi) {
                res.push(Some(Rc::new(Node {
                    val: r,
                    left: l.clone(),
                    right: rt.clone(),
                })));
            }
        }
    }
    res
}

fn preorder(n: &Option<Rc<Node>>) -> String {
    match n {
        None => "#".to_string(),
        Some(x) => format!("{} {} {}", x.val, preorder(&x.left), preorder(&x.right)),
    }
}

fn main() {
    let n = 3;
    let trees = build(1, n);
    println!("Count: {}", trees.len()); // 5
    for t in &trees {
        println!("{}", preorder(t));
    }
}
