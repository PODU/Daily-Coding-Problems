// Root-to-leaf paths via DFS, appending to the current path and recording it at each leaf.
// Time O(n) nodes (O(n*h) including path copies), Space O(h) recursion.

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn dfs(node: &Option<Box<Node>>, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
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

fn root_to_leaf_paths(root: &Option<Box<Node>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    dfs(root, &mut Vec::new(), &mut res);
    res
}

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Node::new(2));
    let mut three = Node::new(3);
    three.left = Some(Node::new(4));
    three.right = Some(Node::new(5));
    root.right = Some(three);
    let root = Some(root);
    println!("{:?}", root_to_leaf_paths(&root)); // [[1, 2], [1, 3, 4], [1, 3, 5]]
}
