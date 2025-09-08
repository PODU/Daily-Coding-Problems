// Symmetric k-ary tree: a tree is symmetric iff left subtree mirrors right subtree.
// Recursively compare children in mirrored order. Time: O(N), Space: O(height).
struct Node {
    val: i32,
    children: Vec<Node>,
}

fn mirror(a: &Node, b: &Node) -> bool {
    if a.val != b.val || a.children.len() != b.children.len() {
        return false;
    }
    let n = a.children.len();
    for i in 0..n {
        if !mirror(&a.children[i], &b.children[n - 1 - i]) {
            return false;
        }
    }
    true
}

fn is_symmetric(root: &Node) -> bool {
    let n = root.children.len();
    for i in 0..n / 2 {
        if !mirror(&root.children[i], &root.children[n - 1 - i]) {
            return false;
        }
    }
    true
}

fn main() {
    let root = Node {
        val: 4,
        children: vec![
            Node { val: 3, children: vec![Node { val: 9, children: vec![] }] },
            Node { val: 5, children: vec![] },
            Node { val: 3, children: vec![Node { val: 9, children: vec![] }] },
        ],
    };
    println!("{}", is_symmetric(&root)); // true
}
