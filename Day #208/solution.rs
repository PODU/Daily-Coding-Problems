// Day 208: Partition a linked list around pivot k (stable).
// Build two lists (< k and >= k) in original order, then concatenate. Time: O(n), Space: O(n).
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn partition(head: Option<Box<Node>>, k: i32) -> Option<Box<Node>> {
    let mut less: Vec<i32> = Vec::new();
    let mut ge: Vec<i32> = Vec::new();
    let mut cur = head;
    while let Some(node) = cur {
        if node.val < k {
            less.push(node.val);
        } else {
            ge.push(node.val);
        }
        cur = node.next;
    }
    less.extend(ge);
    build(&less)
}

fn build(vals: &[i32]) -> Option<Box<Node>> {
    let mut head: Option<Box<Node>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(Node { val: x, next: head }));
    }
    head
}

fn to_str(mut h: &Option<Box<Node>>) -> String {
    let mut parts = Vec::new();
    while let Some(node) = h {
        parts.push(node.val.to_string());
        h = &node.next;
    }
    parts.join(" -> ")
}

fn main() {
    let list = build(&[5, 1, 8, 0, 3]);
    println!("{}", to_str(&partition(list, 3))); // 1 -> 0 -> 5 -> 8 -> 3
}
