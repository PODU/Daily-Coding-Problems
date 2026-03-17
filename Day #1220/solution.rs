// Merge two binary trees recursively (sum overlaps, keep lone nodes). O(min(n1,n2)) time.
// Serialize merged tree as BFS level-order with trailing nulls trimmed.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Rc::new(RefCell::new(Node { val, left, right })))
}

fn merge(a: &Link, b: &Link) -> Link {
    match (a, b) {
        (None, _) => b.clone(),
        (_, None) => a.clone(),
        (Some(x), Some(y)) => {
            let xb = x.borrow();
            let yb = y.borrow();
            node(
                xb.val + yb.val,
                merge(&xb.left, &yb.left),
                merge(&xb.right, &yb.right),
            )
        }
    }
}

fn serialize(root: &Link) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut q: VecDeque<Link> = VecDeque::new();
    q.push_back(root.clone());
    while let Some(cur) = q.pop_front() {
        match cur {
            None => out.push("null".to_string()),
            Some(n) => {
                let nb = n.borrow();
                out.push(nb.val.to_string());
                q.push_back(nb.left.clone());
                q.push_back(nb.right.clone());
            }
        }
    }
    while out.last().map_or(false, |s| s == "null") {
        out.pop();
    }
    format!("[{}]", out.join(", "))
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
    println!("{}", serialize(&merge(&t1, &t2)));
}
