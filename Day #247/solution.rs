// Height-balanced binary tree check.
// Bottom-up DFS returning subtree height, or -1 if unbalanced. O(n) time, O(h) space.

struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new() -> Box<Node> {
        Box::new(Node { left: None, right: None })
    }
}

fn check(root: &Option<Box<Node>>) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let l = check(&n.left);
            if l == -1 {
                return -1;
            }
            let r = check(&n.right);
            if r == -1 {
                return -1;
            }
            if (l - r).abs() > 1 {
                return -1;
            }
            l.max(r) + 1
        }
    }
}

fn is_balanced(root: &Option<Box<Node>>) -> bool {
    check(root) != -1
}

fn main() {
    let mut a = Node::new();
    let mut left = Node::new();
    left.left = Some(Node::new());
    a.left = Some(left);
    a.right = Some(Node::new());
    let a = Some(a);

    let mut b = Node::new();
    let mut bl = Node::new();
    bl.left = Some(Node::new());
    b.left = Some(bl);
    let b = Some(b);

    println!("Balanced tree: {}", is_balanced(&a));
    println!("Unbalanced tree: {}", is_balanced(&b));
}
