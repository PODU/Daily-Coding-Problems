// Cousins: BFS level by level; on the target's level collect nodes whose parent differs. O(n) time, O(n) space.
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
    let r = match root {
        Some(r) => r.clone(),
        None => return res,
    };
    // queue of (node, parent_val)
    let mut queue: VecDeque<(Rc<RefCell<Node>>, Option<i32>)> = VecDeque::new();
    queue.push_back((r, None));
    while !queue.is_empty() {
        let mut level: Vec<(Rc<RefCell<Node>>, Option<i32>)> = Vec::new();
        let n = queue.len();
        let mut target_parent: Option<i32> = None;
        let mut found = false;
        for _ in 0..n {
            level.push(queue.pop_front().unwrap());
        }
        for (node, par) in &level {
            let nb = node.borrow();
            if nb.val == target {
                target_parent = *par;
                found = true;
            }
            if let Some(l) = &nb.left {
                queue.push_back((l.clone(), Some(nb.val)));
            }
            if let Some(rg) = &nb.right {
                queue.push_back((rg.clone(), Some(nb.val)));
            }
        }
        if found {
            for (node, par) in &level {
                let v = node.borrow().val;
                if *par != target_parent && v != target {
                    res.push(v);
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
    n2.borrow_mut().left = Some(Node::new(4));
    n2.borrow_mut().right = Some(Node::new(5));
    n3.borrow_mut().right = Some(Node::new(6));
    root.borrow_mut().left = Some(n2);
    root.borrow_mut().right = Some(n3);
    let root_link: Link = Some(root);

    println!("Cousins of 4: {:?}", cousins(&root_link, 4)); // [6]
    println!("Cousins of 6: {:?}", cousins(&root_link, 6)); // [4, 5]
}
