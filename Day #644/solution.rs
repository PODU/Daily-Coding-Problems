// Day 644: Count unival subtrees (all nodes share one value).
// Approach: post-order DFS; a node is unival iff both children are unival and
// their values match the node's. Count as we recurse.
// Time: O(n), Space: O(h).
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn dfs(node: &Option<Box<Node>>, count: &mut i32) -> bool {
    match node {
        None => true,
        Some(n) => {
            let left = dfs(&n.left, count);
            let right = dfs(&n.right, count);
            if !left || !right {
                return false;
            }
            if let Some(l) = &n.left {
                if l.val != n.val {
                    return false;
                }
            }
            if let Some(r) = &n.right {
                if r.val != n.val {
                    return false;
                }
            }
            *count += 1;
            true
        }
    }
}

fn count_unival(root: &Option<Box<Node>>) -> i32 {
    let mut count = 0;
    dfs(root, &mut count);
    count
}

fn leaf(v: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn main() {
    let root = Some(Box::new(Node {
        val: 0,
        left: leaf(1),
        right: Some(Box::new(Node {
            val: 0,
            left: Some(Box::new(Node { val: 1, left: leaf(1), right: leaf(1) })),
            right: leaf(0),
        })),
    }));
    println!("{}", count_unival(&root)); // 5
}
