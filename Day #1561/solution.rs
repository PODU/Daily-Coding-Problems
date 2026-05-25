// BFS level-order traversal of a binary tree using a queue. Time O(n), Space O(n).
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, left: None, right: None }))
    }
}

fn main() {
    let root = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    let n4 = Node::new(4);
    let n5 = Node::new(5);
    n3.borrow_mut().left = Some(n4.clone());
    n3.borrow_mut().right = Some(n5.clone());
    root.borrow_mut().left = Some(n2.clone());
    root.borrow_mut().right = Some(n3.clone());

    let mut out: Vec<String> = Vec::new();
    let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    q.push_back(root);
    while let Some(n) = q.pop_front() {
        let nb = n.borrow();
        out.push(nb.val.to_string());
        if let Some(l) = &nb.left {
            q.push_back(l.clone());
        }
        if let Some(r) = &nb.right {
            q.push_back(r.clone());
        }
    }
    println!("{}", out.join(", "));
}
