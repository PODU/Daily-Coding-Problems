// Reverse a singly linked list in-place by re-pointing each next pointer.
// Time: O(n), Space: O(1).

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn reverse(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn to_str(mut head: &Option<Box<Node>>) -> String {
    let mut parts: Vec<String> = Vec::new();
    while let Some(node) = head {
        parts.push(node.val.to_string());
        head = &node.next;
    }
    parts.join(" ")
}

fn main() {
    let head = Some(Box::new(Node {
        val: 1,
        next: Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node {
                val: 3,
                next: Some(Box::new(Node {
                    val: 4,
                    next: Some(Box::new(Node { val: 5, next: None })),
                })),
            })),
        })),
    }));
    println!("{}", to_str(&head));
    let head = reverse(head);
    println!("{}", to_str(&head));
}
