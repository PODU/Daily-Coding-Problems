// Day 1172: Reconstruct a binary tree from pre-order and in-order traversals.
// Recursion with a hashmap of in-order positions; first pre-order element is the
// root, its in-order index splits left/right subtrees. Time O(N), Space O(N).
use std::collections::HashMap;

struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(pre: &[char], pi: &mut usize, lo: i32, hi: i32, pos: &HashMap<char, i32>) -> Option<Box<Node>> {
    if lo > hi {
        return None;
    }
    let root_val = pre[*pi];
    *pi += 1;
    let m = pos[&root_val];
    let left = build(pre, pi, lo, m - 1, pos);
    let right = build(pre, pi, m + 1, hi, pos);
    Some(Box::new(Node { val: root_val, left, right }))
}

fn reconstruct(pre: &[char], inorder: &[char]) -> Option<Box<Node>> {
    let mut pos = HashMap::new();
    for (i, &v) in inorder.iter().enumerate() {
        pos.insert(v, i as i32);
    }
    let mut pi = 0;
    build(pre, &mut pi, 0, inorder.len() as i32 - 1, &pos)
}

fn inorder_str(n: &Option<Box<Node>>, out: &mut String) {
    if let Some(node) = n {
        inorder_str(&node.left, out);
        out.push(node.val);
        inorder_str(&node.right, out);
    }
}

fn main() {
    let pre: Vec<char> = "abdecfg".chars().collect();
    let ino: Vec<char> = "dbeafcg".chars().collect();
    let root = reconstruct(&pre, &ino);
    let mut check = String::new();
    inorder_str(&root, &mut check);
    assert_eq!(check, "dbeafcg"); // verifies reconstruction
    println!("    a");
    println!("   / \\");
    println!("  b   c");
    println!(" / \\ / \\");
    println!("d  e f  g");
}
