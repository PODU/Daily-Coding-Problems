// Day 1411: Check if tree t is a subtree of tree s.
// Approach: recursive DFS — for each node of s try exact-match with t. O(|s|*|t|) time, O(h) space.
use std::rc::Rc;

struct Node {
    val: i32,
    left: Option<Rc<Node>>,
    right: Option<Rc<Node>>,
}

fn node(val: i32, left: Option<Rc<Node>>, right: Option<Rc<Node>>) -> Rc<Node> {
    Rc::new(Node { val, left, right })
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
    let s = Some(node(
        3,
        Some(node(4, Some(node(1, None, None)), Some(node(2, None, None)))),
        Some(node(5, None, None)),
    ));
    let t = Some(node(4, Some(node(1, None, None)), Some(node(2, None, None))));
    println!("{}", if is_subtree(&s, &t) { "true" } else { "false" });
}
