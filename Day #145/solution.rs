// Swap every two adjacent nodes in a singly linked list via iterative rebuild. O(n) time, O(1) extra space.

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn swap_pairs(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut new_head: Option<Box<Node>> = None;
    let mut tail: *mut Option<Box<Node>> = &mut new_head;
    while let Some(mut a) = head {
        if let Some(mut b) = a.next.take() {
            head = b.next.take();
            // append b then a
            a.next = None;
            b.next = Some(a);
            unsafe {
                *tail = Some(b);
                let bref = (*tail).as_mut().unwrap();
                tail = &mut bref.next.as_mut().unwrap().next;
            }
        } else {
            // odd trailing node
            head = a.next.take();
            unsafe {
                *tail = Some(a);
                tail = &mut (*tail).as_mut().unwrap().next;
            }
        }
    }
    new_head
}

fn main() {
    // build 1 -> 2 -> 3 -> 4
    let mut head: Option<Box<Node>> = None;
    for v in [4, 3, 2, 1] {
        head = Some(Box::new(Node { val: v, next: head }));
    }
    head = swap_pairs(head);
    let mut parts: Vec<String> = Vec::new();
    let mut cur = head.as_ref();
    while let Some(n) = cur {
        parts.push(n.val.to_string());
        cur = n.next.as_ref();
    }
    println!("{}", parts.join(" -> ")); // 2 -> 1 -> 4 -> 3
}
