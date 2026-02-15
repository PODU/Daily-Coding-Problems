// Postorder DFS: each node returns val+max(0,L,R) upward; global best = val+max(0,L)+max(0,R); O(n) time O(h) space
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn leaf(val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode { val, left: None, right: None }))
    }
    fn node(val: i32, left: Link, right: Link) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode { val, left, right }))
    }
}

fn dfs(node: &Link, best: &mut i32) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let n = n.borrow();
            let l = 0_i32.max(dfs(&n.left, best));
            let r = 0_i32.max(dfs(&n.right, best));
            *best = (*best).max(n.val + l + r);
            n.val + l.max(r)
        }
    }
}

fn max_path_sum(root: &Link) -> i32 {
    let mut best = i32::MIN;
    dfs(root, &mut best);
    best
}

fn main() {
    //       -10
    //       /  \
    //      9    20
    //          /  \
    //         15   7
    let root1 = TreeNode::node(
        -10,
        Some(TreeNode::leaf(9)),
        Some(TreeNode::node(
            20,
            Some(TreeNode::leaf(15)),
            Some(TreeNode::leaf(7)),
        )),
    );
    println!("Max path sum: {}", max_path_sum(&Some(root1)));

    //    1
    //   / \
    //  2   3
    let root2 = TreeNode::node(1, Some(TreeNode::leaf(2)), Some(TreeNode::leaf(3)));
    println!("Max path sum: {}", max_path_sum(&Some(root2)));
}
