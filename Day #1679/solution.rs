// Day 1679: Size of largest BST subtree. Post-order returns (isBST, size, min, max)
// per subtree, tracking the global best. Time O(n), Space O(h).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i64,
    left: Link,
    right: Link,
}

fn node(val: i64, left: Link, right: Link) -> Link {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

struct Info {
    is_bst: bool,
    size: i32,
    mn: i64,
    mx: i64,
}

fn dfs(n: &Link, best: &mut i32) -> Info {
    match n {
        None => Info { is_bst: true, size: 0, mn: i64::MAX, mx: i64::MIN },
        Some(rc) => {
            let b = rc.borrow();
            let l = dfs(&b.left, best);
            let r = dfs(&b.right, best);
            if l.is_bst && r.is_bst && l.mx < b.val && b.val < r.mn {
                let sz = l.size + r.size + 1;
                if sz > *best {
                    *best = sz;
                }
                Info { is_bst: true, size: sz, mn: b.val.min(l.mn), mx: b.val.max(r.mx) }
            } else {
                Info { is_bst: false, size: 0, mn: i64::MIN, mx: i64::MAX }
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
    let root = node(
        10,
        node(5, node(1, None, None), node(8, None, None)),
        node(15, None, node(7, None, None)),
    );
    println!("{}", largest_bst(&root)); // 3
}
