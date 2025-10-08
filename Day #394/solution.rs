// Root-to-leaf path sum via DFS subtracting node values; leaf checks remainder==0. O(n) time, O(h) space.
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn has_path_sum(root: &Option<Box<Node>>, k: i32) -> bool {
    match root {
        None => false,
        Some(n) => {
            if n.left.is_none() && n.right.is_none() {
                return k - n.val == 0;
            }
            has_path_sum(&n.left, k - n.val) || has_path_sum(&n.right, k - n.val)
        }
    }
}

fn leaf(v: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn main() {
    let root = Some(Box::new(Node {
        val: 8,
        left: Some(Box::new(Node { val: 4, left: leaf(2), right: leaf(6) })),
        right: Some(Box::new(Node { val: 13, left: None, right: leaf(19) })),
    }));
    println!("{}", has_path_sum(&root, 18));
}
