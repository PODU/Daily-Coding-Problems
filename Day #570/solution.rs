// Day 570: Check whether tree t is a subtree of tree s.
// At each node of s test same_tree(node, t). Time O(|s|*|t|), Space O(height).
use std::rc::Rc;

struct Node {
    val: i32,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

fn leaf(v: i32) -> Option<Rc<Node>> {
    Some(Rc::new(Node { val: v, left: None, right: None }))
}

fn node(v: i32, l: Option<Rc<Node>>, r: Option<Rc<Node>>) -> Option<Rc<Node>> {
    Some(Rc::new(Node { val: v, left: l, right: r }))
}

fn same_tree(a: &Option<Rc<Node>>, b: &Option<Rc<Node>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => {
            x.val == y.val && same_tree(&x.left, &y.left) && same_tree(&x.right, &y.right)
        }
        _ => false,
    }
}

fn is_subtree(s: &Option<Rc<Node>>, t: &Option<Rc<Node>>) -> bool {
    match s {
        None => false,
        Some(n) => same_tree(s, t) || is_subtree(&n.left, t) || is_subtree(&n.right, t),
    }
}

fn main() {
    let s = node(3, node(4, leaf(1), leaf(2)), leaf(5));
    let t = node(4, leaf(1), leaf(2));
    println!("{}", is_subtree(&s, &t)); // true
    let t2 = node(4, leaf(1), node(2, leaf(0), None));
    println!("{}", is_subtree(&s, &t2)); // false
}
