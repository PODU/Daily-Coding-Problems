// Reconstruct BST from postorder. Process postorder from the right with an
// upper-bound recursion (reverse postorder = root,right,left). Time O(n), Space O(h).

struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(post: &[i64], idx: &mut isize, bound: i64) -> Option<Box<Node>> {
    if *idx < 0 || post[*idx as usize] < bound {
        return None;
    }
    let val = post[*idx as usize];
    *idx -= 1;
    let right = build(post, idx, val);
    let left = build(post, idx, bound);
    Some(Box::new(Node { val, left, right }))
}

fn bst_from_postorder(post: &[i64]) -> Option<Box<Node>> {
    let mut idx = post.len() as isize - 1;
    build(post, &mut idx, i64::MIN)
}

fn preorder(root: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(n) = root {
        out.push(n.val);
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let post: Vec<i64> = vec![2, 4, 3, 8, 7, 5];
    let root = bst_from_postorder(&post);
    let mut pre = Vec::new();
    preorder(&root, &mut pre);
    let parts: Vec<String> = pre.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
