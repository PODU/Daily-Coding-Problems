// Binary tree max path sum: DFS returning max downward gain; track global max = node + max(0,left) + max(0,right).
// Time O(n), Space O(h).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;
struct Node { val: i32, left: Link, right: Link }
impl Node {
    fn new(val: i32, left: Link, right: Link) -> Link {
        Some(Rc::new(RefCell::new(Node { val, left, right })))
    }
}

fn gain(n: &Link, best: &mut i32) -> i32 {
    if let Some(node) = n {
        let node = node.borrow();
        let l = gain(&node.left, best).max(0);
        let r = gain(&node.right, best).max(0);
        *best = (*best).max(node.val + l + r);
        node.val + l.max(r)
    } else {
        0
    }
}

fn max_path_sum(root: &Link) -> i32 {
    let mut best = i32::MIN;
    gain(root, &mut best);
    best
}

fn main() {
    let root = Node::new(-10,
        Node::new(9, None, None),
        Node::new(20, Node::new(15, None, None), Node::new(7, None, None)));
    println!("{}", max_path_sum(&root)); // expected 42
}
