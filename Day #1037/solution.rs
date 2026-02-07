// Sorted array -> height-balanced BST: recursively pick middle as root.
// Time: O(n), Space: O(log n) recursion.

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn build(a: &[i32], lo: i32, hi: i32) -> Option<Box<Node>> {
    if lo > hi {
        return None;
    }
    let mid = lo + (hi - lo) / 2;
    let mut root = Box::new(Node {
        val: a[mid as usize],
        left: None,
        right: None,
    });
    root.left = build(a, lo, mid - 1);
    root.right = build(a, mid + 1, hi);
    Some(root)
}

fn print_rotated(node: &Option<Box<Node>>, depth: usize) {
    if let Some(n) = node {
        print_rotated(&n.right, depth + 1);
        println!("{}{}", " ".repeat(depth * 4), n.val);
        print_rotated(&n.left, depth + 1);
    }
}

fn inorder(node: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder(&n.left, out);
        out.push(n.val);
        inorder(&n.right, out);
    }
}

fn main() {
    let a = [-10, -3, 0, 5, 9];
    let root = build(&a, 0, a.len() as i32 - 1);
    println!("Height-balanced BST (rotated 90 deg):");
    print_rotated(&root, 0);
    let mut io: Vec<i32> = Vec::new();
    inorder(&root, &mut io);
    let parts: Vec<String> = io.iter().map(|x| x.to_string()).collect();
    println!("In-order: {}", parts.join(" "));
}
