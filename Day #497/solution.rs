// Convert binary tree to FULL binary tree by removing nodes with exactly one child.
// Post-order recursion: a one-child node is replaced by its processed child.
// Time: O(n), Space: O(h) recursion.

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

fn full_tree(node: Option<Box<Node>>) -> Option<Box<Node>> {
    match node {
        None => None,
        Some(mut n) => {
            n.left = full_tree(n.left.take());
            n.right = full_tree(n.right.take());
            match (n.left.is_some(), n.right.is_some()) {
                (true, false) => n.left.take(),
                (false, true) => n.right.take(),
                _ => Some(n),
            }
        }
    }
}

fn preorder(node: &Option<Box<Node>>, out: &mut Vec<String>) {
    if let Some(n) = node {
        out.push(n.val.to_string());
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let mut root = Node::new(0);
    root.left = Some(Node::new(1));
    root.right = Some(Node::new(2));
    root.left.as_mut().unwrap().left = Some(Node::new(3));
    root.left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Node::new(5));
    root.right.as_mut().unwrap().right = Some(Node::new(4));
    root.right.as_mut().unwrap().right.as_mut().unwrap().left = Some(Node::new(6));
    root.right.as_mut().unwrap().right.as_mut().unwrap().right = Some(Node::new(7));

    let result = full_tree(Some(root));
    let mut out = Vec::new();
    preorder(&result, &mut out);
    println!("{}", out.join(" "));
}
