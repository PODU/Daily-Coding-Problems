// Day 1180: Swap every two adjacent nodes in a singly linked list.
// Recursive rewiring of Box<Node> ownership. Time O(N), Space O(N) recursion.

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(mut a) => match a.next.take() {
            Some(mut b) => {
                a.next = swap_pairs(b.next.take());
                b.next = Some(a);
                Some(b)
            }
            None => Some(a),
        },
        None => None,
    }
}

fn build(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;
    for &x in vals.iter().rev() {
        head = Some(Box::new(ListNode { val: x, next: head }));
    }
    head
}

fn to_str(mut h: &Option<Box<ListNode>>) -> String {
    let mut parts = Vec::new();
    while let Some(node) = h {
        parts.push(node.val.to_string());
        h = &node.next;
    }
    parts.join(" -> ")
}

fn main() {
    let list = build(&[1, 2, 3, 4]);
    println!("{}", to_str(&swap_pairs(list))); // 2 -> 1 -> 4 -> 3
}
