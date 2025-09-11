// Day 256: Rearrange linked list values into zigzag low->high->low form.
// One pass over node values: at even i ensure a[i]<=a[i+1], at odd i ensure a[i]>=a[i+1], swap on violation.
// Time: O(n), Space: O(1). Uses a Vec to model the list for safe in-place ownership.

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn wiggle(head: &mut Option<Box<Node>>) {
    let mut less = true; // even index: want current <= next
    let mut cur = head.as_deref_mut();
    while let Some(node) = cur {
        if let Some(nxt) = node.next.as_deref_mut() {
            if less {
                if node.val > nxt.val {
                    std::mem::swap(&mut node.val, &mut nxt.val);
                }
            } else if node.val < nxt.val {
                std::mem::swap(&mut node.val, &mut nxt.val);
            }
            less = !less;
            cur = node.next.as_deref_mut();
        } else {
            break;
        }
    }
}

fn list_to_string(head: &Option<Box<Node>>) -> String {
    let mut parts: Vec<String> = Vec::new();
    let mut cur = head.as_deref();
    while let Some(node) = cur {
        parts.push(node.val.to_string());
        cur = node.next.as_deref();
    }
    parts.join(" -> ")
}

fn main() {
    let vals = [1, 2, 3, 4, 5];
    let mut head: Option<Box<Node>> = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(Node { val: v, next: head }));
    }
    wiggle(&mut head);
    println!("{}", list_to_string(&head)); // 1 -> 3 -> 2 -> 5 -> 4
}
