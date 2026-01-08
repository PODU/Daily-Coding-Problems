// Largest BST subtree via post-order returning (isBST, size, min, max). Time O(n), Space O(h).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

fn leaf(v: i32) -> Link {
    Some(Box::new(Node { val: v, left: None, right: None }))
}
fn node(v: i32, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val: v, left, right }))
}

struct Info {
    is_bst: bool,
    size: i32,
    mn: i32,
    mx: i32,
}

fn dfs(n: &Link, best: &mut i32) -> Info {
    match n {
        None => Info { is_bst: true, size: 0, mn: i32::MAX, mx: i32::MIN },
        Some(node) => {
            let l = dfs(&node.left, best);
            let r = dfs(&node.right, best);
            if l.is_bst && r.is_bst && l.mx < node.val && node.val < r.mn {
                let sz = l.size + r.size + 1;
                if sz > *best {
                    *best = sz;
                }
                Info {
                    is_bst: true,
                    size: sz,
                    mn: node.val.min(l.mn),
                    mx: node.val.max(r.mx),
                }
            } else {
                Info { is_bst: false, size: 0, mn: 0, mx: 0 }
            }
        }
    }
}

fn largest_bst(root: &Link) -> i32 {
    let mut best = 0;
    dfs(root, &mut best);
    best
}

fn main() {
    let root = node(10, node(5, leaf(1), leaf(8)), node(15, None, leaf(7)));
    println!("{}", largest_bst(&root));
}
