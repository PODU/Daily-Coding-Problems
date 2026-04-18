// Second largest in BST via reverse in-order (right,node,left), stop at 2nd visited node. O(h) space, O(n) worst time.
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

fn second_largest(root: &Link) -> i32 {
    let mut st: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut cur = root.clone();
    let mut count = 0;
    loop {
        while let Some(n) = cur {
            st.push(n.clone());
            cur = n.borrow().right.clone();
        }
        if let Some(n) = st.pop() {
            count += 1;
            if count == 2 {
                return n.borrow().val;
            }
            cur = n.borrow().left.clone();
        } else {
            break;
        }
    }
    -1
}

fn main() {
    let root = Node::new(5);
    let l = Node::new(3);
    l.borrow_mut().left = Some(Node::new(2));
    l.borrow_mut().right = Some(Node::new(4));
    let r = Node::new(8);
    r.borrow_mut().left = Some(Node::new(7));
    r.borrow_mut().right = Some(Node::new(9));
    root.borrow_mut().left = Some(l);
    root.borrow_mut().right = Some(r);
    let root_link: Link = Some(root);
    println!("{}", second_largest(&root_link));
}
