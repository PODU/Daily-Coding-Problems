// Merge two binary trees recursively (sum overlaps, keep lone nodes); print merged tree level-order skipping nulls.
// Time: O(n), Space: O(n).
use std::collections::VecDeque;

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn merge(a: Link, b: Link) -> Link {
    match (a, b) {
        (None, b) => b,
        (a, None) => a,
        (Some(x), Some(y)) => {
            Some(Box::new(Node {
                val: x.val + y.val,
                left: merge(x.left, y.left),
                right: merge(x.right, y.right),
            }))
        }
    }
}

fn main() {
    let t1 = node(
        1,
        node(3, node(5, None, None), None),
        node(2, None, None),
    );
    let t2 = node(
        2,
        node(1, None, node(4, None, None)),
        node(3, None, node(7, None, None)),
    );

    let m = merge(t1, t2);

    let mut level: VecDeque<&Box<Node>> = VecDeque::new();
    if let Some(ref root) = m {
        level.push_back(root);
    }
    while !level.is_empty() {
        let mut parts: Vec<String> = Vec::new();
        let mut next: VecDeque<&Box<Node>> = VecDeque::new();
        for c in &level {
            parts.push(c.val.to_string());
            if let Some(ref l) = c.left {
                next.push_back(l);
            }
            if let Some(ref r) = c.right {
                next.push_back(r);
            }
        }
        println!("{}", parts.join(" "));
        level = next;
    }
}
