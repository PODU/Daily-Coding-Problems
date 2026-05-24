// Merge two binary trees by summing overlapping nodes; recurse and reuse the
// non-null subtree when only one side exists. Time O(n), Space O(h).
use std::collections::VecDeque;

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

fn merge(a: Link, b: Link) -> Link {
    match (a, b) {
        (None, b) => b,
        (a, None) => a,
        (Some(mut na), Some(nb)) => {
            na.val += nb.val;
            na.left = merge(na.left.take(), nb.left);
            na.right = merge(na.right.take(), nb.right);
            Some(na)
        }
    }
}

fn main() {
    let mut t1 = Node::new(1);
    t1.left = Some(Node::new(3));
    t1.right = Some(Node::new(2));
    t1.left.as_mut().unwrap().left = Some(Node::new(5));

    let mut t2 = Node::new(2);
    t2.left = Some(Node::new(1));
    t2.right = Some(Node::new(3));
    t2.left.as_mut().unwrap().right = Some(Node::new(4));
    t2.right.as_mut().unwrap().right = Some(Node::new(7));

    let m = merge(Some(t1), Some(t2));

    let mut out: Vec<String> = Vec::new();
    let mut q: VecDeque<Link> = VecDeque::new();
    q.push_back(m);
    while let Some(cur) = q.pop_front() {
        match cur {
            Some(node) => {
                out.push(node.val.to_string());
                q.push_back(node.left);
                q.push_back(node.right);
            }
            None => out.push("null".to_string()),
        }
    }
    while out.last().map_or(false, |s| s == "null") {
        out.pop();
    }

    println!("{}", out.join(" "));
}
