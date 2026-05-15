// Count unival subtrees (all nodes in subtree share one value).
// Approach: post-order DFS; a node is unival iff children are unival and equal in value.
// Time: O(n), Space: O(h).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn dfs(node: &Link, count: &mut i32) -> bool {
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

fn count_unival_subtrees(root: &Link) -> i32 {
    let mut count = 0;
    dfs(root, &mut count);
    count
}

fn main() {
    let root = Some(Node::new(0,
        Some(Node::new(1, None, None)),
        Some(Node::new(0,
            Some(Node::new(1, Some(Node::new(1, None, None)), Some(Node::new(1, None, None)))),
            Some(Node::new(0, None, None))))));
    println!("{}", count_unival_subtrees(&root)); // 5
}
