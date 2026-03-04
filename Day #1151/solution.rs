// Day 1151: Palindrome linked list.
// Collect values then two-pointer compare (works for singly & doubly). O(n) time, O(n) space.
struct Node { val: i32, next: Option<Box<Node>> }

fn build(vals: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn is_palindrome(head: &Option<Box<Node>>) -> bool {
    let mut vals = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        vals.push(node.val);
        cur = &node.next;
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
    println!("{}", if is_palindrome(&build(&[1, 4, 3, 4, 1])) { "True" } else { "False" }); // True
    println!("{}", if is_palindrome(&build(&[1, 4])) { "True" } else { "False" });          // False
}
