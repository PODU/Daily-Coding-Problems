// Day 89: Validate BST via recursive [lo, hi] range check (left<=root<=right allowed).
// Time O(n), Space O(h).
struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn valid(node: &Option<Box<Node>>, lo: i64, hi: i64) -> bool {
    match node {
        None => true,
        Some(n) => {
            n.val >= lo && n.val <= hi
                && valid(&n.left, lo, n.val)
                && valid(&n.right, n.val, hi)
        }
    }
}

fn is_bst(root: &Option<Box<Node>>) -> bool {
    valid(root, i64::MIN, i64::MAX)
}

fn node(v: i64, l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: l, right: r }))
}

fn main() {
    let a = node(5, node(3, node(2, None, None), node(4, None, None)), node(8, None, None));
    println!("{}", is_bst(&a)); // true

    let b = node(5, node(3, None, node(6, None, None)), node(8, None, None)); // invalid
    println!("{}", is_bst(&b)); // false
}
