// Min root-to-leaf path sum via DFS, reconstructing the path. Time O(n), Space O(h).
struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i64) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn dfs(node: &Option<Box<Node>>, cur: &mut Vec<i64>, best: &mut i64, best_path: &mut Vec<i64>) {
    if let Some(n) = node {
        cur.push(n.val);
        if n.left.is_none() && n.right.is_none() {
            let s: i64 = cur.iter().sum();
            if s < *best {
                *best = s;
                *best_path = cur.clone();
            }
        } else {
            dfs(&n.left, cur, best, best_path);
            dfs(&n.right, cur, best, best_path);
        }
        cur.pop();
    }
}

fn main() {
    let mut root = Node::new(10);
    let mut left = Node::new(5);
    left.right = Some(Node::new(2));
    root.left = Some(left);
    let mut right = Node::new(5);
    let mut r1 = Node::new(1);
    r1.left = Some(Node::new(-1));
    right.right = Some(r1);
    root.right = Some(right);

    let root_opt = Some(root);
    let mut cur = Vec::new();
    let mut best = i64::MAX;
    let mut best_path = Vec::new();
    dfs(&root_opt, &mut cur, &mut best, &mut best_path);

    let parts: Vec<String> = best_path.iter().map(|x| x.to_string()).collect();
    println!("[{}], which has sum {}.", parts.join(", "), best);
}
