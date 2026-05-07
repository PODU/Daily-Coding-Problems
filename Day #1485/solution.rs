// Day 1485: Determine whether a k-ary tree is symmetric about its root.
// Subtrees mirror iff values match and child i mirrors child (k-1-i).
// Time O(N), Space O(H).

struct Node {
    val: i32,
    children: Vec<Node>,
}

fn is_mirror(a: &Node, b: &Node) -> bool {
    if a.val != b.val || a.children.len() != b.children.len() {
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
    let tree = Node {
        val: 4,
        children: vec![
            Node { val: 3, children: vec![Node { val: 9, children: vec![] }] },
            Node { val: 5, children: vec![] },
            Node { val: 3, children: vec![Node { val: 9, children: vec![] }] },
        ],
    };
    println!("{}", is_symmetric(&tree)); // true
}
