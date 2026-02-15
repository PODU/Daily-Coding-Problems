// K-ary tree symmetry: recursively mirror-match children list. O(n) time/space.
#[derive(Debug)]
struct Node { val: i32, children: Vec<Node> }

impl Node {
    fn new(val: i32, children: Vec<Node>) -> Self { Node { val, children } }
}

fn is_mirror(l: &Node, r: &Node) -> bool {
    if l.val != r.val { return false; }
    let n = l.children.len();
    if r.children.len() != n { return false; }
    (0..n).all(|i| is_mirror(&l.children[i], &r.children[n - 1 - i]))
}

fn is_symmetric(root: &Node) -> bool {
    let n = root.children.len();
    (0..n / 2).all(|i| is_mirror(&root.children[i], &root.children[n - 1 - i]))
}

fn main() {
    // Symmetric: 4 -> [3,5,3], first 3 -> [9], last 3 -> [9]
    let root = Node::new(4, vec![
        Node::new(3, vec![Node::new(9, vec![])]),
        Node::new(5, vec![]),
        Node::new(3, vec![Node::new(9, vec![])]),
    ]);
    println!("Symmetric: {}", is_symmetric(&root));

    // Asymmetric: 1 -> [2,3]
    let r2 = Node::new(1, vec![Node::new(2, vec![]), Node::new(3, vec![])]);
    println!("Symmetric: {}", is_symmetric(&r2));
}
