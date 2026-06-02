// Reconstruct binary tree from preorder+inorder using inorder index hashmap
// and a moving preorder pointer. Time O(n), Space O(n).
use std::collections::HashMap;

struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(pre: &[char], idx: &HashMap<char, i32>, pre_idx: &mut usize, in_l: i32, in_r: i32) -> Option<Box<Node>> {
    if in_l > in_r {
        return None;
    }
    let root_val = pre[*pre_idx];
    *pre_idx += 1;
    let mid = idx[&root_val];
    let left = build(pre, idx, pre_idx, in_l, mid - 1);
    let right = build(pre, idx, pre_idx, mid + 1, in_r);
    Some(Box::new(Node { val: root_val, left, right }))
}

fn preorder_t(n: &Option<Box<Node>>, out: &mut Vec<char>) {
    if let Some(node) = n {
        out.push(node.val);
        preorder_t(&node.left, out);
        preorder_t(&node.right, out);
    }
}
fn inorder_t(n: &Option<Box<Node>>, out: &mut Vec<char>) {
    if let Some(node) = n {
        inorder_t(&node.left, out);
        out.push(node.val);
        inorder_t(&node.right, out);
    }
}
fn postorder_t(n: &Option<Box<Node>>, out: &mut Vec<char>) {
    if let Some(node) = n {
        postorder_t(&node.left, out);
        postorder_t(&node.right, out);
        out.push(node.val);
    }
}

fn join(v: &[char]) -> String {
    v.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let pre = vec!['a', 'b', 'd', 'e', 'c', 'f', 'g'];
    let ino = vec!['d', 'b', 'e', 'a', 'f', 'c', 'g'];
    let mut idx = HashMap::new();
    for (i, &c) in ino.iter().enumerate() {
        idx.insert(c, i as i32);
    }
    let mut pre_idx = 0usize;
    let root = build(&pre, &idx, &mut pre_idx, 0, ino.len() as i32 - 1);

    let (mut po, mut pr, mut io) = (Vec::new(), Vec::new(), Vec::new());
    postorder_t(&root, &mut po);
    preorder_t(&root, &mut pr);
    inorder_t(&root, &mut io);
    println!("postorder: {}", join(&po));
    println!("preorder:  {}", join(&pr));
    println!("inorder:   {}", join(&io));
}
