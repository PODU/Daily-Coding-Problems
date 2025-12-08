// Day 714: Swap every two adjacent nodes in a singly linked list. Rebuild via an
// owned Option<Box> chain, swapping pairs. Time O(n), space O(1) extra.
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn swap_pairs(head: Option<Box<Node>>) -> Option<Box<Node>> {
    match head {
        Some(mut a) => match a.next.take() {
            Some(mut b) => {
                let rest = swap_pairs(b.next.take());
                a.next = rest;
                b.next = Some(a);
                Some(b)
            }
            None => Some(a),
        },
        None => None,
    }
}

fn main() {
    // Build 1 -> 2 -> 3 -> 4
    let mut head: Option<Box<Node>> = None;
    for v in [4, 3, 2, 1] {
        head = Some(Box::new(Node { val: v, next: head }));
    }
    head = swap_pairs(head);
    let mut parts = Vec::new();
    let mut cur = &head;
    while let Some(n) = cur {
        parts.push(n.val.to_string());
        cur = &n.next;
    }
    println!("{}", parts.join(" -> "));
}
