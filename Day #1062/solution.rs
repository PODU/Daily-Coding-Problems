// Day 1062: Swap every two adjacent nodes in a singly linked list.
// Approach: recursive pointer manipulation via Option<Box> (Rust ownership-friendly). Time O(n), Space O(n) recursion stack.

struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

fn swap_pairs(mut head: Option<Box<Node>>) -> Option<Box<Node>> {
    // If there are at least two nodes, swap the first pair then recurse on rest.
    match head.take() {
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

fn print_list(mut head: &Option<Box<Node>>) {
    let mut parts: Vec<String> = Vec::new();
    while let Some(node) = head {
        parts.push(node.val.to_string());
        head = &node.next;
    }
    println!("{}", parts.join(" -> "));
}

fn main() {
    let head = Some(Box::new(Node {
        val: 1,
        next: Some(Box::new(Node {
            val: 2,
            next: Some(Box::new(Node {
                val: 3,
                next: Some(Box::new(Node { val: 4, next: None })),
            })),
        })),
    }));
    let head = swap_pairs(head);
    print_list(&head); // 2 -> 1 -> 4 -> 3
}
