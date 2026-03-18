// Cousins in a binary tree: BFS level by level tracking parent; collect same-level
// nodes with a different parent than target. Time O(n), Space O(n).
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

fn cousins(root: &Link, target: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let root = match root {
        Some(r) => r.clone(),
        None => return res,
    };
    // queue holds (node, parent value as Option<i32> identity proxy via Rc ptr)
    let mut queue: VecDeque<(Rc<RefCell<Node>>, Option<*const RefCell<Node>>)> = VecDeque::new();
    queue.push_back((root, None));
    while !queue.is_empty() {
        let sz = queue.len();
        let mut level: Vec<(Rc<RefCell<Node>>, Option<*const RefCell<Node>>)> = Vec::new();
        let mut target_parent: Option<*const RefCell<Node>> = None;
        let mut found = false;
        for _ in 0..sz {
            let (node, parent) = queue.pop_front().unwrap();
            let pptr = Rc::as_ptr(&node);
            if node.borrow().val == target {
                found = true;
                target_parent = parent;
            }
            if let Some(l) = node.borrow().left.clone() {
                queue.push_back((l, Some(pptr)));
            }
            if let Some(r) = node.borrow().right.clone() {
                queue.push_back((r, Some(pptr)));
            }
            level.push((node, parent));
        }
        if found {
            for (n, p) in &level {
                if n.borrow().val != target && *p != target_parent {
                    res.push(n.borrow().val);
                }
            }
            return res;
        }
    }
    res
}

fn main() {
    let root = Node::new(1);
    let n2 = Node::new(2);
    let n3 = Node::new(3);
    let n4 = Node::new(4);
    let n5 = Node::new(5);
    let n6 = Node::new(6);
    n2.borrow_mut().left = Some(n4.clone());
    n2.borrow_mut().right = Some(n5.clone());
    n3.borrow_mut().right = Some(n6.clone());
    root.borrow_mut().left = Some(n2.clone());
    root.borrow_mut().right = Some(n3.clone());

    let res = cousins(&Some(root), 4);
    let parts: Vec<String> = res.iter().map(|v| v.to_string()).collect();
    println!("[{}]", parts.join(", "));
}
