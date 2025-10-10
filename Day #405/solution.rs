// Largest BST subtree: bottom-up DFS returning (isBST, size, min, max); combine children.
// Time O(n), Space O(h).

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

// returns (is_bst, size, min, max)
fn dfs(node: &Option<Box<Node>>, best: &mut i32) -> (bool, i32, i32, i32) {
    match node {
        None => (true, 0, i32::MAX, i32::MIN),
        Some(n) => {
            let (lb, ls, lmin, lmax) = dfs(&n.left, best);
            let (rb, rs, rmin, rmax) = dfs(&n.right, best);
            if lb && rb && n.val > lmax && n.val < rmin {
                let size = ls + rs + 1;
                if size > *best {
                    *best = size;
                }
                return (true, size, lmin.min(n.val), rmax.max(n.val));
            }
            (false, 0, 0, 0)
        }
    }
}

fn main() {
    let mut root = Node::new(10);
    let mut left = Node::new(5);
    left.left = Some(Node::new(1));
    left.right = Some(Node::new(8));
    let mut right = Node::new(15);
    right.right = Some(Node::new(7));
    root.left = Some(left);
    root.right = Some(right);
    let some_root = Some(root);

    let mut best = 0;
    dfs(&some_root, &mut best);
    println!("{}", best);
}
