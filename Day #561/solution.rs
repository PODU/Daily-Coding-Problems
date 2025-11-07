// Sorted array -> height-balanced BST: recurse on mid=(lo+hi)/2 as root.
// Time: O(N), Space: O(N) for nodes + O(log N) recursion.
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn build(a: &[i32], lo: i32, hi: i32) -> Link {
    if lo > hi {
        return None;
    }
    let mid = (lo + hi) / 2;
    let node = Rc::new(RefCell::new(TreeNode {
        val: a[mid as usize],
        left: build(a, lo, mid - 1),
        right: build(a, mid + 1, hi),
    }));
    Some(node)
}

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7];
    let root = build(&a, 0, a.len() as i32 - 1);
    let mut out: Vec<String> = Vec::new();
    let mut q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    if let Some(r) = root {
        q.push_back(r);
    }
    while let Some(n) = q.pop_front() {
        let nb = n.borrow();
        out.push(nb.val.to_string());
        if let Some(l) = nb.left.clone() {
            q.push_back(l);
        }
        if let Some(r) = nb.right.clone() {
            q.push_back(r);
        }
    }
    println!("{}", out.join(" "));
}
