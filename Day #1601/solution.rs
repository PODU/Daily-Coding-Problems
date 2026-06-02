// Min root-to-leaf path sum via recursive DFS; leaf returns its val, internal node adds min of existing children.
// Reconstruct path by tracking the chosen child. Time O(n), space O(h).

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

// returns (min_sum, path) from node down to a leaf.
fn min_path(node: &Node) -> (i32, Vec<i32>) {
    if node.left.is_none() && node.right.is_none() {
        return (node.val, vec![node.val]);
    }
    let mut best: Option<(i32, Vec<i32>)> = None;
    for child in [&node.left, &node.right] {
        if let Some(c) = child {
            let (s, p) = min_path(c);
            if best.as_ref().map_or(true, |(bs, _)| s < *bs) {
                best = Some((s, p));
            }
        }
    }
    let (bs, bp) = best.unwrap();
    let mut path = vec![node.val];
    path.extend(bp);
    (node.val + bs, path)
}

fn main() {
    let mut root = Node::new(10);
    root.left = Some(Node::new(5));
    root.right = Some(Node::new(5));
    root.left.as_mut().unwrap().right = Some(Node::new(2));
    root.right.as_mut().unwrap().right = Some(Node::new(1));
    root.right.as_mut().unwrap().right.as_mut().unwrap().left = Some(Node::new(-1));

    let (sum, path) = min_path(&root);
    let parts: Vec<String> = path.iter().map(|v| v.to_string()).collect();
    println!("The minimum path is [{}], which has sum {}.", parts.join(", "), sum);
}
