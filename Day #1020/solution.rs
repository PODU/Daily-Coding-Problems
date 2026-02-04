// BST two-sum: in-order traversal -> sorted vec, two-pointer scan.
// O(n) time, O(n) space.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn inorder(r: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(n) = r {
        inorder(&n.left, out);
        out.push(n.val);
        inorder(&n.right, out);
    }
}

fn find_pair(root: &Option<Box<Node>>, k: i32) -> Option<(i32, i32)> {
    let mut v = Vec::new();
    inorder(root, &mut v);
    let (mut i, mut j) = (0usize, v.len().wrapping_sub(1));
    while i < j {
        let s = v[i] + v[j];
        if s == k {
            return Some((v[i], v[j]));
        }
        if s < k {
            i += 1;
        } else {
            j -= 1;
        }
    }
    None
}

fn main() {
    let mut root = Node::new(10);
    root.left = Some(Node::new(5));
    let mut right = Node::new(15);
    right.left = Some(Node::new(11));
    right.right = Some(Node::new(15));
    root.right = Some(right);
    let tree = Some(root);
    if let Some((a, b)) = find_pair(&tree, 20) {
        println!("{} and {}", a, b);
    }
}
