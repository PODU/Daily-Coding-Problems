// Day 258: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing the output order on alternate levels.
// Time: O(n), Space: O(n).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

impl TreeNode {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

fn boustrophedon(root: &Link) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    if root.is_none() {
        return out;
    }
    let mut queue: Vec<Rc<RefCell<TreeNode>>> = vec![root.clone().unwrap()];
    let mut left_to_right = true;
    while !queue.is_empty() {
        let sz = queue.len();
        let mut level = vec![0; sz];
        let mut next: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
        for i in 0..sz {
            let node = queue[i].borrow();
            let idx = if left_to_right { i } else { sz - 1 - i };
            level[idx] = node.val;
            if let Some(l) = &node.left {
                next.push(l.clone());
            }
            if let Some(r) = &node.right {
                next.push(r.clone());
            }
        }
        out.extend(level);
        queue = next;
        left_to_right = !left_to_right;
    }
    out
}

fn main() {
    let root = TreeNode::new(
        1,
        TreeNode::new(2, TreeNode::new(4, None, None), TreeNode::new(5, None, None)),
        TreeNode::new(3, TreeNode::new(6, None, None), TreeNode::new(7, None, None)),
    );
    let res = boustrophedon(&root);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", ")); // [1, 3, 2, 4, 5, 6, 7]
}
