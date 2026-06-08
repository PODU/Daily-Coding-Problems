// Root-to-leaf paths via DFS, carrying current path; record at leaves. Time O(n), Space O(h).
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

fn main() {
    let mut root = Node::new(1);
    root.left = Some(Node::new(2));
    let mut three = Node::new(3);
    three.left = Some(Node::new(4));
    three.right = Some(Node::new(5));
    root.right = Some(three);

    let mut res: Vec<Vec<i32>> = Vec::new();
    dfs(&Some(root), &mut Vec::new(), &mut res);

    let parts: Vec<String> = res
        .iter()
        .map(|p| {
            let nums: Vec<String> = p.iter().map(|v| v.to_string()).collect();
            format!("[{}]", nums.join(", "))
        })
        .collect();
    println!("[{}]", parts.join(", "));
}
