// Max path sum between any two nodes via DFS returning best downward gain.
// Time O(n), Space O(h).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

fn gain(n: &Link, best: &mut i32) -> i32 {
    match n {
        None => 0,
        Some(rc) => {
            let nd = rc.borrow();
            let l = gain(&nd.left, best).max(0);
            let r = gain(&nd.right, best).max(0);
            *best = (*best).max(nd.val + l + r);
            nd.val + l.max(r)
        }
    }
}

fn max_path_sum(root: &Link) -> i32 {
    let mut best = i32::MIN;
    gain(root, &mut best);
    best
}

fn main() {
    let root = node(
        -10,
        node(9, None, None),
        node(20, node(15, None, None), node(7, None, None)),
    );
    println!("{}", max_path_sum(&root)); // 42
}
