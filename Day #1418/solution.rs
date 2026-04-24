// Day 1418: inorder successor of a BST node using parent pointers.
// Approach: if right subtree exists, leftmost of it; else climb until node is a left child. O(h).
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
    parent: Option<Weak<RefCell<Node>>>,
}

fn new_node(val: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node { val, left: None, right: None, parent: None }))
}

fn set_left(p: &Rc<RefCell<Node>>, c: &Rc<RefCell<Node>>) {
    c.borrow_mut().parent = Some(Rc::downgrade(p));
    p.borrow_mut().left = Some(Rc::clone(c));
}
fn set_right(p: &Rc<RefCell<Node>>, c: &Rc<RefCell<Node>>) {
    c.borrow_mut().parent = Some(Rc::downgrade(p));
    p.borrow_mut().right = Some(Rc::clone(c));
}

fn successor(node: &Rc<RefCell<Node>>) -> Link {
    if let Some(r) = node.borrow().right.clone() {
        let mut cur = r;
        loop {
            let next = cur.borrow().left.clone();
            match next {
                Some(l) => cur = l,
                None => break,
            }
        }
        return Some(cur);
    }
    let mut cur = Rc::clone(node);
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
    let root = new_node(10);
    let n5 = new_node(5);
    let n30 = new_node(30);
    let n22 = new_node(22);
    let n35 = new_node(35);
    set_left(&root, &n5);
    set_right(&root, &n30);
    set_left(&n30, &n22);
    set_right(&n30, &n35);

    match successor(&n22) {
        Some(s) => println!("{}", s.borrow().val), // 30
        None => println!("null"),
    }
}
