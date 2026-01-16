// DFS tracking depth; record the node value seen at maximum depth. Time O(n), Space O(h).
struct Node {
    val: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: &str) -> Box<Node> {
        Box::new(Node { val: val.to_string(), left: None, right: None })
    }
}

fn dfs(node: &Option<Box<Node>>, depth: i32, max_depth: &mut i32, deepest: &mut String) {
    if let Some(n) = node {
        if depth > *max_depth {
            *max_depth = depth;
            *deepest = n.val.clone();
        }
        dfs(&n.left, depth + 1, max_depth, deepest);
        dfs(&n.right, depth + 1, max_depth, deepest);
    }
}

fn deepest_node(root: &Option<Box<Node>>) -> String {
    let mut max_depth = -1;
    let mut deepest = String::new();
    dfs(root, 0, &mut max_depth, &mut deepest);
    deepest
}

fn main() {
    let mut a = Node::new("a");
    let mut b = Node::new("b");
    b.left = Some(Node::new("d"));
    a.left = Some(b);
    a.right = Some(Node::new("c"));
    let root = Some(a);
    println!("{}", deepest_node(&root));
}
