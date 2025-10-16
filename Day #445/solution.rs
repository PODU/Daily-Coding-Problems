// Day 445: Prune a 0/1 binary tree, removing all-zero subtrees.
// Postorder recursion, O(n) time, O(h) space.

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

fn show(node: &Link, prefix: &str, tag: &str) {
    if let Some(n) = node {
        println!("{}{}{}", prefix, tag, n.val);
        let child = format!("{}  ", prefix);
        show(&n.left, &child, "L-- ");
        show(&n.right, &child, "R-- ");
    }
}

fn main() {
    let root = Node::new(
        0,
        Some(Node::new(1, None, None)),
        Some(Node::new(
            0,
            Some(Node::new(1, Some(Node::new(0, None, None)), Some(Node::new(0, None, None)))),
            Some(Node::new(0, None, None)),
        )),
    );
    let pruned = prune(Some(root));
    show(&pruned, "", "");
    // 0
    //   L-- 1
    //   R-- 0
    //     L-- 1
}
