// Day 935: Check if a binary tree is height-balanced.
// Bottom-up DFS returning height, -1 signals imbalance. Time O(n), Space O(h).
use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

struct Node {
    #[allow(dead_code)]
    val: i32,
    left: Link,
    right: Link,
}

fn node(val: i32, left: Link, right: Link) -> Link {
    Some(Rc::new(RefCell::new(Node { val, left, right })))
}

// Returns height if balanced, else -1.
fn check(n: &Link) -> i32 {
    match n {
        None => 0,
        Some(rc) => {
            let b = rc.borrow();
            let l = check(&b.left);
            if l == -1 {
                return -1;
            }
            let r = check(&b.right);
            if r == -1 {
                return -1;
            }
            if (l - r).abs() > 1 {
                return -1;
            }
            1 + l.max(r)
        }
    }
}

fn is_balanced(root: &Link) -> bool {
    check(root) != -1
}

fn main() {
    let a = node(1, node(2, node(4, None, None), None), node(3, None, None));
    println!("{}", is_balanced(&a)); // true

    let b = node(1, node(2, node(3, node(4, None, None), None), None), None);
    println!("{}", is_balanced(&b)); // false
}
