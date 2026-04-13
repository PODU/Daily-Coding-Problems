// Zigzag (boustrophedon) level order of a binary tree. BFS per level, reverse alternate levels. O(N) time, O(N) space.
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

fn zigzag(root: &Link) -> Vec<i32> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::new();
    queue.push_back(root.as_ref().unwrap().clone());
    let mut left_to_right = true;
    while !queue.is_empty() {
        let sz = queue.len();
        let mut level = vec![0; sz];
        for i in 0..sz {
            let cur = queue.pop_front().unwrap();
            let idx = if left_to_right { i } else { sz - 1 - i };
            level[idx] = cur.borrow().val;
            if let Some(l) = cur.borrow().left.as_ref() {
                queue.push_back(l.clone());
            }
            if let Some(r) = cur.borrow().right.as_ref() {
                queue.push_back(r.clone());
            }
        }
        res.extend(level);
        left_to_right = !left_to_right;
    }
    res
}

fn main() {
    let root = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    n2.borrow_mut().left = Some(Node::new(4));
    n2.borrow_mut().right = Some(Node::new(5));
    n3.borrow_mut().left = Some(Node::new(6));
    n3.borrow_mut().right = Some(Node::new(7));
    root.borrow_mut().left = Some(n2);
    root.borrow_mut().right = Some(n3);

    let res = zigzag(&Some(root));
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
