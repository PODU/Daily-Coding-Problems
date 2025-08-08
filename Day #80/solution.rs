// Day 80: Return a deepest node of a binary tree via DFS tracking depth.
// Time O(n), Space O(h).
struct Node {
    val: char,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(node: &Option<Box<Node>>, depth: i32, best: &mut i32, res: &mut Option<char>) {
    if let Some(n) = node {
        if depth > *best {
            *best = depth;
            *res = Some(n.val);
        }
        dfs(&n.left, depth + 1, best, res);
        dfs(&n.right, depth + 1, best, res);
    }
}

fn deepest_node(root: &Option<Box<Node>>) -> Option<char> {
    let mut best = -1;
    let mut res = None;
    dfs(root, 0, &mut best, &mut res);
    res
}

fn main() {
    let a = Some(Box::new(Node {
        val: 'a',
        left: Some(Box::new(Node {
            val: 'b',
            left: Some(Box::new(Node { val: 'd', left: None, right: None })),
            right: None,
        })),
        right: Some(Box::new(Node { val: 'c', left: None, right: None })),
    }));
    println!("{}", deepest_node(&a).unwrap()); // d
}
