// Prune subtrees that contain only 0s via post-order recursion. O(n) time, O(h) stack.

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn prune(root: Link) -> Link {
    match root {
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

fn preorder(r: &Link, out: &mut Vec<String>) {
    if let Some(n) = r {
        out.push(n.val.to_string());
        preorder(&n.left, out);
        preorder(&n.right, out);
    }
}

fn main() {
    let root = node(
        0,
        node(1, None, None),
        node(
            0,
            node(1, node(0, None, None), node(0, None, None)),
            node(0, None, None),
        ),
    );
    let pruned = prune(root);
    let mut out = Vec::new();
    preorder(&pruned, &mut out);
    println!("preorder: {}", out.join(" ")); // 0 1 0 1
}
