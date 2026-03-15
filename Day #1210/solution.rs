// Day 1210: Floor and ceiling of a target in a BST.
// Single root-to-leaf descent updating best candidates. Time O(h), Space O(1).
struct Node {
    val: i32,
    l: Option<Box<Node>>,
    r: Option<Box<Node>>,
}

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node { val: v, l: None, r: None })),
        Some(mut n) => {
            if v < n.val {
                n.l = insert(n.l.take(), v);
            } else {
                n.r = insert(n.r.take(), v);
            }
            Some(n)
        }
    }
}

fn floor_ceil(root: &Option<Box<Node>>, x: i32) -> (Option<i32>, Option<i32>) {
    let mut fl = None;
    let mut ce = None;
    let mut cur = root;
    while let Some(n) = cur {
        if n.val == x {
            return (Some(x), Some(x));
        }
        if n.val < x {
            fl = Some(n.val);
            cur = &n.r;
        } else {
            ce = Some(n.val);
            cur = &n.l;
        }
    }
    (fl, ce)
}

fn show(p: Option<i32>) -> String {
    p.map_or("None".to_string(), |v| v.to_string())
}

fn main() {
    let mut root = None;
    for v in [8, 4, 12, 2, 6, 10, 14] {
        root = insert(root, v);
    }
    let (fl, ce) = floor_ceil(&root, 7);
    println!("floor={} ceiling={}", show(fl), show(ce)); // floor=6 ceiling=8
}
