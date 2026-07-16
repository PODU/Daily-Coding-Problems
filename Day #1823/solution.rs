// Convert to full binary tree by removing single-child nodes (post-order recursion).
// O(N) time, O(H) space.
type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    l: Link,
    r: Link,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, l: None, r: None })
    }
}

fn fullify(n: Link) -> Link {
    match n {
        None => None,
        Some(mut node) => {
            node.l = fullify(node.l.take());
            node.r = fullify(node.r.take());
            match (node.l.is_some(), node.r.is_some()) {
                (false, true) => node.r.take(),
                (true, false) => node.l.take(),
                _ => Some(node),
            }
        }
    }
}

fn serialize(n: &Link) -> String {
    match n {
        None => String::new(),
        Some(node) => {
            if node.l.is_none() && node.r.is_none() {
                node.val.to_string()
            } else {
                format!("{}({}, {})", node.val, serialize(&node.l), serialize(&node.r))
            }
        }
    }
}

fn main() {
    // Build the example tree.
    let n5 = Node::new(5);
    let mut n3 = Node::new(3);
    n3.r = Some(n5);
    let mut n1 = Node::new(1);
    n1.l = Some(n3);

    let n6 = Node::new(6);
    let n7 = Node::new(7);
    let mut n4 = Node::new(4);
    n4.l = Some(n6);
    n4.r = Some(n7);
    let mut n2 = Node::new(2);
    n2.r = Some(n4);

    let mut n0 = Node::new(0);
    n0.l = Some(n1);
    n0.r = Some(n2);

    let root = fullify(Some(n0));
    println!("{}", serialize(&root)); // 0(5, 4(6, 7))
}
