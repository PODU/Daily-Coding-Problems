// Day 115: Subtree check via recursive structural match. O(|s|*|t|) worst case.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn same(a: &Option<Box<Node>>, b: &Option<Box<Node>>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(x), Some(y)) => x.val == y.val && same(&x.left, &y.left) && same(&x.right, &y.right),
        _ => false,
    }
}

fn is_subtree(s: &Option<Box<Node>>, t: &Option<Box<Node>>) -> bool {
    match s {
        None => t.is_none(),
        Some(node) => {
            same(s, t) || is_subtree(&node.left, t) || is_subtree(&node.right, t)
        }
    }
}

fn main() {
    let s = Some(Node::new(
        3,
        Some(Node::new(4, Some(Node::new(1, None, None)), Some(Node::new(2, None, None)))),
        Some(Node::new(5, None, None)),
    ));
    let t = Some(Node::new(4, Some(Node::new(1, None, None)), Some(Node::new(2, None, None))));
    let u = Some(Node::new(4, Some(Node::new(0, None, None)), None));
    println!("{}", is_subtree(&s, &t)); // true
    println!("{}", is_subtree(&s, &u)); // false
}
