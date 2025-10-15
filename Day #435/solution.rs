// Day 435: Reconstruct a binary tree from preorder + inorder traversals.
// Approach: recurse, using a HashMap of inorder value->index to find roots in O(1).
// Time: O(n), Space: O(n).
use std::collections::HashMap;
use std::collections::VecDeque;

struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(
    pre: &[String],
    pi: &mut usize,
    in_l: i32,
    in_r: i32,
    idx: &HashMap<String, i32>,
) -> Option<Box<Node>> {
    if in_l > in_r {
        return None;
    }
    let root_val = pre[*pi].clone();
    *pi += 1;
    let mid = idx[&root_val];
    let left = build(pre, pi, in_l, mid - 1, idx);
    let right = build(pre, pi, mid + 1, in_r, idx);
    Some(Box::new(Node { val: root_val, left, right }))
}

fn main() {
    let preorder: Vec<String> = ["a", "b", "d", "e", "c", "f", "g"]
        .iter().map(|s| s.to_string()).collect();
    let inorder: Vec<String> = ["d", "b", "e", "a", "f", "c", "g"]
        .iter().map(|s| s.to_string()).collect();

    let mut idx = HashMap::new();
    for (i, v) in inorder.iter().enumerate() {
        idx.insert(v.clone(), i as i32);
    }
    let mut pi = 0usize;
    let root = build(&preorder, &mut pi, 0, inorder.len() as i32 - 1, &idx);

    // Print level-order: a b c d e f g
    let mut out: Vec<String> = Vec::new();
    let mut q: VecDeque<&Box<Node>> = VecDeque::new();
    if let Some(ref r) = root {
        q.push_back(r);
    }
    while let Some(n) = q.pop_front() {
        out.push(n.val.clone());
        if let Some(ref l) = n.left {
            q.push_back(l);
        }
        if let Some(ref rr) = n.right {
            q.push_back(rr);
        }
    }
    println!("{}", out.join(" "));
}
