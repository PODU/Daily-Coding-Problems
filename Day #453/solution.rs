// Day 453: Two nodes in a BST summing to K.
// Inorder -> sorted vec, then two-pointer. Time O(n), Space O(n).

struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn inorder(n: &Option<Box<Node>>, out: &mut Vec<i32>) {
    if let Some(node) = n {
        inorder(&node.left, out);
        out.push(node.val);
        inorder(&node.right, out);
    }
}

fn two_sum(root: &Option<Box<Node>>, k: i32) -> Option<(i32, i32)> {
    let mut a = Vec::new();
    inorder(root, &mut a);
    let (mut i, mut j) = (0usize, a.len().saturating_sub(1));
    while i < j {
        let s = a[i] + a[j];
        if s == k {
            return Some((a[i], a[j]));
        } else if s < k {
            i += 1;
        } else {
            j -= 1;
        }
    }
    None
}

fn leaf(v: i32) -> Option<Box<Node>> {
    Some(Box::new(Node { val: v, left: None, right: None }))
}

fn main() {
    let root = Some(Box::new(Node {
        val: 10,
        left: leaf(5),
        right: Some(Box::new(Node { val: 15, left: leaf(11), right: leaf(15) })),
    }));
    if let Some((x, y)) = two_sum(&root, 20) {
        println!("{} and {}", x, y); // 5 and 15
    }
}
