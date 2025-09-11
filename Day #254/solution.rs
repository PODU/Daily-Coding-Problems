// Prune to full binary tree: post-order DFS; a node with exactly one child is
// replaced by that child. Returns new root. Time O(n), Space O(h) recursion.
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

fn prune(node: Link) -> Link {
    match node {
        None => None,
        Some(mut n) => {
            n.left = prune(n.left.take());
            n.right = prune(n.right.take());
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
    let mut root = Node::new(0);
    root.left = Some(Node::new(1));
    root.right = Some(Node::new(2));
    root.left.as_mut().unwrap().left = Some(Node::new(3));
    root.left.as_mut().unwrap().left.as_mut().unwrap().right = Some(Node::new(5));
    root.right.as_mut().unwrap().right = Some(Node::new(4));
    let r = root.right.as_mut().unwrap().right.as_mut().unwrap();
    r.left = Some(Node::new(6));
    r.right = Some(Node::new(7));

    let pruned = prune(Some(root));
    let mut pre = Vec::new();
    preorder(&pruned, &mut pre);
    let parts: Vec<String> = pre.iter().map(|v| v.to_string()).collect();
    println!("Preorder after pruning: {}", parts.join(" "));
}
