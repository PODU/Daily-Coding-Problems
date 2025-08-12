// Day 110: Root-to-leaf paths via DFS backtracking. O(n) nodes, O(h) stack.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn dfs(n: &Node, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    path.push(n.val);
    if n.left.is_none() && n.right.is_none() {
        res.push(path.clone());
    } else {
        if let Some(ref l) = n.left {
            dfs(l, path, res);
        }
        if let Some(ref r) = n.right {
            dfs(r, path, res);
        }
    }
    path.pop();
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
    let mut res = Vec::new();
    dfs(&root, &mut Vec::new(), &mut res);
    println!("{:?}", res);
}
