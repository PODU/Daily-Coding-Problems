// Height-balanced check via bottom-up recursion returning height, -1 sentinel = unbalanced.
// Time O(n), Space O(h).

type Link = Option<Box<Node>>;

struct Node {
    #[allow(dead_code)]
    val: i32,
    left: Link,
    right: Link,
}

impl Node {
    fn new(val: i32, left: Link, right: Link) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn height(node: &Link) -> i32 {
    match node {
        None => 0,
        Some(n) => {
            let lh = height(&n.left);
            if lh == -1 {
                return -1;
            }
            let rh = height(&n.right);
            if rh == -1 {
                return -1;
            }
            if (lh - rh).abs() > 1 {
                return -1;
            }
            lh.max(rh) + 1
        }
    }
}

fn is_balanced(root: &Link) -> bool {
    height(root) != -1
}

fn main() {
    // Balanced tree
    let a: Link = Some(Node::new(
        1,
        Some(Node::new(2, Some(Node::new(4, None, None)), None)),
        Some(Node::new(3, None, None)),
    ));

    // Unbalanced left-leaning chain 1 -> 2 -> 3 -> 4
    let b: Link = Some(Node::new(
        1,
        Some(Node::new(
            2,
            Some(Node::new(3, Some(Node::new(4, None, None)), None)),
            None,
        )),
        None,
    ));

    println!("{}", if is_balanced(&a) { "true" } else { "false" });
    println!("{}", if is_balanced(&b) { "true" } else { "false" });
}
