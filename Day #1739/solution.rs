// Tree node with parent pointer + locked_descendants count; lock/unlock walk ancestors O(h).
// is_locked O(1). lock/unlock O(h). Uses Rc<RefCell> + Weak parent. Space O(n).
use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    parent: RefCell<Weak<Node>>,
    left: RefCell<Option<Rc<Node>>>,
    right: RefCell<Option<Rc<Node>>>,
    locked: RefCell<bool>,
    locked_descendants: RefCell<i32>,
}

impl Node {
    fn new() -> Rc<Node> {
        Rc::new(Node {
            parent: RefCell::new(Weak::new()),
            left: RefCell::new(None),
            right: RefCell::new(None),
            locked: RefCell::new(false),
            locked_descendants: RefCell::new(0),
        })
    }

    fn is_locked(&self) -> bool {
        *self.locked.borrow()
    }

    fn any_ancestor_locked(&self) -> bool {
        let mut cur = self.parent.borrow().upgrade();
        while let Some(p) = cur {
            if *p.locked.borrow() {
                return true;
            }
            cur = p.parent.borrow().upgrade();
        }
        false
    }

    fn lock(&self) -> bool {
        if *self.locked.borrow() || *self.locked_descendants.borrow() > 0 || self.any_ancestor_locked() {
            return false;
        }
        *self.locked.borrow_mut() = true;
        let mut cur = self.parent.borrow().upgrade();
        while let Some(p) = cur {
            *p.locked_descendants.borrow_mut() += 1;
            cur = p.parent.borrow().upgrade();
        }
        true
    }

    fn unlock(&self) -> bool {
        if !*self.locked.borrow() || *self.locked_descendants.borrow() > 0 || self.any_ancestor_locked() {
            return false;
        }
        *self.locked.borrow_mut() = false;
        let mut cur = self.parent.borrow().upgrade();
        while let Some(p) = cur {
            *p.locked_descendants.borrow_mut() -= 1;
            cur = p.parent.borrow().upgrade();
        }
        true
    }
}

fn child_of(parent: &Rc<Node>) -> Rc<Node> {
    let n = Node::new();
    *n.parent.borrow_mut() = Rc::downgrade(parent);
    n
}

fn main() {
    let root = Node::new();
    let left = child_of(&root);
    let right = child_of(&root);
    let ll = child_of(&left);
    let lr = child_of(&left);
    *root.left.borrow_mut() = Some(left.clone());
    *root.right.borrow_mut() = Some(right.clone());
    *left.left.borrow_mut() = Some(ll.clone());
    *left.right.borrow_mut() = Some(lr.clone());

    let _ = right;
    println!("{}", ll.lock());
    println!("{}", left.lock());
    println!("{}", root.lock());
    println!("{}", ll.unlock());
    println!("{}", left.lock());
    println!("{}", root.lock());
    let _ = root.is_locked();
}
