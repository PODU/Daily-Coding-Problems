// Day 1036: Reconstruct BST from postorder traversal.
// Approach: walk postorder in reverse (root,right,left) using value bounds.
// Time: O(n), Space: O(h) recursion.
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

fn print_sideways(n: &Option<Box<Node>>, depth: usize) {
    if let Some(node) = n {
        print_sideways(&node.right, depth + 1);
        println!("{}{}", " ".repeat(depth * 4), node.val);
        print_sideways(&node.left, depth + 1);
    }
}

fn main() {
    let post: Vec<i64> = vec![2, 4, 3, 8, 7, 5];
    let mut idx: isize = post.len() as isize - 1;
    let root = build(&post, &mut idx, i64::MIN);
    println!("Reconstructed BST (rotated 90 deg, root=5):");
    print_sideways(&root, 0);
}
