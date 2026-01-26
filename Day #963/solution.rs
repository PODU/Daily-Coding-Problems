// Day 963: Find intersecting node of two singly linked lists.
// Approach: two-pointer traversal with shared Rc nodes. Time O(M+N), Space O(1).
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    next: Link,
}

fn get_intersection(a: &Link, b: &Link) -> Link {
    if a.is_none() || b.is_none() {
        return None;
    }
    let mut p = a.clone();
    let mut q = b.clone();
    loop {
        let same = match (&p, &q) {
            (Some(x), Some(y)) => Rc::ptr_eq(x, y),
            (None, None) => true,
            _ => false,
        };
        if same {
            return p;
        }
        p = match p {
            Some(n) => n.borrow().next.clone(),
            None => b.clone(),
        };
        q = match q {
            Some(n) => n.borrow().next.clone(),
            None => a.clone(),
        };
    }
}

fn main() {
    let n10 = Rc::new(RefCell::new(Node { val: 10, next: None }));
    let n8 = Rc::new(RefCell::new(Node { val: 8, next: Some(n10) }));
    let a = Some(Rc::new(RefCell::new(Node {
        val: 3,
        next: Some(Rc::new(RefCell::new(Node { val: 7, next: Some(n8.clone()) }))),
    })));
    let b = Some(Rc::new(RefCell::new(Node {
        val: 99,
        next: Some(Rc::new(RefCell::new(Node { val: 1, next: Some(n8.clone()) }))),
    })));

    let res = get_intersection(&a, &b).unwrap();
    let v = res.borrow().val;
    println!("the node with value {}", v);
}
