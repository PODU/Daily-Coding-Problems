// Reverse singly linked list in-place: iterative 3-pointer using Option<Box>. O(n) time, O(1) space.

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn reverse_list(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut prev: Option<Box<Node>> = None;
    while let Some(mut cur) = head {
        head = cur.next.take();
        cur.next = prev;
        prev = Some(cur);
    }
    prev
}

fn main() {
    // Build 1 -> 2 -> 3 -> 4 -> 5
    let mut head: Option<Box<Node>> = None;
    for v in (1..=5).rev() {
        head = Some(Box::new(Node { val: v, next: head }));
    }

    head = reverse_list(head);

    let mut parts: Vec<String> = Vec::new();
    let mut p = &head;
    while let Some(node) = p {
        parts.push(node.val.to_string());
        p = &node.next;
    }
    println!("{}", parts.join(" -> "));
}
