// Day 94: Max path sum in a binary tree. DFS returns best downward gain; at each
// node consider a path bending through it. O(n) time, O(h) space.
struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn gain(n: &Option<Box<Node>>, best: &mut i64) -> i64 {
    match n {
        None => 0,
        Some(node) => {
            let l = gain(&node.left, best).max(0);
            let r = gain(&node.right, best).max(0);
            *best = (*best).max(node.val + l + r);
            node.val + l.max(r)
        }
    }
}

fn leaf(v: i64) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn main() {
    let root = Some(Box::new(Node {
        val: -10,
        left: leaf(9),
        right: Some(Box::new(Node { val: 20, left: leaf(15), right: leaf(7) })),
    }));
    let mut best = i64::MIN;
    gain(&root, &mut best);
    println!("{}", best); // 42
}
