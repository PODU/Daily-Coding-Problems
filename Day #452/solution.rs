// Day 452: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(n,m)), Space O(max(n,m)).

type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    next: Link,
}

fn add_lists(mut a: &Link, mut b: &Link) -> Link {
    let mut head: Link = None;
    let mut tail: &mut Link = &mut head;
    let mut carry = 0;
    while a.is_some() || b.is_some() || carry != 0 {
        let mut sum = carry;
        if let Some(n) = a {
            sum += n.val;
            a = &n.next;
        }
        if let Some(n) = b {
            sum += n.val;
            b = &n.next;
        }
        carry = sum / 10;
        *tail = Some(Box::new(Node { val: sum % 10, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn build(xs: &[i32]) -> Link {
    let mut head: Link = None;
    let mut tail = &mut head;
    for &x in xs {
        *tail = Some(Box::new(Node { val: x, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn show(mut n: &Link) {
    let mut parts = Vec::new();
    while let Some(node) = n {
        parts.push(node.val.to_string());
        n = &node.next;
    }
    println!("{}", parts.join(" -> "));
}

fn main() {
    let r = add_lists(&build(&[9, 9]), &build(&[5, 2]));
    show(&r); // 4 -> 2 -> 1
}
