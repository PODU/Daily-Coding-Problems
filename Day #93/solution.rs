// Day 93: Largest BST subtree size. Post-order DFS returning (isBST, size, min,
// max) per node and combining children. O(n) time, O(h) space.
struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// returns (is_bst, size, min, max)
fn dfs(n: &Option<Box<Node>>, best: &mut i64) -> (bool, i64, i64, i64) {
    match n {
        None => (true, 0, i64::MAX, i64::MIN),
        Some(node) => {
            let (lb, ls, lmin, lmax) = dfs(&node.left, best);
            let (rb, rs, rmin, rmax) = dfs(&node.right, best);
            if lb && rb && lmax < node.val && node.val < rmin {
                let size = ls + rs + 1;
                *best = (*best).max(size);
                (true, size, lmin.min(node.val), rmax.max(node.val))
            } else {
                (false, 0, 0, 0)
            }
        }
    }
}

fn leaf(v: i64) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn main() {
    let root = Some(Box::new(Node {
        val: 10,
        left: Some(Box::new(Node { val: 5, left: leaf(1), right: leaf(8) })),
        right: Some(Box::new(Node { val: 15, left: None, right: leaf(7) })),
    }));
    let mut best = 0;
    dfs(&root, &mut best);
    println!("{}", best); // 3
}
