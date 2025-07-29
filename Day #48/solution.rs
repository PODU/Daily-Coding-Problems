// Day 48: Reconstruct binary tree from preorder + inorder.
// Hashmap of inorder positions; recurse. Time: O(n), Space: O(n).
use std::collections::{HashMap, VecDeque};

struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(
    pre: &[char],
    pre_idx: &mut usize,
    in_l: i32,
    in_r: i32,
    pos: &HashMap<char, i32>,
) -> Option<Box<Node>> {
    if in_l > in_r {
        return None;
    }
    let root_val = pre[*pre_idx];
    *pre_idx += 1;
    let mid = pos[&root_val];
    let left = build(pre, pre_idx, in_l, mid - 1, pos);
    let right = build(pre, pre_idx, mid + 1, in_r, pos);
    Some(Box::new(Node { val: root_val, left, right }))
}

fn main() {
    let pre: Vec<char> = "abdecfg".chars().collect();
    let ino: Vec<char> = "dbeafcg".chars().collect();
    let mut pos = HashMap::new();
    for (i, &c) in ino.iter().enumerate() {
        pos.insert(c, i as i32);
    }
    let mut pre_idx = 0;
    let root = build(&pre, &mut pre_idx, 0, ino.len() as i32 - 1, &pos);

    // Level-order traversal confirms reconstruction: a b c d e f g
    let mut out = Vec::new();
    let mut q: VecDeque<&Box<Node>> = VecDeque::new();
    if let Some(ref r) = root {
        q.push_back(r);
    }
    while let Some(n) = q.pop_front() {
        out.push(n.val.to_string());
        if let Some(ref l) = n.left {
            q.push_back(l);
        }
        if let Some(ref r) = n.right {
            q.push_back(r);
        }
    }
    println!("{}", out.join(" "));
}
