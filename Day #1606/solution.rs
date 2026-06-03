// Day 1606: Check if a binary tree is height-balanced.
// Bottom-up recursion returning height, -1 if unbalanced. Time O(n), Space O(h).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn check(node: &Link) -> i32 {
    // height, or -1 if unbalanced
    match node {
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

fn is_balanced(root: &Link) -> bool {
    check(root) != -1
}

fn main() {
    let root = Some(Node::new(
        1,
        Some(Node::new(2, Some(Node::new(4, None, None)), None)),
        Some(Node::new(3, None, None)),
    ));
    println!("{}", if is_balanced(&root) { "true" } else { "false" }); // true
}
