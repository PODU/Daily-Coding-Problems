// Day 434: BST floor (largest <= n) and ceiling (smallest >= n).
// Single O(h) walk: node.val < n -> floor candidate (go right); node.val > n -> ceiling
// candidate (go left); equal -> both are n. O(h) time, O(1) space.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node { val: v, left: None, right: None })),
        Some(mut n) => {
            if v < n.val {
                n.left = insert(n.left.take(), v);
            } else {
                n.right = insert(n.right.take(), v);
            }
            Some(n)
        }
    }
}

fn floor_ceil(root: &Option<Box<Node>>, n: i32) -> (Option<i32>, Option<i32>) {
    let mut f = None;
    let mut c = None;
    let mut cur = root;
    while let Some(node) = cur {
        if node.val == n {
            return (Some(n), Some(n));
        } else if node.val < n {
            f = Some(node.val);
            cur = &node.right;
        } else {
            c = Some(node.val);
            cur = &node.left;
        }
    }
    (f, c)
}

fn main() {
    let mut root = None;
    for v in [8, 4, 12, 2, 6, 10, 14] {
        root = insert(root, v);
    }
    for n in [5, 8, 15, 1] {
        let (f, c) = floor_ceil(&root, n);
        let fs = match f { Some(x) => x.to_string(), None => "None".to_string() };
        let cs = match c { Some(x) => x.to_string(), None => "None".to_string() };
        println!("n={}: floor={}, ceiling={}", n, fs, cs);
    }
}
