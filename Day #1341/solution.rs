// Check height-balanced binary tree via bottom-up DFS; -1 sentinel marks unbalanced.
// Time O(n), Space O(h).

type Link = Option<Box<Node>>;

struct Node {
    left: Link,
    right: Link,
}

fn leaf() -> Link {
    Some(Box::new(Node { left: None, right: None }))
}

fn height(root: &Link) -> i32 {
    match root {
        None => 0,
        Some(n) => {
            let l = height(&n.left);
            if l == -1 {
                return -1;
            }
            let r = height(&n.right);
            if r == -1 {
                return -1;
            }
            if (l - r).abs() > 1 {
                return -1;
            }
            l.max(r) + 1
        }
    }
}

fn is_balanced(root: &Link) -> bool {
    height(root) != -1
}

fn main() {
    // Balanced tree [1,2,3,4,5,null,6]
    let a = Some(Box::new(Node {
        left: Some(Box::new(Node { left: leaf(), right: leaf() })),
        right: Some(Box::new(Node { left: None, right: leaf() })),
    }));
    println!("Balanced: {}", is_balanced(&a));

    // Skewed tree 1 -> 2 -> 3
    let b = Some(Box::new(Node {
        left: Some(Box::new(Node { left: leaf(), right: None })),
        right: None,
    }));
    println!("Balanced: {}", is_balanced(&b));
}
