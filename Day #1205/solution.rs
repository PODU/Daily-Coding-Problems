// Day 1205: Add two numbers stored as reversed-digit linked lists.
// Traverse both lists with a running carry. Time O(max(m,n)), Space O(max(m,n)).
type Link = Option<Box<Node>>;

struct Node {
    val: i32,
    next: Link,
}

fn build(ds: &[i32]) -> Link {
    let mut head: Link = None;
    for &d in ds.iter().rev() {
        head = Some(Box::new(Node { val: d, next: head }));
    }
    head
}

fn add_lists(mut a: &Link, mut b: &Link) -> Link {
    let mut head: Link = None;
    let mut tail = &mut head;
    let mut carry = 0;
    while a.is_some() || b.is_some() || carry != 0 {
        let mut s = carry;
        if let Some(n) = a { s += n.val; a = &n.next; }
        if let Some(n) = b { s += n.val; b = &n.next; }
        carry = s / 10;
        *tail = Some(Box::new(Node { val: s % 10, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn main() {
    let a = build(&[9, 9]);
    let b = build(&[5, 2]);
    let s = add_lists(&a, &b);
    let mut out = Vec::new();
    let mut cur = &s;
    while let Some(n) = cur {
        out.push(n.val.to_string());
        cur = &n.next;
    }
    println!("{}", out.join(" -> ")); // 4 -> 2 -> 1
}
