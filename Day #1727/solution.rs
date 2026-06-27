// Level of a binary tree with the minimum node-value sum.
// BFS level-order, track the level whose sum is smallest. O(n) time, O(w) space (max width).
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i64,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i64, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }
}

fn min_sum_level(root: &Link) -> i32 {
    if root.is_none() {
        return -1;
    }
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());
    let mut level = 0i32;
    let mut best_level = 0i32;
    let mut best_sum = i64::MAX;
    while !queue.is_empty() {
        let mut sum: i64 = 0;
        for _ in 0..queue.len() {
            let n = queue.pop_front().unwrap();
            let nb = n.borrow();
            sum += nb.val;
            if let Some(l) = &nb.left {
                queue.push_back(l.clone());
            }
            if let Some(r) = &nb.right {
                queue.push_back(r.clone());
            }
        }
        if sum < best_sum {
            best_sum = sum;
            best_level = level;
        }
        level += 1;
    }
    best_level
}

fn main() {
    let root = Node::new(
        5,
        Node::new(2, Node::new(-5, None, None), None),
        Node::new(3, None, None),
    );
    println!("Level with minimum sum: {}", min_sum_level(&root));
}
