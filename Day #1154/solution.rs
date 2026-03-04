// Day 1154: Minimum root-to-leaf path sum.
// DFS tracking running sum/path, keep best at leaves. O(n) time, O(h) space.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn leaf(v: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn dfs(node: &Option<Box<Node>>, path: &mut Vec<i32>, sum: i32, best: &mut i32, best_path: &mut Vec<i32>) {
    if let Some(n) = node {
        path.push(n.val);
        let s = sum + n.val;
        if n.left.is_none() && n.right.is_none() {
            if s < *best {
                *best = s;
                *best_path = path.clone();
            }
        } else {
            dfs(&n.left, path, s, best, best_path);
            dfs(&n.right, path, s, best, best_path);
        }
        path.pop();
    }
}

fn main() {
    let root = Some(Box::new(Node {
        val: 10,
        left: Some(Box::new(Node { val: 5, left: None, right: leaf(2) })),
        right: Some(Box::new(Node {
            val: 5,
            left: None,
            right: Some(Box::new(Node { val: 1, left: leaf(-1), right: None })),
        })),
    }));
    let mut best = i32::MAX;
    let mut best_path = Vec::new();
    dfs(&root, &mut Vec::new(), 0, &mut best, &mut best_path);
    let parts: Vec<String> = best_path.iter().map(|v| v.to_string()).collect();
    println!("[{}], which has sum {}", parts.join(", "), best);
    // [10, 5, 1, -1], which has sum 15
}
