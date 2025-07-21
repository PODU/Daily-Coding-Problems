// Count unival subtrees: postorder; a subtree is unival if both children are
// unival and match the node's value. Time: O(n), Space: O(h).
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

fn is_unival(n: &Option<Box<Node>>, count: &mut i32) -> bool {
    match n {
        None => true,
        Some(node) => {
            let l = is_unival(&node.left, count);
            let r = is_unival(&node.right, count);
            if !l || !r {
                return false;
            }
            if let Some(c) = &node.left {
                if c.val != node.val {
                    return false;
                }
            }
            if let Some(c) = &node.right {
                if c.val != node.val {
                    return false;
                }
            }
            *count += 1;
            true
        }
    }
}

fn main() {
    let root = Node::new(
        0,
        Some(Node::new(1, None, None)),
        Some(Node::new(
            0,
            Some(Node::new(1, Some(Node::new(1, None, None)), Some(Node::new(1, None, None)))),
            Some(Node::new(0, None, None)),
        )),
    );
    let mut count = 0;
    is_unival(&Some(root), &mut count);
    println!("{}", count);
}
