// Day 1484: Construct all structurally unique BSTs with N nodes (values 1..N).
// For each root i, combine all left BSTs of (lo..i-1) with all right BSTs of
// (i+1..hi). Count is Catalan(N). Time/Space O(Catalan(N) * N).
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
    let mut trees = Vec::new();
    for i in lo..=hi {
        for l in build(lo, i - 1) {
            for r in build(i + 1, hi) {
                trees.push(Some(Rc::new(Node {
                    val: i,
                    left: l.clone(),
                    right: r.clone(),
                })));
            }
        }
    }
    trees
}

fn preorder(node: &Option<Rc<Node>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        out.push(n.val);
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let trees = build(1, 3);
    println!("{}", trees.len()); // 5
    for t in &trees {
        let mut out = Vec::new();
        preorder(t, &mut out);
        println!("{:?}", out);
    }
}
