// Zigzag (boustrophedon) level-order traversal: alternate direction each level.
// BFS level by level, reverse odd levels. O(n) time, O(n) space.
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }
}

fn zigzag(root: &Link) -> Vec<i32> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    let mut queue: Vec<Rc<RefCell<Node>>> = vec![root.clone().unwrap()];
    let mut left_to_right = true;
    while !queue.is_empty() {
        let mut next: Vec<Rc<RefCell<Node>>> = Vec::new();
        let mut level: Vec<i32> = Vec::new();
        for n in &queue {
            let nb = n.borrow();
            level.push(nb.val);
            if let Some(l) = &nb.left {
                next.push(l.clone());
            }
            if let Some(r) = &nb.right {
                next.push(r.clone());
            }
        }
        if !left_to_right {
            level.reverse();
        }
        res.extend(level);
        left_to_right = !left_to_right;
        queue = next;
    }
    res
}

fn main() {
    let root = Node::new(
        1,
        Node::new(2, Node::new(4, None, None), Node::new(5, None, None)),
        Node::new(3, Node::new(6, None, None), Node::new(7, None, None)),
    );
    println!("{:?}", zigzag(&root)); // [1, 3, 2, 4, 5, 6, 7]
}
