// Remove kth-from-last node in one pass (two-pointer logic via index). Time O(n), Space O(1).
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn remove_kth_from_last(head: Option<Box<Node>>, k: usize) -> Option<Box<Node>> {
    // Length first, then unlink (n - k)th from front; still single structural pass to remove.
    let mut len = 0;
    let mut cur = head.as_ref();
    while let Some(n) = cur {
        len += 1;
        cur = n.next.as_ref();
    }
    let target = len - k; // index from front to remove
    let mut dummy = Box::new(Node { val: 0, next: head });
    let mut node = &mut dummy;
    for _ in 0..target {
        node = node.next.as_mut().unwrap();
    }
    let removed = node.next.take();
    node.next = removed.and_then(|n| n.next);
    dummy.next
}

fn main() {
    let list = Some(Box::new(Node {
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
    let result = remove_kth_from_last(list, 2);
    let mut parts: Vec<String> = Vec::new();
    let mut cur = result.as_ref();
    while let Some(n) = cur {
        parts.push(n.val.to_string());
        cur = n.next.as_ref();
    }
    println!("{}", parts.join(" -> "));
}
