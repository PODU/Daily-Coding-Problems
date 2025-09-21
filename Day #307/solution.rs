// Day 307: BST floor (largest <= x) and ceiling (smallest >= x). O(h) per query.
struct Node { v: i32, l: Option<Box<Node>>, r: Option<Box<Node>> }

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node { v, l: None, r: None })),
        Some(mut n) => {
            if v < n.v { n.l = insert(n.l.take(), v); } else { n.r = insert(n.r.take(), v); }
            Some(n)
        }
    }
}

fn floor_ceil(root: &Option<Box<Node>>, x: i32) -> (Option<i32>, Option<i32>) {
    let mut floor = None;
    let mut ceil = None;
    let mut cur = root;
    while let Some(n) = cur {
        if n.v == x { return (Some(n.v), Some(n.v)); }
        if n.v < x { floor = Some(n.v); cur = &n.r; }
        else { ceil = Some(n.v); cur = &n.l; }
    }
    (floor, ceil)
}

fn fmt(o: Option<i32>) -> String {
    o.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for v in [8, 4, 12, 2, 6, 10, 14] { root = insert(root, v); }
    let (fl, ce) = floor_ceil(&root, 5);
    println!("Floor: {}, Ceiling: {}", fmt(fl), fmt(ce)); // Floor: 4, Ceiling: 6
}
