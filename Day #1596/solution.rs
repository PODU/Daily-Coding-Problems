// Approach: Symmetric k-ary tree via is_mirror recursion comparing children mirror-wise.
// Time O(n), Space O(h) recursion.

struct Node {
    val: i32,
    children: Vec<Node>,
}

fn is_mirror(a: &Node, b: &Node) -> bool {
    if a.val != b.val {
        return false;
    }
    if a.children.len() != b.children.len() {
        return false;
    }
    let k = a.children.len();
    for i in 0..k {
        if !is_mirror(&a.children[i], &b.children[k - 1 - i]) {
            return false;
        }
    }
    true
}

fn is_symmetric(root: &Node) -> bool {
    is_mirror(root, root)
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
    println!("{}", if is_symmetric(&root) { "true" } else { "false" });
}
