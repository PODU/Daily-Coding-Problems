// Second largest in BST via parent-walk: find largest; if it has a left subtree,
// answer = max of that subtree, else answer = parent of largest. Time O(h), Space O(1).
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(v: i32) -> Box<Node> {
        Box::new(Node { val: v, left: None, right: None })
    }
}

fn insert(root: Option<Box<Node>>, v: i32) -> Option<Box<Node>> {
    match root {
        None => Some(Node::new(v)),
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

fn max_node(n: &Node) -> i32 {
    let mut cur = n;
    while let Some(ref r) = cur.right {
        cur = r;
    }
    cur.val
}

fn second_largest(root: &Node) -> i32 {
    let mut cur = root;
    let mut parent: Option<&Node> = None;
    while let Some(ref r) = cur.right {
        parent = Some(cur);
        cur = r;
    }
    if let Some(ref l) = cur.left {
        return max_node(l);
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
