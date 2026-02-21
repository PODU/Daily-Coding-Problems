// Day 1112 - Minimum root-to-leaf path sum (return the path)
// Approach: DFS, track best leaf path by sum. Time: O(n), Space: O(h).

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(node: &Option<Box<Node>>, path: &mut Vec<i32>, s: i32, best: &mut (i32, Vec<i32>)) {
    if let Some(n) = node {
        path.push(n.val);
        let s = s + n.val;
        if n.left.is_none() && n.right.is_none() {
            if s < best.0 {
                best.0 = s;
                best.1 = path.clone();
            }
        } else {
            dfs(&n.left, path, s, best);
            dfs(&n.right, path, s, best);
        }
        path.pop();
    }
}

fn leaf(v: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
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
    let mut best = (i32::MAX, Vec::new());
    dfs(&root, &mut Vec::new(), 0, &mut best);
    let parts: Vec<String> = best.1.iter().map(|v| v.to_string()).collect();
    println!("[{}], which has sum {}", parts.join(", "), best.0);
}
