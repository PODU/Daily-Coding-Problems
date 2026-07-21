// Day 1855: LCA in a binary tree with parent pointers.
// Two-pointer walk (like linked-list intersection): swap to the other node at root; meet at LCA. O(h) time, O(1) space.
// Uses Rc<RefCell<>> + Weak parent to model the parent-pointer tree safely.
use std::cell::RefCell;
use std::rc::{Rc, Weak};

type Link = Rc<RefCell<Node>>;

struct Node {
    val: i32,
    left: Option<Link>,
    right: Option<Link>,
    parent: Option<Weak<RefCell<Node>>>,
}

fn node(val: i32) -> Link {
    Rc::new(RefCell::new(Node { val, left: None, right: None, parent: None }))
}

fn set_left(p: &Link, c: &Link) {
    c.borrow_mut().parent = Some(Rc::downgrade(p));
    p.borrow_mut().left = Some(Rc::clone(c));
}
fn set_right(p: &Link, c: &Link) {
    c.borrow_mut().parent = Some(Rc::downgrade(p));
    p.borrow_mut().right = Some(Rc::clone(c));
}

fn depth(mut n: Link) -> usize {
    let mut d = 0;
    loop {
        let p = n.borrow().parent.as_ref().and_then(|w| w.upgrade());
        match p {
            Some(par) => { n = par; d += 1; }
            None => break,
        }
    }
    d
}

fn parent(n: &Link) -> Link {
    n.borrow().parent.as_ref().unwrap().upgrade().unwrap()
}

fn lca(p: &Link, q: &Link) -> Link {
    let (mut a, mut b) = (Rc::clone(p), Rc::clone(q));
    let (mut da, mut db) = (depth(Rc::clone(p)), depth(Rc::clone(q)));
    while da > db { a = parent(&a); da -= 1; }
    while db > da { b = parent(&b); db -= 1; }
    while !Rc::ptr_eq(&a, &b) {
        a = parent(&a);
        b = parent(&b);
    }
    a
}

fn main() {
    let n: Vec<Link> = (0..=8).map(|i| node(i as i32)).collect();
    set_left(&n[1], &n[2]); set_right(&n[1], &n[3]);
    set_left(&n[2], &n[4]); set_right(&n[2], &n[5]);
    set_right(&n[3], &n[6]);
    set_left(&n[5], &n[7]); set_right(&n[5], &n[8]);

    println!("{}", lca(&n[7], &n[8]).borrow().val); // 5
    println!("{}", lca(&n[7], &n[6]).borrow().val); // 1
    println!("{}", lca(&n[4], &n[8]).borrow().val); // 2
}
