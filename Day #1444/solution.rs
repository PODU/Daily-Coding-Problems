// Day 1444: Convert a binary tree to a full binary tree by removing every node
// with exactly one child (its single child is promoted up).
// Post-order recursion. Time O(n), Space O(h).
type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn to_full(node: Link) -> Link {
    match node {
        None => None,
        Some(mut n) => {
            n.left = to_full(n.left.take());
            n.right = to_full(n.right.take());
            match (n.left.is_some(), n.right.is_some()) {
                (true, false) => n.left.take(),
                (false, true) => n.right.take(),
                _ => Some(n),
            }
        }
    }
}

fn preorder(node: &Link, out: &mut Vec<i32>) {
    if let Some(n) = node {
        out.push(n.val);
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let mut n0 = Node::new(0);
    let mut n1 = Node::new(1);
    let mut n3 = Node::new(3);
    n3.right = Some(Node::new(5));
    n1.left = Some(n3);
    n0.left = Some(n1);

    let mut n2 = Node::new(2);
    let mut n4 = Node::new(4);
    n4.left = Some(Node::new(6));
    n4.right = Some(Node::new(7));
    n2.right = Some(n4);
    n0.right = Some(n2);

    let full = to_full(Some(n0));
    let mut out = Vec::new();
    preorder(&full, &mut out);
    let s: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("Preorder of full tree: {}", s.join(" ")); // 0 5 4 6 7
}
