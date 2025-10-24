// Day 484: Second largest node in a BST.
// O(h): walk right to largest; second largest is its parent, or max of largest's left subtree.
// Time O(h), Space O(1).
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Box::new(Node { val: v, left: None, right: None })),
        Some(mut n) => {
            if v < n.val {
                n.left = insert(n.left.take(), v);
            } else {
                n.right = insert(n.right.take(), v);
            }
            Some(n)
        }
    }
}

fn second_largest(root: &Option<Box<Node>>) -> Option<i32> {
    let r = root.as_ref()?;
    if r.left.is_none() && r.right.is_none() {
        return None;
    }
    let mut cur = r;
    let mut parent: Option<&Box<Node>> = None;
    while let Some(right) = cur.right.as_ref() {
        parent = Some(cur);
        cur = right;
    }
    if let Some(left) = cur.left.as_ref() {
        let mut m = left;
        while let Some(right) = m.right.as_ref() {
            m = right;
        }
        return Some(m.val);
    }
    parent.map(|p| p.val)
}

fn main() {
    let mut root = None;
    for v in [5, 3, 8, 2, 4, 7, 10] {
        root = insert(root, v);
    }
    println!("{}", second_largest(&root).unwrap()); // 8
}
