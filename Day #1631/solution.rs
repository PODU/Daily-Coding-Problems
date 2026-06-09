// Validate BST via recursive inclusive min/max bounds (left<=root<=right). O(n) time, O(h) space.
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i64,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i64) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None }))
    }
}

fn is_valid(n: &Link, lo: i64, hi: i64) -> bool {
    match n {
        None => true,
        Some(node) => {
            let node = node.borrow();
            if node.val < lo || node.val > hi {
                return false;
            }
            is_valid(&node.left, lo, node.val) && is_valid(&node.right, node.val, hi)
        }
    }
}

fn validate(root: &Link) -> bool {
    is_valid(root, i64::MIN, i64::MAX)
}

fn main() {
    // Valid BST: root 5, left 3 (2,4), right 8 (6,9)
    let root = Node::new(5);
    let l = Node::new(3);
    l.borrow_mut().left = Some(Node::new(2));
    l.borrow_mut().right = Some(Node::new(4));
    let r = Node::new(8);
    r.borrow_mut().left = Some(Node::new(6));
    r.borrow_mut().right = Some(Node::new(9));
    root.borrow_mut().left = Some(l);
    root.borrow_mut().right = Some(r);

    // Invalid: root 5, left child 6
    let bad = Node::new(5);
    bad.borrow_mut().left = Some(Node::new(6));

    println!("{}", if validate(&Some(root)) { "true" } else { "false" });
    println!("{}", if validate(&Some(bad)) { "true" } else { "false" });
}
