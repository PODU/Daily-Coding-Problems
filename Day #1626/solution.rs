// Day 1626: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(m,n)), Space O(max(m,n)).
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn add_lists(mut a: Option<Box<Node>>, mut b: Option<Box<Node>>) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    let mut tail = &mut head;
    let mut carry = 0;
    while a.is_some() || b.is_some() || carry != 0 {
        let mut sum = carry;
        if let Some(n) = a {
            sum += n.val;
            a = n.next;
        }
        if let Some(n) = b {
            sum += n.val;
            b = n.next;
        }
        carry = sum / 10;
        *tail = Some(Box::new(Node { val: sum % 10, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn build(vals: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    let mut tail = &mut head;
    for &x in vals {
        *tail = Some(Box::new(Node { val: x, next: None }));
        tail = &mut tail.as_mut().unwrap().next;
    }
    head
}

fn main() {
    let r = add_lists(build(&[9, 9]), build(&[5, 2]));
    let mut parts = Vec::new();
    let mut cur = &r;
    while let Some(n) = cur {
        parts.push(n.val.to_string());
        cur = &n.next;
    }
    println!("{}", parts.join(" -> "));
}
