// Day 135: Minimum root-to-leaf path sum (with path reconstruction).
// DFS over the tree. O(n) time, O(h) recursion space.
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

fn min_path(r: &Option<Box<Node>>) -> (i32, Vec<i32>) {
    match r {
        None => (i32::MAX, vec![]),
        Some(n) => {
            if n.left.is_none() && n.right.is_none() {
                return (n.val, vec![n.val]);
            }
            let mut best_sum = i32::MAX;
            let mut best_path = vec![];
            for c in [&n.left, &n.right] {
                if c.is_none() {
                    continue;
                }
                let (s, p) = min_path(c);
                if s < best_sum {
                    best_sum = s;
                    best_path = p;
                }
            }
            let mut path = vec![n.val];
            path.extend(best_path);
            (best_sum + n.val, path)
        }
    }
}

fn main() {
    let mut root = Node::new(10);
    let mut l = Node::new(5);
    l.right = Some(Node::new(2));
    let mut r = Node::new(5);
    let mut r1 = Node::new(1);
    r1.left = Some(Node::new(-1));
    r.right = Some(r1);
    root.left = Some(l);
    root.right = Some(r);
    let tree = Some(root);

    let (total, path) = min_path(&tree);
    let parts: Vec<String> = path.iter().map(|v| v.to_string()).collect();
    println!("{} (path [{}])", total, parts.join(", "));
}
