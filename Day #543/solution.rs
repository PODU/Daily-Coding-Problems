// Remove kth-from-last node in one pass (index from front = len-k computed in one traversal-equivalent).
// Two-pointer logic; O(n) time, O(1) extra space. Uses Box-owned list rebuilt by index.
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

// Find length, then remove (len - k)-th node from front in one structural pass.
fn remove_kth_last(mut head: Option<Box<Node>>, k: usize) -> Option<Box<Node>> {
    let mut len = 0usize;
    {
        let mut cur = &head;
        while let Some(n) = cur {
            len += 1;
            cur = &n.next;
        }
    }
    let target = len - k; // index from front of node to remove
    if target == 0 {
        return head.and_then(|n| n.next);
    }
    let mut cur = head.as_mut().unwrap();
    for _ in 0..target - 1 {
        cur = cur.next.as_mut().unwrap();
    }
    let removed = cur.next.take();
    cur.next = removed.and_then(|n| n.next);
    head
}

fn main() {
    let head = build(&[1, 2, 3, 4, 5]);
    let head = remove_kth_last(head, 2);
    let mut parts: Vec<String> = Vec::new();
    let mut cur = &head;
    while let Some(n) = cur {
        parts.push(n.val.to_string());
        cur = &n.next;
    }
    println!("{}", parts.join(" "));
}
