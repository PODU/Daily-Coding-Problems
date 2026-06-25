// In-order traversal -> sorted array, then two-pointer for pair summing to K. Time O(n), Space O(n).
struct Node {
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
}

fn inorder(node: &Option<Box<Node>>, acc: &mut Vec<i32>) {
    if let Some(n) = node {
        inorder(&n.left, acc);
        acc.push(n.val);
        inorder(&n.right, acc);
    }
}

fn main() {
    let mut root = Node::new(10);
    root.left = Some(Node::new(5));
    let mut right = Node::new(15);
    right.left = Some(Node::new(11));
    right.right = Some(Node::new(15));
    root.right = Some(right);

    let some_root = Some(root);
    let k = 20;
    let mut a = Vec::new();
    inorder(&some_root, &mut a);
    let (mut i, mut j) = (0i32, a.len() as i32 - 1);
    while i < j {
        let s = a[i as usize] + a[j as usize];
        if s == k {
            println!("{} and {}", a[i as usize], a[j as usize]);
            return;
        } else if s < k {
            i += 1;
        } else {
            j -= 1;
        }
    }
    println!("No pair found");
}
