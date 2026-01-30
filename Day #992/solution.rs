// Day 992: Second largest node in a BST.
// The largest is the rightmost node; the 2nd largest is its left subtree's max,
// or its parent if it has no left child. O(h) time, O(1) extra space.
type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn insert(root: Link, val: i32) -> Link {
    match root {
        None => Some(Box::new(Node { val, left: None, right: None })),
        Some(mut n) => {
            if val < n.val {
                n.left = insert(n.left.take(), val);
            } else {
                n.right = insert(n.right.take(), val);
            }
            Some(n)
        }
    }
}

fn second_largest(root: &Node) -> i32 {
    let mut cur = root;
    let mut parent: Option<&Node> = None;
    while let Some(r) = cur.right.as_deref() {
        parent = Some(cur);
        cur = r;
    }
    if let Some(mut l) = cur.left.as_deref() {
        while let Some(r) = l.right.as_deref() {
            l = r;
        }
        return l.val;
    }
    parent.unwrap().val
}

fn main() {
    let mut root: Link = None;
    for v in [5, 3, 8, 2, 4, 7, 9] {
        root = insert(root, v);
    }
    println!("{}", second_largest(root.as_ref().unwrap())); // 8
}
