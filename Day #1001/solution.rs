// Day 1001: Validate a binary search tree.
// Recurse carrying an allowed [low, high] range; left key <= root <= right key.
// O(n) time, O(h) space.
type Link = Option<Box<Node>>;

struct Node {
    val: i64,
    left: Link,
    right: Link,
}

fn node(val: i64, left: Link, right: Link) -> Link {
    Some(Box::new(Node { val, left, right }))
}

fn is_bst(n: &Link, low: i64, high: i64) -> bool {
    match n {
        None => true,
        Some(node) => {
            node.val >= low
                && node.val <= high
                && is_bst(&node.left, low, node.val)
                && is_bst(&node.right, node.val, high)
        }
    }
}

fn main() {
    let valid = node(5,
        node(3, node(2, None, None), node(4, None, None)),
        node(8, node(6, None, None), node(9, None, None)));
    let invalid = node(5, node(6, None, None), node(8, None, None));
    println!("{}", is_bst(&valid, i64::MIN, i64::MAX));   // true
    println!("{}", is_bst(&invalid, i64::MIN, i64::MAX)); // false
}
