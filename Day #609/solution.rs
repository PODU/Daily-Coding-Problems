// Day 609: Inorder successor in a BST using parent pointers.
// Approach: if right child exists, take its leftmost; else climb until coming from a left child. Time O(h).
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
    parent: Option<Weak<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None, parent: None }))
    }
}

fn set_left(p: &Rc<RefCell<Node>>, c: &Rc<RefCell<Node>>) {
    p.borrow_mut().left = Some(c.clone());
    c.borrow_mut().parent = Some(Rc::downgrade(p));
}

fn set_right(p: &Rc<RefCell<Node>>, c: &Rc<RefCell<Node>>) {
    p.borrow_mut().right = Some(c.clone());
    c.borrow_mut().parent = Some(Rc::downgrade(p));
}

fn inorder_successor(node: &Rc<RefCell<Node>>) -> Link {
    if let Some(r) = node.borrow().right.clone() {
        let mut c = r;
        loop {
            let next = c.borrow().left.clone();
            match next {
                Some(l) => c = l,
                None => return Some(c),
            }
        }
    }
    let mut cur = node.clone();
    loop {
        let parent = cur.borrow().parent.as_ref().and_then(|w| w.upgrade());
        match parent {
            Some(p) => {
                let is_right = p
                    .borrow()
                    .right
                    .as_ref()
                    .map_or(false, |r| Rc::ptr_eq(r, &cur));
                if is_right {
                    cur = p;
                } else {
                    return Some(p);
                }
            }
            None => return None,
        }
    }
}

fn main() {
    let n10 = Node::new(10);
    let n5 = Node::new(5);
    let n30 = Node::new(30);
    let n22 = Node::new(22);
    let n35 = Node::new(35);
    set_left(&n10, &n5);
    set_right(&n10, &n30);
    set_left(&n30, &n22);
    set_right(&n30, &n35);

    match inorder_successor(&n22) {
        Some(s) => println!("{}", s.borrow().val), // 30
        None => println!("null"),
    }
}
