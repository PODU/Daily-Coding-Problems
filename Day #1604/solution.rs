// BST floor (largest <= x) & ceiling (smallest >= x). Iterative O(h) time, O(1) space.
// Floor: go right recording when val<=x else left. Ceiling: symmetric.
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

fn floor_bst(root: &Option<Box<Node>>, x: i32) -> Option<i32> {
    let mut res = None;
    let mut cur = root;
    while let Some(n) = cur {
        if n.val == x {
            return Some(x);
        }
        if n.val < x {
            res = Some(n.val);
            cur = &n.r;
        } else {
            cur = &n.l;
        }
    }
    res
}

fn ceil_bst(root: &Option<Box<Node>>, x: i32) -> Option<i32> {
    let mut res = None;
    let mut cur = root;
    while let Some(n) = cur {
        if n.val == x {
            return Some(x);
        }
        if n.val > x {
            res = Some(n.val);
            cur = &n.l;
        } else {
            cur = &n.r;
        }
    }
    res
}

fn show(v: Option<i32>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "None".to_string(),
    }
}

fn query(root: &Option<Box<Node>>, x: i32) {
    println!(
        "x={} -> floor {}, ceiling {}",
        x,
        show(floor_bst(root, x)),
        show(ceil_bst(root, x))
    );
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for v in [8, 4, 12, 2, 6, 10, 14] {
        root = insert(root, v);
    }
    query(&root, 5);  // floor 4, ceiling 6
    query(&root, 8);  // floor 8, ceiling 8
    query(&root, 15); // floor 14, ceiling None
    query(&root, 1);  // floor None, ceiling 2
}
