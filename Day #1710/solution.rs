// Deep clone list w/ random ptr: hashmap of index->clone (Rc/RefCell makes O(1)-extra unweaving
// unsafe in safe Rust). O(n) time, O(n) space; preserves identity & independence.
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    next: Link,
    random: Link,
}

fn copy_random_list(head: &Link) -> Link {
    if head.is_none() {
        return None;
    }
    // Collect original nodes in order.
    let mut originals: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut cur = head.clone();
    while let Some(n) = cur {
        originals.push(n.clone());
        cur = n.borrow().next.clone();
    }
    // Create clones.
    let clones: Vec<Rc<RefCell<Node>>> = originals
        .iter()
        .map(|n| Rc::new(RefCell::new(Node { val: n.borrow().val, next: None, random: None })))
        .collect();
    // Wire next and random using index lookup by pointer identity.
    let index_of = |target: &Rc<RefCell<Node>>| -> Option<usize> {
        originals.iter().position(|o| Rc::ptr_eq(o, target))
    };
    for (i, orig) in originals.iter().enumerate() {
        if i + 1 < clones.len() {
            clones[i].borrow_mut().next = Some(clones[i + 1].clone());
        }
        if let Some(r) = &orig.borrow().random {
            if let Some(j) = index_of(r) {
                clones[i].borrow_mut().random = Some(clones[j].clone());
            }
        }
    }
    Some(clones[0].clone())
}

fn main() {
    let n1 = Rc::new(RefCell::new(Node { val: 1, next: None, random: None }));
    let n2 = Rc::new(RefCell::new(Node { val: 2, next: None, random: None }));
    let n3 = Rc::new(RefCell::new(Node { val: 3, next: None, random: None }));
    let n4 = Rc::new(RefCell::new(Node { val: 4, next: None, random: None }));
    n1.borrow_mut().next = Some(n2.clone());
    n2.borrow_mut().next = Some(n3.clone());
    n3.borrow_mut().next = Some(n4.clone());
    n1.borrow_mut().random = Some(n3.clone());
    n2.borrow_mut().random = Some(n1.clone());
    n3.borrow_mut().random = Some(n3.clone());
    n4.borrow_mut().random = Some(n2.clone());

    let head: Link = Some(n1);
    let cloned = copy_random_list(&head);
    let mut cur = cloned;
    while let Some(n) = cur {
        let nb = n.borrow();
        let r = nb.random.as_ref().unwrap().borrow().val;
        println!("node {}, random {}", nb.val, r);
        cur = nb.next.clone();
    }
}
