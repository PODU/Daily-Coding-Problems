// Day 490: Bottom view of a binary tree.
// BFS by horizontal distance (root 0, left hd-1, right hd+1); the last node seen in BFS
// order for each hd is the lowest. Output sorted by hd. Time O(n log n), Space O(n).
use std::collections::{BTreeMap, VecDeque};

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn bottom_view(root: &Option<Box<Node>>) -> Vec<i32> {
    let mut hd_to_val: BTreeMap<i32, i32> = BTreeMap::new();
    let mut q: VecDeque<(&Box<Node>, i32)> = VecDeque::new();
    if let Some(r) = root {
        q.push_back((r, 0));
    }
    while let Some((node, hd)) = q.pop_front() {
        hd_to_val.insert(hd, node.val); // last in BFS order = lowest
        if let Some(l) = &node.left {
            q.push_back((l, hd - 1));
        }
        if let Some(r) = &node.right {
            q.push_back((r, hd + 1));
        }
    }
    hd_to_val.values().copied().collect()
}

fn main() {
    let root = Some(Node::new(
        5,
        Some(Node::new(
            3,
            Some(Node::new(1, Some(Node::new(0, None, None)), None)),
            Some(Node::new(4, None, None)),
        )),
        Some(Node::new(
            7,
            Some(Node::new(6, None, None)),
            Some(Node::new(9, Some(Node::new(8, None, None)), None)),
        )),
    ));
    println!("{:?}", bottom_view(&root)); // [0, 1, 3, 6, 8, 9]
}
