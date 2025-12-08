// Day 715: Linked-list palindrome. Collect values then two-pointer compare
// (handles both singly and doubly linked cases). Time O(n), space O(n).
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

fn is_palindrome(head: &Option<Box<Node>>) -> bool {
    let mut vals = Vec::new();
    let mut cur = head;
    while let Some(n) = cur {
        vals.push(n.val);
        cur = &n.next;
    }
    let (mut i, mut j) = (0usize, vals.len());
    while i + 1 < j {
        j -= 1;
        if vals[i] != vals[j] {
            return false;
        }
        i += 1;
    }
    true
}

fn b2s(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}

fn main() {
    println!("{}", b2s(is_palindrome(&build(&[1, 4, 3, 4, 1]))));
    println!("{}", b2s(is_palindrome(&build(&[1, 4]))));
}
