// Day 954: count unival subtrees (all nodes in subtree share one value).
// Post-order DFS, returning whether the subtree is unival. Time O(n), Space O(h).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn dfs(n: &Link, count: &mut i32) -> bool {
    match n {
        None => true,
        Some(node) => {
            let l = dfs(&node.left, count);
            let r = dfs(&node.right, count);
            if !l || !r {
                return false;
            }
            if let Some(c) = &node.left {
                if c.val != node.val {
                    return false;
                }
            }
            if let Some(c) = &node.right {
                if c.val != node.val {
                    return false;
                }
            }
            *count += 1;
            true
        }
    }
}

fn count_unival(root: &Link) -> i32 {
    let mut c = 0;
    dfs(root, &mut c);
    c
}

fn main() {
    let root = node(
        0,
        node(1, None, None),
        node(
            0,
            node(1, node(1, None, None), node(1, None, None)),
            node(0, None, None),
        ),
    );
    println!("{}", count_unival(&root)); // 5
}
