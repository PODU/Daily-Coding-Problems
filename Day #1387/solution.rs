// BST two-sum: in-order into sorted array, then two-pointer scan for pair summing to K. O(n) time, O(n) space.
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

fn inorder(root: &Link, a: &mut Vec<i32>) {
    if let Some(n) = root {
        inorder(&n.borrow().left, a);
        a.push(n.borrow().val);
        inorder(&n.borrow().right, a);
    }
}

fn two_sum(root: &Link, k: i32) -> (i32, i32) {
    let mut a = Vec::new();
    inorder(root, &mut a);
    let (mut i, mut j) = (0usize, a.len().wrapping_sub(1));
    while i < j {
        let s = a[i] + a[j];
        if s == k {
            return (a[i], a[j]);
        }
        if s < k {
            i += 1;
        } else {
            j -= 1;
        }
    }
    (-1, -1)
}

fn main() {
    let root = Node::new(10);
    let r = Node::new(15);
    r.borrow_mut().left = Some(Node::new(11));
    r.borrow_mut().right = Some(Node::new(15));
    root.borrow_mut().left = Some(Node::new(5));
    root.borrow_mut().right = Some(r);
    let root_link: Link = Some(root);
    let (x, y) = two_sum(&root_link, 20);
    println!("{} {}", x, y);
}
