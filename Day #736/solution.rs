// Count nodes in a complete binary tree.
// Compare left/right spine heights: full subtree -> 2^h-1, else recurse.
// Time: O(log^2 n), Space: O(log n).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn left_height(mut n: &Link) -> i32 {
    let mut h = 0;
    while let Some(b) = n {
        h += 1;
        n = &b.left;
    }
    h
}

fn right_height(mut n: &Link) -> i32 {
    let mut h = 0;
    while let Some(b) = n {
        h += 1;
        n = &b.right;
    }
    h
}

fn count_nodes(root: &Link) -> i32 {
    match root {
        None => 0,
        Some(b) => {
            let lh = left_height(root);
            let rh = right_height(root);
            if lh == rh {
                (1 << lh) - 1
            } else {
                1 + count_nodes(&b.left) + count_nodes(&b.right)
            }
        }
    }
}

fn main() {
    let root = node(
        1,
        node(2, node(4, None, None), node(5, None, None)),
        node(3, node(6, None, None), None),
    );
    println!("{}", count_nodes(&root)); // 6
}
