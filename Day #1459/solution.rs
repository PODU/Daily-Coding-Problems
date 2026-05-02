// Invert binary tree by swapping children recursively; print level-order (BFS).
// Time O(n), Space O(n).
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

    let mut out: Vec<char> = Vec::new();
    let mut q: VecDeque<Link> = VecDeque::new();
    q.push_back(root);
    while let Some(cur) = q.pop_front() {
        if let Some(n) = cur {
            let nb = n.borrow();
            out.push(nb.val);
            q.push_back(nb.left.clone());
            q.push_back(nb.right.clone());
        }
    }
    let strs: Vec<String> = out.iter().map(|c| c.to_string()).collect();
    println!("{}", strs.join(" "));
}
