// Two-pointer intersection of singly linked lists using shared Rc nodes; redirect to other head at end.
// Time O(M+N), Space O(1) (aside from the shared list storage).
use std::rc::Rc;
use std::cell::RefCell;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    next: Link,
}

fn get_intersection(head_a: &Link, head_b: &Link) -> Link {
    if head_a.is_none() || head_b.is_none() {
        return None;
    }
    let mut p_a = head_a.clone();
    let mut p_b = head_b.clone();
    loop {
        let same = match (&p_a, &p_b) {
            (Some(x), Some(y)) => Rc::ptr_eq(x, y),
            (None, None) => true,
            _ => false,
        };
        if same {
            return p_a;
        }
        p_a = match p_a {
            Some(n) => n.borrow().next.clone(),
            None => head_b.clone(),
        };
        p_b = match p_b {
            Some(n) => n.borrow().next.clone(),
            None => head_a.clone(),
        };
    }
}

fn node(val: i32) -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node { val, next: None }))
}

fn main() {
    let n8 = node(8);
    n8.borrow_mut().next = Some(node(10));

    let a = node(3);
    let a7 = node(7);
    a7.borrow_mut().next = Some(n8.clone());
    a.borrow_mut().next = Some(a7);

    let b = node(99);
    let b1 = node(1);
    b1.borrow_mut().next = Some(n8.clone());
    b.borrow_mut().next = Some(b1);

    let head_a: Link = Some(a);
    let head_b: Link = Some(b);

    let res = get_intersection(&head_a, &head_b);
    match res {
        Some(n) => println!("{}", n.borrow().val),
        None => println!("null"),
    }
}
