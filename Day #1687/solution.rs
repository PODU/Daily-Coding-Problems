// BFS level order; last node dequeued is a deepest node. Time O(n), Space O(n).
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

fn deepest_node(root: &Link) -> Option<char> {
    let root = root.as_ref()?;
    let mut q: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    q.push_back(Rc::clone(root));
    let mut last = Rc::clone(root);
    while let Some(node) = q.pop_front() {
        last = Rc::clone(&node);
        if let Some(l) = &node.borrow().left {
            q.push_back(Rc::clone(l));
        }
        if let Some(r) = &node.borrow().right {
            q.push_back(Rc::clone(r));
        }
    }
    Some(last.borrow().val)
}

fn main() {
    let a = Node::new('a');
    let b = Node::new('b');
    let c = Node::new('c');
    let d = Node::new('d');
    a.borrow_mut().left = Some(Rc::clone(&b));
    a.borrow_mut().right = Some(Rc::clone(&c));
    b.borrow_mut().left = Some(Rc::clone(&d));
    let root: Link = Some(a);
    println!("{}", deepest_node(&root).unwrap());
}
