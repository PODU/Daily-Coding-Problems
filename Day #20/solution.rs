// Intersection of two linked lists: two-pointer switch trick over shared nodes.
// Time O(M+N), Space O(1). Uses Rc/RefCell to model a shared tail.
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    next: Link,
}

fn get_intersection(a: &Link, b: &Link) -> Link {
    let mut pa = a.clone();
    let mut pb = b.clone();
    loop {
        let same = match (&pa, &pb) {
            (Some(x), Some(y)) => Rc::ptr_eq(x, y),
            (None, None) => true,
            _ => false,
        };
        if same {
            return pa;
        }
        pa = match pa {
            Some(n) => n.borrow().next.clone(),
            None => b.clone(),
        };
        pb = match pb {
            Some(n) => n.borrow().next.clone(),
            None => a.clone(),
        };
    }
}

fn node(val: i32, next: Link) -> Link {
    Some(Rc::new(RefCell::new(Node { val, next })))
}

fn main() {
    let shared = node(8, node(10, None));

    let a = node(3, node(7, shared.clone()));
    let b = node(99, node(1, shared.clone()));

    let res = get_intersection(&a, &b);
    println!("{}", res.unwrap().borrow().val);
}
