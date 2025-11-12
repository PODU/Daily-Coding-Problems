// Day 587: Binary tree root-to-leaf paths.
// Approach: DFS, accumulate current path, record at leaves. Time O(n), Space O(h).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left, right }))
    }
}

fn dfs(node: &Link, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if let Some(n) = node {
        let n = n.borrow();
        path.push(n.val);
        if n.left.is_none() && n.right.is_none() {
            res.push(path.clone());
        } else {
            dfs(&n.left, path, res);
            dfs(&n.right, path, res);
        }
        path.pop();
    }
}

fn root_to_leaf_paths(root: &Link) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut path = Vec::new();
    dfs(root, &mut path, &mut res);
    res
}

fn main() {
    let root = Node::new(
        1,
        Some(Node::new(2, None, None)),
        Some(Node::new(
            3,
            Some(Node::new(4, None, None)),
            Some(Node::new(5, None, None)),
        )),
    );

    let paths = root_to_leaf_paths(&Some(root));
    let parts: Vec<String> = paths
        .iter()
        .map(|p| {
            let nums: Vec<String> = p.iter().map(|v| v.to_string()).collect();
            format!("[{}]", nums.join(", "))
        })
        .collect();
    println!("[{}]", parts.join(", "));
}
