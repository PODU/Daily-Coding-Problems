// Reverse a singly linked list in-place by re-pointing each node's next pointer.
// Time: O(n), Space: O(1). Uses Option<Box<Node>> ownership transfer.
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

fn to_str(mut h: &Option<Box<Node>>) -> String {
    let mut parts = Vec::new();
    while let Some(node) = h {
        parts.push(node.val.to_string());
        h = &node.next;
    }
    parts.join(" -> ")
}

fn main() {
    let head = Some(Box::new(Node { val: 1, next:
        Some(Box::new(Node { val: 2, next:
        Some(Box::new(Node { val: 3, next:
        Some(Box::new(Node { val: 4, next:
        Some(Box::new(Node { val: 5, next: None })) })) })) })) }));
    let head = reverse(head);
    println!("{}", to_str(&head)); // 5 -> 4 -> 3 -> 2 -> 1
}
