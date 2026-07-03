// Day 1755: Count nodes in a COMPLETE binary tree in better than O(n).
// Compare left/right spine heights: if equal, subtree is perfect (2^h - 1);
// else 1 + recurse on both children. Time O(log^2 n), Space O(log n).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None }))
    }
}

fn left_height(mut node: Link) -> i32 {
    let mut h = 0;
    while let Some(n) = node {
        h += 1;
        node = n.borrow().left.clone();
    }
    h
}

fn right_height(mut node: Link) -> i32 {
    let mut h = 0;
    while let Some(n) = node {
        h += 1;
        node = n.borrow().right.clone();
    }
    h
}

fn count_nodes(root: &Link) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let lh = left_height(Some(n.clone()));
            let rh = right_height(Some(n.clone()));
            if lh == rh {
                (1 << lh) - 1 // perfect subtree
            } else {
                1 + count_nodes(&n.borrow().left) + count_nodes(&n.borrow().right)
            }
        }
    }
}

fn main() {
    // Complete binary tree with 6 nodes (values 1..6):
    //          1
    //        /   \
    //       2     3
    //      / \   /
    //     4   5 6
    let root = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    n2.borrow_mut().left = Some(Node::new(4));
    n2.borrow_mut().right = Some(Node::new(5));
    n3.borrow_mut().left = Some(Node::new(6));
    root.borrow_mut().left = Some(n2);
    root.borrow_mut().right = Some(n3);

    let link: Link = Some(root);
    println!("{}", count_nodes(&link)); // expected: 6
}
