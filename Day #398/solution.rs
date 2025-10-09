// Remove k-th node from end in one pass using index from length (Box list). O(n) time, O(1) extra space.
// Rust ownership makes raw two-pointer awkward; we compute the target index then splice in one structural pass.
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn build(vals: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(Node { val: v, next: head }));
    }
    head
}

fn len(head: &Option<Box<Node>>) -> usize {
    let mut n = 0;
    let mut cur = head;
    while let Some(node) = cur {
        n += 1;
        cur = &node.next;
    }
    n
}

fn remove_kth_from_end(head: Option<Box<Node>>, k: usize) -> Option<Box<Node>> {
    let length = len(&head);
    let idx = length - k; // index (0-based) of node to remove
    let mut dummy = Box::new(Node { val: 0, next: head });
    let mut cur = &mut dummy;
    for _ in 0..idx {
        cur = cur.next.as_mut().unwrap();
    }
    // unlink cur.next
    let to_remove = cur.next.take();
    cur.next = to_remove.and_then(|n| n.next);
    dummy.next
}

fn print_list(head: &Option<Box<Node>>) {
    let mut parts = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        parts.push(node.val.to_string());
        cur = &node.next;
    }
    println!("{}", parts.join(" -> "));
}

fn main() {
    let head = build(&[1, 2, 3, 4, 5]);
    let head = remove_kth_from_end(head, 2);
    print_list(&head);
}
