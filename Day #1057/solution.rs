// Generate all distinct BSTs with values 1..N: recursively pick each value as
// root, combine all left/right subtree shapes. Count is Catalan(N).
// Time/Space O(Catalan(N) * N).
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<TreeNode>>,
    right: Option<Rc<TreeNode>>,
}

fn build(lo: i32, hi: i32) -> Vec<Option<Rc<TreeNode>>> {
    if lo > hi {
        return vec![None];
    }
    let mut res = Vec::new();
    for root in lo..=hi {
        let lefts = build(lo, root - 1);
        let rights = build(root + 1, hi);
        for l in &lefts {
            for r in &rights {
                res.push(Some(Rc::new(TreeNode {
                    val: root,
                    left: l.clone(),
                    right: r.clone(),
                })));
            }
        }
    }
    res
}

fn preorder(node: &Option<Rc<TreeNode>>, out: &mut String) {
    match node {
        None => out.push('#'),
        Some(n) => {
            out.push_str(&n.val.to_string());
            out.push(' ');
            preorder(&n.left, out);
            out.push(' ');
            preorder(&n.right, out);
        }
    }
}

fn main() {
    let n = 3;
    let trees = build(1, n);
    println!("{}", trees.len()); // 5 for N=3
    for t in &trees {
        let mut s = String::new();
        preorder(t, &mut s);
        println!("{}", s);
    }
}
