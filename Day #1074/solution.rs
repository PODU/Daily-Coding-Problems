// LCA with parent pointers: get depths via parent walk, level-up deeper node, advance both until equal. O(h) time O(1) space.
// Uses Rc<RefCell<Node>> for shared ownership needed by parent pointers in safe Rust.
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    val: i32,
    left: Link,
    right: Link,
    parent: Option<std::rc::Weak<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None, parent: None }))
    }
}

fn link(par: &Rc<RefCell<Node>>, child: &Rc<RefCell<Node>>, is_left: bool) {
    child.borrow_mut().parent = Some(Rc::downgrade(par));
    if is_left { par.borrow_mut().left  = Some(Rc::clone(child)); }
    else        { par.borrow_mut().right = Some(Rc::clone(child)); }
}

fn depth(n: &Rc<RefCell<Node>>) -> usize {
    let mut d = 0;
    let mut cur = Rc::clone(n);
    loop {
        let parent_weak = cur.borrow().parent.clone();
        match parent_weak {
            None => break,
            Some(w) => { cur = w.upgrade().unwrap(); d += 1; }
        }
    }
    d
}

fn lca(a: &Rc<RefCell<Node>>, b: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    let mut da = depth(a);
    let mut db = depth(b);
    let mut ca = Rc::clone(a);
    let mut cb = Rc::clone(b);
    while da > db {
        let p = ca.borrow().parent.clone().unwrap().upgrade().unwrap();
        ca = p; da -= 1;
    }
    while db > da {
        let p = cb.borrow().parent.clone().unwrap().upgrade().unwrap();
        cb = p; db -= 1;
    }
    while !Rc::ptr_eq(&ca, &cb) {
        let pa = ca.borrow().parent.clone().unwrap().upgrade().unwrap();
        let pb = cb.borrow().parent.clone().unwrap().upgrade().unwrap();
        ca = pa; cb = pb;
    }
    ca
}

fn main() {
    let n3 = Node::new(3);
    let n5 = Node::new(5); link(&n3, &n5, true);
    let n1 = Node::new(1); link(&n3, &n1, false);
    let n6 = Node::new(6); link(&n5, &n6, true);
    let n2 = Node::new(2); link(&n5, &n2, false);
    let n0 = Node::new(0); link(&n1, &n0, true);
    let n8 = Node::new(8); link(&n1, &n8, false);
    let n7 = Node::new(7); link(&n2, &n7, true);
    let n4 = Node::new(4); link(&n2, &n4, false);

    println!("LCA(7,4) = {}", lca(&n7, &n4).borrow().val);
    println!("LCA(6,4) = {}", lca(&n6, &n4).borrow().val);
    println!("LCA(7,1) = {}", lca(&n7, &n1).borrow().val);
}
