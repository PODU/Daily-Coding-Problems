// Add two numbers stored as reversed-digit linked lists via elementary addition
// with carry, walking both lists. Time O(max(m,n)), Space O(max(m,n)).

type Link = Option<Box<ListNode>>;

struct ListNode {
    val: i32,
    next: Link,
}

fn build(digits: &[i32]) -> Link {
    let mut head: Link = None;
    for &x in digits.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn add_lists(mut a: &Link, mut b: &Link) -> Link {
    let mut dummy = Box::new(ListNode { val: 0, next: None });
    let mut tail = &mut dummy;
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
        tail.next = Some(Box::new(ListNode { val: s % 10, next: None }));
        tail = tail.next.as_mut().unwrap();
    }
    dummy.next
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
    let a = build(&[9, 9]);
    let b = build(&[5, 2]);
    println!("{}", to_str(&add_lists(&a, &b)));
}
