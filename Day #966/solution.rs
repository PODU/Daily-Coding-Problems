// Day 966: Deep clone a linked list where each node has a random pointer.
// Approach: index-based copy (Rc/RefCell graph). Time O(n), Space O(n).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    next: Link,
    random: Link,
}

fn clone_list(head: &Link) -> Link {
    // collect originals in order
    let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut cur = head.clone();
    while let Some(n) = cur {
        nodes.push(n.clone());
        cur = n.borrow().next.clone();
    }
    if nodes.is_empty() {
        return None;
    }
    // create copies
    let copies: Vec<Rc<RefCell<Node>>> = nodes
        .iter()
        .map(|n| Rc::new(RefCell::new(Node { val: n.borrow().val, next: None, random: None })))
        .collect();
    // map address -> index
    let index_of = |target: &Link| -> Option<usize> {
        target.as_ref().and_then(|t| {
            nodes.iter().position(|n| Rc::ptr_eq(n, t))
        })
    };
    for i in 0..nodes.len() {
        if i + 1 < copies.len() {
            copies[i].borrow_mut().next = Some(copies[i + 1].clone());
        }
        if let Some(j) = index_of(&nodes[i].borrow().random) {
            copies[i].borrow_mut().random = Some(copies[j].clone());
        }
    }
    Some(copies[0].clone())
}

fn main() {
    let a = Rc::new(RefCell::new(Node { val: 1, next: None, random: None }));
    let b = Rc::new(RefCell::new(Node { val: 2, next: None, random: None }));
    let c = Rc::new(RefCell::new(Node { val: 3, next: None, random: None }));
    a.borrow_mut().next = Some(b.clone());
    b.borrow_mut().next = Some(c.clone());
    a.borrow_mut().random = Some(c.clone());
    b.borrow_mut().random = Some(a.clone());
    c.borrow_mut().random = Some(b.clone());

    let mut cur = clone_list(&Some(a));
    while let Some(n) = cur {
        let rv = n.borrow().random.as_ref().map(|r| r.borrow().val).unwrap_or(-1);
        println!("val={} random={}", n.borrow().val, rv);
        cur = n.borrow().next.clone();
    }
}
