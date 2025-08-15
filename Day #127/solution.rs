// Day 127: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. O(max(m,n)) time, O(1) extra space.
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
        let mut s = carry;
        if let Some(n) = a {
            s += n.val;
            a = &n.next;
        }
        if let Some(n) = b {
            s += n.val;
            b = &n.next;
        }
        carry = s / 10;
        *tail = Some(Box::new(Node { val: s % 10, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn build(d: &[i32]) -> Link {
    let mut head: Link = None;
    let mut tail: &mut Link = &mut head;
    for &v in d {
        *tail = Some(Box::new(Node { val: v, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn to_str(mut n: &Link) -> String {
    let mut parts = Vec::new();
    while let Some(node) = n {
        parts.push(node.val.to_string());
        n = &node.next;
    }
    parts.join(" -> ")
}

fn main() {
    let a = build(&[9, 9]); // 99
    let b = build(&[5, 2]); // 25
    println!("{}", to_str(&add_lists(&a, &b))); // 4 -> 2 -> 1
}
