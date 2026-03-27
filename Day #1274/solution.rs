// Day 1274: Prune a 0/1 binary tree, removing every subtree that contains only 0s.
// Post-order recursion: a node survives iff it is 1 or has a surviving child. O(n).
type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Box::new(Node { val, left, right }))
    }
}

fn prune(node: Link) -> Link {
    match node {
        None => None,
        Some(mut n) => {
            n.left = prune(n.left.take());
            n.right = prune(n.right.take());
            if n.val == 0 && n.left.is_none() && n.right.is_none() {
                None
            } else {
                Some(n)
            }
        }
    }
}

fn serialize(node: &Link) -> String {
    match node {
        None => "null".to_string(),
        Some(n) => format!("{}({},{})", n.val, serialize(&n.left), serialize(&n.right)),
    }
}

fn main() {
    let root = Node::new(
        0,
        Node::new(1, None, None),
        Node::new(
            0,
            Node::new(1, Node::new(0, None, None), Node::new(0, None, None)),
            Node::new(0, None, None),
        ),
    );
    let pruned = prune(root);
    // Pruned tree: 0(1(null,null),0(1(null,null),null))
    println!("{}", serialize(&pruned));
}
