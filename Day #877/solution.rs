// Subtree check: for each node of s, test identical-tree with t. Time O(m*n), Space O(h).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn leaf(v: i32) -> Link {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn node(v: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val: v, left, right }))
}

fn same_tree(a: &Link, b: &Link) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            x.val == y.val && same_tree(&x.left, &y.left) && same_tree(&x.right, &y.right)
        }
        _ => false,
    }
}

fn is_subtree(s: &Link, t: &Link) -> bool {
    match s {
        None => false,
        Some(node) => {
            same_tree(s, t) || is_subtree(&node.left, t) || is_subtree(&node.right, t)
        }
    }
}

fn main() {
    let s = node(3, node(4, leaf(1), leaf(2)), leaf(5));
    let t = node(4, leaf(1), leaf(2));
    println!("{}", is_subtree(&s, &t));
}
