// Iterative in-place reversal of a singly linked list using owned Box pointers.
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

fn main() {
    let mut head: Option<Box<Node>> = None;
    for i in (1..=5).rev() {
        head = Some(Box::new(Node { val: i, next: head }));
    }
    head = reverse(head);
    let mut out: Vec<String> = Vec::new();
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        out.push(node.val.to_string());
        cur = node.next.as_ref();
    }
    println!("{}", out.join(" "));
}
