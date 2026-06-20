// Zigzag list: single pass, even index wants lst[i]<=lst[i+1], odd wants lst[i]>=lst[i+1]; swap if violated.
// O(n) time, O(1) extra space.

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn build(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &v in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: v, next: head }));
    }
    head
}

fn zigzag(head: &mut Option<Box<ListNode>>) {
    let mut cur = head.as_deref_mut();
    let mut i = 0;
    while let Some(node) = cur {
        if let Some(next) = node.next.as_mut() {
            let need_swap = if i % 2 == 0 {
                node.val > next.val
            } else {
                node.val < next.val
            };
            if need_swap {
                std::mem::swap(&mut node.val, &mut next.val);
            }
        } else {
            break;
        }
        cur = node.next.as_deref_mut();
        i += 1;
    }
}

fn main() {
    let mut head = build(&[1, 2, 3, 4, 5]);
    zigzag(&mut head);

    let mut parts: Vec<String> = Vec::new();
    let mut cur = head.as_deref();
    while let Some(node) = cur {
        parts.push(node.val.to_string());
        cur = node.next.as_deref();
    }
    println!("{}", parts.join(" -> "));
}
