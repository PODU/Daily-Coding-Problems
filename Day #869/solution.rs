// Day 869: Is a linked list a palindrome (works for singly linked; doubly trivial).
// Approach: collect values then two-pointer compare (Rust-idiomatic; O(1) extra beyond values).
// Time: O(n), Space: O(n).
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

fn main() {
    println!("{}", title(is_palindrome(&build(&[1, 4, 3, 4, 1])))); // True
    println!("{}", title(is_palindrome(&build(&[1, 4]))));          // False
}

fn title(b: bool) -> &'static str {
    if b { "True" } else { "False" }
}
