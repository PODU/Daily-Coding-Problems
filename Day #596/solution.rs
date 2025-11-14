// Day 596: Invert a binary tree (mirror it).
// Approach: recursively swap left/right children. Time O(n), Space O(h).
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: char,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: char) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None }))
    }
}

fn invert(node: &Link) {
    if let Some(n) = node {
        let mut nb = n.borrow_mut();
        std::mem::swap(&mut nb.left, &mut nb.right);
        let (l, r) = (nb.left.clone(), nb.right.clone());
        drop(nb);
        invert(&l);
        invert(&r);
    }
}

fn main() {
    let a = Node::new('a');
    let b = Node::new('b');
    let c = Node::new('c');
    let d = Node::new('d');
    let e = Node::new('e');
    let f = Node::new('f');
    a.borrow_mut().left = Some(b.clone());
    a.borrow_mut().right = Some(c.clone());
    b.borrow_mut().left = Some(d.clone());
    b.borrow_mut().right = Some(e.clone());
    c.borrow_mut().left = Some(f.clone());

    let root: Link = Some(a);
    invert(&root);

    let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    q.push_back(root.unwrap());
    while !q.is_empty() {
        let sz = q.len();
        let mut line: Vec<String> = Vec::new();
        for _ in 0..sz {
            let n = q.pop_front().unwrap();
            let nb = n.borrow();
            line.push(nb.val.to_string());
            if let Some(l) = &nb.left {
                q.push_back(l.clone());
            }
            if let Some(r) = &nb.right {
                q.push_back(r.clone());
            }
        }
        println!("{}", line.join(" "));
    }
}
