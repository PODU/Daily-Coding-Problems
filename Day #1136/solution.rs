// Bottom view via BFS tracking horizontal distance; last (deepest) node per hd wins. O(n log n).
use std::collections::{BTreeMap, VecDeque};

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn node(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { val, left, right }))
}

fn main() {
    let root = node(5,
        node(3, node(1, node(0, None, None), None), node(4, None, None)),
        node(7, node(6, None, None), node(9, node(8, None, None), None)));
    let mut map: BTreeMap<i32, i32> = BTreeMap::new();
    let mut q: VecDeque<(&Box<Node>, i32)> = VecDeque::new();
    if let Some(ref r) = root {
        q.push_back((r, 0));
    }
    while let Some((n, hd)) = q.pop_front() {
        map.insert(hd, n.val);
        if let Some(ref l) = n.left {
            q.push_back((l, hd - 1));
        }
        if let Some(ref rr) = n.right {
            q.push_back((rr, hd + 1));
        }
    }
    let parts: Vec<String> = map.values().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
