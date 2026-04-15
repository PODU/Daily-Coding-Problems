// Count complete-tree nodes: if left height == right height subtree is perfect (2^h-1),
// else recurse. Time O(log^2 n), Space O(log n) recursion.
use std::cell::RefCell;
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

fn left_height(mut node: Link) -> i32 {
    let mut h = 0;
    while let Some(n) = node {
        h += 1;
        node = n.borrow().left.clone();
    }
    h
}

fn right_height(mut node: Link) -> i32 {
    let mut h = 0;
    while let Some(n) = node {
        h += 1;
        node = n.borrow().right.clone();
    }
    h
}

fn count_nodes(root: &Link) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let lh = left_height(Some(n.clone()));
            let rh = right_height(Some(n.clone()));
            if lh == rh {
                (1 << lh) - 1
            } else {
                1 + count_nodes(&n.borrow().left) + count_nodes(&n.borrow().right)
            }
        }
    }
}

fn main() {
    let root = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    let n4 = Node::new(4);
    let n5 = Node::new(5);
    let n6 = Node::new(6);
    n2.borrow_mut().left = Some(n4);
    n2.borrow_mut().right = Some(n5);
    n3.borrow_mut().left = Some(n6);
    root.borrow_mut().left = Some(n2);
    root.borrow_mut().right = Some(n3);
    let root_link: Link = Some(root);
    println!("{}", count_nodes(&root_link));
}
