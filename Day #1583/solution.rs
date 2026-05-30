// Day 1583: Bottom view of a binary tree.
// BFS tracking horizontal distance; last node seen per hd (deepest wins). Time: O(n log n); Space: O(n).
use std::collections::{BTreeMap, VecDeque};

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    l: Link,
    r: Link,
}

fn node(val: i32, l: Link, r: Link) -> Link {
    Some(Box::new(Node { val, l, r }))
}

fn bottom_view(root: &Link) -> Vec<i32> {
    let mut hd_val: BTreeMap<i32, i32> = BTreeMap::new();
    let mut q: VecDeque<(&Node, i32)> = VecDeque::new();
    if let Some(r) = root {
        q.push_back((r, 0));
    }
    while let Some((n, hd)) = q.pop_front() {
        hd_val.insert(hd, n.val);
        if let Some(l) = &n.l {
            q.push_back((l, hd - 1));
        }
        if let Some(rr) = &n.r {
            q.push_back((rr, hd + 1));
        }
    }
    hd_val.values().cloned().collect()
}

fn main() {
    let root = node(
        5,
        node(3, node(1, node(0, None, None), None), node(4, None, None)),
        node(7, node(6, None, None), node(9, node(8, None, None), None)),
    );
    println!("{:?}", bottom_view(&root)); // [0, 1, 3, 6, 8, 9]
}
