// Second largest in BST: walk right to largest; second largest is parent of
// largest (if no left subtree) else max of largest's left subtree. O(h) time, O(1) space.
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

fn second_largest(root: &Node) -> i32 {
    let mut parent: Option<&Node> = None;
    let mut cur = root;
    while let Some(ref r) = cur.right {
        parent = Some(cur);
        cur = r;
    }
    if let Some(ref l) = cur.left {
        let mut cur2 = l.as_ref();
        while let Some(ref r) = cur2.right {
            cur2 = r;
        }
        return cur2.val;
    }
    parent.unwrap().val
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for v in [5, 3, 8, 2, 4, 7, 9] {
        root = insert(root, v);
    }
    println!("{}", second_largest(root.as_ref().unwrap()));
}
