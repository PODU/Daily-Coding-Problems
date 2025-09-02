// Day 204: Count nodes of a complete binary tree faster than O(n).
// Compare left/right spine heights: if equal subtree is perfect (2^h - 1), else recurse.
// Time: O(log^2 n), Space: O(log n).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Default)]
struct Node {
    left: Link,
    right: Link,
}

fn node() -> Rc<RefCell<Node>> {
    Rc::new(RefCell::new(Node::default()))
}

fn left_height(n: &Link) -> i32 {
    let mut h = 0;
    let mut cur = n.clone();
    while let Some(x) = cur {
        h += 1;
        cur = x.borrow().left.clone();
    }
    h
}

fn right_height(n: &Link) -> i32 {
    let mut h = 0;
    let mut cur = n.clone();
    while let Some(x) = cur {
        h += 1;
        cur = x.borrow().right.clone();
    }
    h
}

fn count_nodes(root: &Link) -> i32 {
    match root {
        None => 0,
        Some(r) => {
            let lh = left_height(root);
            let rh = right_height(root);
            if lh == rh {
                (1 << lh) - 1 // perfect subtree
            } else {
                1 + count_nodes(&r.borrow().left) + count_nodes(&r.borrow().right)
            }
        }
    }
}

fn main() {
    let n: Vec<Link> = (0..7).map(|i| if i == 0 { None } else { Some(node()) }).collect();
    n[1].as_ref().unwrap().borrow_mut().left = n[2].clone();
    n[1].as_ref().unwrap().borrow_mut().right = n[3].clone();
    n[2].as_ref().unwrap().borrow_mut().left = n[4].clone();
    n[2].as_ref().unwrap().borrow_mut().right = n[5].clone();
    n[3].as_ref().unwrap().borrow_mut().left = n[6].clone();
    println!("{}", count_nodes(&n[1])); // 6
}
