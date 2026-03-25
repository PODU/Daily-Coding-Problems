// Day 1263: All root-to-leaf paths in a binary tree.
// DFS carrying the current path. O(n) nodes visited, O(h) recursion + output size.
type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn dfs(node: &Link, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if let Some(n) = node {
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

fn root_to_leaf(root: &Link) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut path = Vec::new();
    dfs(root, &mut path, &mut res);
    res
}

fn main() {
    let root = Some(Node::new(
        1,
        Some(Node::new(2, None, None)),
        Some(Node::new(
            3,
            Some(Node::new(4, None, None)),
            Some(Node::new(5, None, None)),
        )),
    ));
    println!("{:?}", root_to_leaf(&root));
}
