// Reconstruct BST from postorder: scan right-to-left (preorder of root,right,left)
// with an upper-bound recursion. Time O(n), space O(n).

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(post: &[i32], idx: &mut i32, bound: i32) -> Option<Box<Node>> {
    if *idx < 0 || post[*idx as usize] < bound {
        return None;
    }
    let val = post[*idx as usize];
    *idx -= 1;
    let mut root = Box::new(Node { val, left: None, right: None });
    root.right = build(post, idx, val);
    root.left = build(post, idx, bound);
    Some(root)
}

fn preorder(n: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(node) = n {
        out.push(node.val);
        preorder(&node.left, out);
        preorder(&node.right, out);
    }
}

fn inorder(n: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(node) = n {
        inorder(&node.left, out);
        out.push(node.val);
        inorder(&node.right, out);
    }
}

fn join(xs: &[i32]) -> String {
    xs.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    let post = vec![2, 4, 3, 8, 7, 5];
    let mut idx = post.len() as i32 - 1;
    let root = build(&post, &mut idx, i32::MIN);
    let mut pre = Vec::new();
    let mut ino = Vec::new();
    preorder(&root, &mut pre);
    inorder(&root, &mut ino);
    println!("preorder: {}", join(&pre));
    println!("inorder:  {}", join(&ino));
}
