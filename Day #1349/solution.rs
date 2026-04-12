// Day 1349: Reconstruct a BST from its postorder traversal.
// Consume postorder right-to-left with value bounds (right subtree before left). O(n) time, O(h) space.
struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(post: &[i64], idx: &mut i64, bound: i64) -> Option<Box<Node>> {
    if *idx < 0 || post[*idx as usize] < bound {
        return None;
    }
    let val = post[*idx as usize];
    *idx -= 1;
    let mut node = Box::new(Node { val, left: None, right: None });
    node.right = build(post, idx, val);
    node.left = build(post, idx, bound);
    Some(node)
}

fn preorder(n: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(node) = n {
        out.push(node.val);
        preorder(&node.left, out);
        preorder(&node.right, out);
    }
}

fn inorder(n: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(node) = n {
        inorder(&node.left, out);
        out.push(node.val);
        inorder(&node.right, out);
    }
}

fn main() {
    let post = vec![2i64, 4, 3, 8, 7, 5];
    let mut idx = post.len() as i64 - 1;
    let root = build(&post, &mut idx, i64::MIN);
    let mut pre = Vec::new();
    let mut ino = Vec::new();
    preorder(&root, &mut pre);
    inorder(&root, &mut ino);
    println!("Root: {}", root.as_ref().unwrap().val);
    let pj: Vec<String> = pre.iter().map(|x| x.to_string()).collect();
    let ij: Vec<String> = ino.iter().map(|x| x.to_string()).collect();
    println!("Preorder: {}", pj.join(" "));
    println!("Inorder: {}", ij.join(" "));
}
