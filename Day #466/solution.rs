// Symmetric k-ary tree: recursively compare children of two subtrees in mirror order.
// Time: O(n), Space: O(h) recursion.
struct Node {
    val: i32,
    children: Vec<Node>,
}

fn mirror(a: &Node, b: &Node) -> bool {
    if a.val != b.val {
        return false;
    }
    if a.children.len() != b.children.len() {
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
    mirror(root, root)
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
    println!("{}", if is_symmetric(&root) { "True" } else { "False" });
}
