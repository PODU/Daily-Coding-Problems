// Count unival subtrees via post-order DFS; node is unival if both children unival and values match.
// Time: O(n), Space: O(h) recursion.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// returns true if subtree is unival; increments count.
fn dfs(node: &Option<Box<Node>>, count: &mut i32) -> bool {
    match node {
        None => true,
        Some(n) => {
            let l = dfs(&n.left, count);
            let r = dfs(&n.right, count);
            if !l || !r {
                return false;
            }
            if let Some(c) = &n.left {
                if c.val != n.val {
                    return false;
                }
            }
            if let Some(c) = &n.right {
                if c.val != n.val {
                    return false;
                }
            }
            *count += 1;
            true
        }
    }
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
    let mut count = 0;
    dfs(&root, &mut count);
    println!("{}", count);
}
